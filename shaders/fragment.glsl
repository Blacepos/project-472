#version 330

#define MAX_MARCHES 300
#define MAX_DIST 100.0
#define MIN_DIST 0.005
#define SHADOW_DELTA 0.08
#define SOFT_SHADOWS 0

uniform ivec2 iResolution;
uniform float iTime;
uniform vec3 iCameraPos;
float cameraZoom = 0.7;

//
// Lights
//

#define LIGHT_BUFFER_SIZE 256
#define LIGHT_SIZE 12
uniform float iLights[LIGHT_BUFFER_SIZE];
uniform int iNumLights;
struct Light {
	vec3 position;
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
};
Light read_light(int i) {
	return Light(
		vec3(
			iLights[i],
			iLights[i+1],
			iLights[i+2]
		),
		vec3(
			iLights[i+3],
			iLights[i+4],
			iLights[i+5]
		),
		vec3(
			iLights[i+6],
			iLights[i+7],
			iLights[i+8]
		),
		vec3(
			iLights[i+9],
			iLights[i+10],
			iLights[i+11]
		)
	);
}

//
// Scene
//
#define SCENE_BUFFER_SIZE 384
uniform float iScene[SCENE_BUFFER_SIZE];
uniform int iNumObjs;

//
// Helpers
//
int id_to_int(float id) {
	return int(id+0.1);
} 

//
// Primitives
//
// Sphere
#define SPHERE_ID 0
#define SPHERE_SIZE 5
struct Sphere {
	vec3 center;
	float radius;
};
/// Read a sphere from the scene buffer. DOES include the ID
Sphere read_sphere(int i) {
	return Sphere(
		vec3(
			iScene[i+1],
			iScene[i+2],
			iScene[i+3]
		),
		iScene[i+4]
	);
}
float dist_sphere(vec3 point, Sphere s) {
	return length(point-s.center)-s.radius;
}

// Rectangular prism
#define RECT_PRISM_ID 1
#define RECT_PRISM_SIZE 7
struct RectPrism {
	vec3 center;
	vec3 extents;
};
RectPrism read_rect_prism(int i) {
	return RectPrism(
		vec3(
			iScene[i+1],
			iScene[i+2],
			iScene[i+3]
		),
		vec3(
			iScene[i+4],
			iScene[i+5],
			iScene[i+6]
		)
	);
}
float dist_rect_prism(vec3 point, RectPrism r) {
	point -= r.center;
	return length(max(abs(point)-r.extents, 0.0));
}

// Cylinder
#define CYLINDER_ID 2
#define CYLINDER_SIZE 8
struct Cylinder {
	vec3 point1;
	vec3 point2;
	float radius;
};
Cylinder read_cylinder(int i) {
	return Cylinder(
		vec3(
			iScene[i+1],
			iScene[i+2],
			iScene[i+3]
		),
		vec3(
			iScene[i+4],
			iScene[i+5],
			iScene[i+6]
		),
		iScene[i+7]
	);
}
float dist_cylinder(vec3 point, Cylinder c) {
	vec3  ba = c.point2 - c.point1;
	vec3  pa = point - c.point1;
	float baba = dot(ba,ba);
	float paba = dot(pa,ba);
	float x = length(pa*baba-ba*paba) - c.radius*baba;
	float y = abs(paba-baba*0.5)-baba*0.5;
	float x2 = x*x;
	float y2 = y*y*baba;
	float d = (max(x,y)<0.0)?-min(x2,y2):(((x>0.0)?x2:0.0)+((y>0.0)?y2:0.0));
	return sign(d)*sqrt(abs(d))/baba;
}

// Capsule
#define CAPSULE_ID 3
#define CAPSULE_SIZE 8
struct Capsule {
	vec3 point1;
	vec3 point2;
	float radius;
};
Capsule read_capsule(int i) {
	return Capsule(
		vec3(
			iScene[i+1],
			iScene[i+2],
			iScene[i+3]
		),
		vec3(
			iScene[i+4],
			iScene[i+5],
			iScene[i+6]
		),
		iScene[i+7]
	);
}
float dist_capsule(vec3 point, Capsule c) {
	vec3 pa = point - c.point1, ba = c.point2 - c.point1;
	float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
	return length( pa - ba*h ) - c.radius;
}


//
// Unary Operations
//
#define ROUNDING_ID 100
#define ROUNDING_SIZE 2
float op_rounding(float dist, float radius) {
	return dist - radius;
}

