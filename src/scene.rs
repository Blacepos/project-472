use std::time::Duration;

use sfml::graphics::Shader;

use crate::modeling::{material::Material, object::Object, binary::Binary, primitive::Primitive, lighting::Light, scene::Scene, unary::Unary};


pub fn scene(shader: &mut Shader, elapsed_time: Duration) {
    let t = elapsed_time.as_secs_f32();
    let mut scene = Scene::new();

    //
    // Materials
    //
    let cyan_matte = Material::default()
        .with_diffuse((0., 0.4, 0.8))
        .with_ambient((0.2, 0.2, 0.2))
        .with_white_specular(0.7)
        .with_shininess(100.);

    let msu_green = Material::default()
        .with_diffuse((0.094, 0.271, 0.231))
        .with_ambient((0.2, 0.2, 0.2))
        .with_white_specular(0.7)
        .with_shininess(20.);

    //
    // Project embedding requirement
    //
    let scale = 1.;
    let depth = 6.+3.*(-t/2.).sin();
    let ox = -2.5;
    let oy = 1.+3.*(t/2.).cos();
    let smoothing = 0.2+0.2*(2.*t).sin();
    let four_1 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+0.*scale,oy+2.*scale,depth),
                point2: (ox+0.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+0.*scale,oy+1.*scale,depth),
                point2: (ox+1.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    let four_2 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+0.*scale,oy+1.*scale,depth),
                point2: (ox+1.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+1.*scale,oy+2.*scale,depth),
                point2: (ox+1.*scale,oy+0.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    scene.add_object(four_1);
    scene.add_object(four_2);

    let seven = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+2.*scale,oy+2.*scale,depth),
                point2: (ox+3.*scale,oy+2.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+3.*scale,oy+2.*scale,depth),
                point2: (ox+2.*scale,oy+0.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    scene.add_object(seven);

    let two_1 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+4.*scale,oy+2.*scale,depth),
                point2: (ox+5.*scale,oy+2.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+5.*scale,oy+2.*scale,depth),
                point2: (ox+5.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    let two_2 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+5.*scale,oy+2.*scale,depth),
                point2: (ox+5.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+5.*scale,oy+1.*scale,depth),
                point2: (ox+4.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    let two_3 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+5.*scale,oy+1.*scale,depth),
                point2: (ox+4.*scale,oy+1.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+4.*scale,oy+1.*scale,depth),
                point2: (ox+4.*scale,oy+0.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    let two_4 = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Capsule {
                point1: (ox+4.*scale,oy+1.*scale,depth),
                point2: (ox+4.*scale,oy+0.*scale,depth),
                radius: 0.1*scale,
            },
            model2: Primitive::Capsule {
                point1: (ox+4.*scale,oy+0.*scale,depth),
                point2: (ox+5.*scale,oy+0.*scale,depth),
                radius: 0.1*scale,
            },
            k: smoothing,
        },
        material: msu_green,
    };
    scene.add_object(two_1);
    scene.add_object(two_2);
    scene.add_object(two_3);
    scene.add_object(two_4);

    //
    // Other Objects
    //      
    let conjoined_sphere_rect = Object {
        model: Binary::SmoothUnion {
            model1: Primitive::Sphere {
                center: (1.*t.sin(), 1.*t.cos()+2., 6.),
                radius: 1.
            },
            model2: Primitive::RectPrism {
                center: (-1.*t.sin(), 2., 6.),
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
        position: (-1., 2.5, 12.0),
        ambient: (0.5, 0.5, 0.3),
        diffuse: (0.5, 0.5, 0.3),
        specular: (0.5, 0.5, 0.3),
    };
    scene.add_light(light2);

    scene.set_camera_pos((0., 1., 0.));

    scene.set_in_shader(shader);
}
