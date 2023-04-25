use super::flatten::Flatten;

const SPHERE_ID: f32 = 0.;
const RECT_PRISM_ID: f32 = 1.;

pub enum Primitive {
    Sphere {
        center: (f32, f32, f32),
        radius: f32
    },
    RectPrism {
        center: (f32, f32, f32),
        extents: (f32, f32, f32)
    }
}

impl Flatten for Primitive {
    fn flatten(self) -> Vec<f32> {
        use Primitive::*;
        match self {
            Sphere { center, radius } => {
                vec![
                    SPHERE_ID,
                    center.0,
                    center.1,
                    center.2,
                    radius
                ]
            },
            RectPrism { center, extents } => {
                vec![
                    RECT_PRISM_ID,
                    center.0,
                    center.1,
                    center.2,
                    extents.0,
                    extents.1,
                    extents.2
                ]
            },
        }
    }
}