//
// Binary Operations
//
#define INTERSECTION_ID 200
#define INTERSECTION_SIZE 1
float op_intersection(float dist1, float dist2) {
	return max(dist1, dist2);
}

#define SMOOTH_UNION_ID 201
#define SMOOTH_UNION_SIZE 2
float op_smooth_union(float dist1, float dist2, float k) {
    float h = max(k - abs(dist1 - dist2), 0.0);
    return min(dist1, dist2) - h*h*0.25/k;
}

#define SMOOTH_INTERSECTION_ID 202
#define SMOOTH_INTERSECTION_SIZE 2
float op_smooth_intersection(float dist1, float dist2, float k) {
	// float h = clamp(0.5 - 0.5*(dist2 - dist1)/k, 0.0, 1.0);
    // return mix(dist2, dist1, h) + k*h*(1.0 - h);
	return -op_smooth_union(-dist1, -dist2, k);
}

//
// Material
//
#define MATERIAL_SIZE 13
struct Material {
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	vec3 reflectivity;
	float shininess;
};
/// Read in a material.
/// Note: there is only 1 material and no ID, so indexing starts at 0, unlike the operations and primitives.
Material read_material(int i) {
	return Material(
		vec3(iScene[i+0], iScene[i+1], iScene[i+2]),
		vec3(iScene[i+3], iScene[i+4], iScene[i+5]),
		vec3(iScene[i+6], iScene[i+7], iScene[i+8]),
		vec3(iScene[i+9], iScene[i+10], iScene[i+11]),
		iScene[i+12]
	);
}

//
// Compute scene distance
//

/// Retrieves a single primitive distance for unary operations. Does NOT accept nested operations (yet)
/// Updates the index based on the shape it processed
float unary_operand(vec3 point, inout int i) {
	int obj_id = id_to_int(iScene[i]);
	switch (obj_id) {
		case SPHERE_ID:
			Sphere s = read_sphere(i);
			i += SPHERE_SIZE;
			return dist_sphere(point, s);
		case RECT_PRISM_ID:
			RectPrism r = read_rect_prism(i);
			i += RECT_PRISM_SIZE;
			return dist_rect_prism(point, r);
		case CYLINDER_ID:
			Cylinder c = read_cylinder(i);
			i += CYLINDER_SIZE;
			return dist_cylinder(point, c);
		case CAPSULE_ID:
			Capsule c2 = read_capsule(i);
			i += CAPSULE_SIZE;
			return dist_capsule(point, c2);
		default:
			return MAX_DIST;
	}
}
/// Retrieves two primitive distances for binary operations. Does NOT accept nested operations (yet)
/// Updates the index based on the shapes it processed
vec2 binary_operands(vec3 point, inout int i) {
	// first object
	float dist1 = unary_operand(point, i);
	// second object
	float dist2 = unary_operand(point, i);
	return vec2(dist1, dist2);
}
/// The main scene distance function
float scene_distance(vec3 point, out Material material) {
	float dist = MAX_DIST;
	float min_dist = MAX_DIST;

	int i = 0;
	for (int obj = 0; obj < iNumObjs; ++obj) {
		// get object distance
		int obj_id = id_to_int(iScene[i]);
		switch (obj_id) {
			case SPHERE_ID:
				float s_dist = unary_operand(point, i);
				dist = min(dist, s_dist);
				break;
			case RECT_PRISM_ID:
				float r_dist = unary_operand(point, i);
				dist = min(dist, r_dist);
				break;
			case CYLINDER_ID:
				float c_dist = unary_operand(point, i);
				dist = min(dist, c_dist);
				break;
			case CAPSULE_ID:
				float c2_dist = unary_operand(point, i);
				dist = min(dist, c2_dist);
				break;
			case ROUNDING_ID:
				float radius = iScene[i+1];
				i += ROUNDING_SIZE;
				float prim_dist = unary_operand(point, i);
				dist = min(dist, op_rounding(prim_dist, radius));
				break;
			case INTERSECTION_ID:
				i += INTERSECTION_SIZE;
				vec2 prim_dists = binary_operands(point, i);
				dist = min(dist, op_intersection(prim_dists.x, prim_dists.y));
				break;
			case SMOOTH_UNION_ID:
				float k = iScene[i+1];
				i += SMOOTH_UNION_SIZE;
				vec2 prim_dists2 = binary_operands(point, i);
				dist = min(dist, op_smooth_union(prim_dists2.x, prim_dists2.y, k));
				break;
			case SMOOTH_INTERSECTION_ID:
				float k2 = iScene[i+1];
				i += SMOOTH_INTERSECTION_SIZE;
				vec2 prim_dists3 = binary_operands(point, i);
				dist = min(dist, op_smooth_intersection(prim_dists3.x, prim_dists3.y, k2));
				break;
		}
		if (dist < min_dist) {
			material = read_material(i);
			min_dist = dist;
		}
		i += MATERIAL_SIZE;
	}
	return dist;
}

