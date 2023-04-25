use super::{material::Material, flatten::Flatten};

#[allow(unused)]
const PRIMITIVE_RANGE: (f32, f32) = (0., 99.);
#[allow(unused)]
const UNARY_RANGE: (f32, f32) = (100., 199.);
#[allow(unused)]
const BINARY_RANGE: (f32, f32) = (200., 299.);

pub struct Object<M> {
    pub model: M,
    pub material: Material
}

impl<M> Object<M>
where M: Flatten
{
    pub fn add_to_buffer(self, buf: &mut Vec<f32>) {
        buf.extend(self.flatten())
    }
}

impl<M> Flatten for Object<M>
where M: Flatten
{
    fn flatten(self) -> Vec<f32> {
        let mut data = self.model.flatten();
        data.extend(self.material.flatten());
        data
    }
}

// fn test() {
//     // let m = Unary::Rounding(0.5, Primitive::Sphere { center: (0., 0., 0.), radius: 1. });
//     // let model = Unary::Rounding {
//     //     model: Primitive::Sphere {
//     //         center: (0., 0., 0.),
//     //         radius: 1.
//     //     },
//     //     radius: 0.5
//     // };
//     let object = Object {
//         model: Unary::Rounding {
//             model: Primitive::Sphere {
//                 center: (0., 0., 0.),
//                 radius: 1.
//             },
//             radius: 0.5
//         },
//         material: Material::default()
//     };
// }