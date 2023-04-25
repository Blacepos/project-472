use sfml::graphics::Shader;

use crate::modeling::{material::Material, object::Object, binary::Binary, primitive::Primitive, lighting::Light, scene::Scene, unary::Unary};


pub fn scene(shader: &mut Shader) {
    let mut scene = Scene::new();

    //
    // Materials
    //
    let cyan_matte = Material::default()
        .with_diffuse((0., 0.4, 0.8))
        .with_ambient((0.2, 0.2, 0.2))
        .with_white_specular(0.8)
        .with_shininess(100.);

    //
    // Objects
    //      
    let conjoined_sphere_rect = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Sphere {
                center: (1., 2., 6.),
                radius: 1.
            },
            model2: Primitive::RectPrism {
                center: (-1., 2., 6.),
                extents: (1., 1., 1.)
            },
            k: 0.6
        },
        material: cyan_matte,
    };
    scene.add_object(conjoined_sphere_rect);

    let rect = Object {
        model: Unary::Rounding {
            model: Primitive::RectPrism {
                center: (0., -0.1, 5.),
                extents: (3., 0.2, 3.),
            },
            radius: 0.1
        },
        material: Material::default()
            .with_diffuse((0.8, 0.0, 0.1))
            .with_white_specular(1.)
            .make_shiny()
    };
    scene.add_object(rect);

    //
    // Scene Lights
    //
    let light1 = Light {
        position: (1.0, 5.0, 0.0),
        ambient: (0.1, 0.1, 0.1),
        diffuse: (1., 1., 1.),
        specular: (1., 1., 1.),
    };
    scene.add_light(light1);

    let light2 = Light {
        position: (-1., 4., 12.0),
        ambient: (0.5, 0.5, 0.3),
        diffuse: (0.5, 0.5, 0.3),
        specular: (0.5, 0.5, 0.3),
    };
    scene.add_light(light2);

    scene.set_camera_pos((0., 1., 0.));

    scene.set_in_shader(shader);
}