float ray_march(vec3 origin, vec3 direction, out Material material, out bool hit, inout float min_scene_dist) {
	float curr_dist = 0.0; // current distance from the origin of the ray
	int i;
	float scene_dist;
	hit = false;
	min_scene_dist = MAX_DIST;

	for (i = 0; i < MAX_MARCHES; i++) {
		vec3 curr_pos = origin + direction*curr_dist; // start at origin and march a certain distance in the direction of the ray
		scene_dist = scene_distance(curr_pos, material);
		curr_dist += scene_dist; // march the ray the same distance it is from the scene

		min_scene_dist = min(min_scene_dist, SHADOW_DELTA*scene_dist/curr_dist);

		if (curr_dist > MAX_DIST)
			break;
		if (scene_dist < MIN_DIST) {
			hit = true;
			break;
		}
	}

	return curr_dist; // return where the ray finally stopped in the scene
}

vec3 scene_lighting(vec3 point, vec3 normal, Material material) {
	vec3 hallcolor = vec3(0.);
	int l = 0;
	for (int i=0; i < iNumLights; ++i) {
		Light light = read_light(l);
		l += LIGHT_SIZE;

		vec3 light_direction = normalize(light.position - point);
		float light_dist = length(light.position - point);

		if (dot(normal, light_direction) < 0) continue;

		// always include ambient
		hallcolor += light.ambient * material.ambient;

		// shadow feeler
		Material _m;
		bool shadow_hit = false;
		float min_scene_dist;
		float feeler_dist = ray_march(point + normal * MIN_DIST * 2., light_direction, _m, shadow_hit, min_scene_dist);
		
		vec3 spec_and_diff = vec3(0.);
		// rest of blinn-phong
		spec_and_diff += light.diffuse * material.diffuse * dot(normal, light_direction);
		
		vec3 view_direction = normalize(iCameraPos - point);
		vec3 h = normalize(light_direction + view_direction);
		float sif = pow(dot(normal, h), material.shininess);
		spec_and_diff += light.specular * material.specular * sif;
#if SOFT_SHADOWS
		hallcolor += spec_and_diff * clamp(min_scene_dist * feeler_dist, 0., 1.);
#else
		hallcolor += spec_and_diff * int(!shadow_hit);
#endif
	}
	return hallcolor;
}

vec3 getNormal(vec3 point) // get the 3d "perpendicular" direction.
{
	Material _m;
	float dist = scene_distance(point, _m); // remember, the point is almost exactly MIN_DIST away from the actual surface so we can just get the dist
	vec2 epsilon = vec2(MIN_DIST, 0.0); // epsilon means small value, used for approximating the tangent plane to the surface of the scene

	vec3 normal = dist - vec3(
			scene_distance(point-epsilon.xyy, _m),
			scene_distance(point-epsilon.yxy, _m),
			scene_distance(point-epsilon.yyx, _m));
	return normalize(normal);
}

void main(void)
{
	vec2 uv = vec2(gl_FragCoord.xy-0.5*iResolution)/iResolution.y;

	vec3 fragColor = vec3(1.0);

	vec3 rayOrigin = iCameraPos;
	vec3 rayDirection = normalize(vec3(uv.x, uv.y, cameraZoom));

	Material material;
	bool hit;
	float _msd;
	float dist = ray_march(rayOrigin, rayDirection, material, hit, _msd);

	vec3 point = rayOrigin + rayDirection*dist; // get the point on the surface that we ray marched to
	vec3 normal = getNormal(point);

	if (hit) {
		vec3 diffuseLighting = scene_lighting(point, normal, material);
	
		fragColor = vec3(diffuseLighting); // use the light value from getLight to shade the fragment
	}
	gl_FragColor = vec4(fragColor, 1.0);
}

