use super::flatten::Flatten;

const SPHERE_ID: f32 = 0.;
const RECT_PRISM_ID: f32 = 1.;
const CYLINDER_ID: f32 = 2.;
const CAPSULE_ID: f32 = 3.;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Sphere {
        center: (f32, f32, f32),
        radius: f32
    },
    RectPrism {
        center: (f32, f32, f32),
        extents: (f32, f32, f32)
    },
    Cylinder {
        point1: (f32, f32, f32),
        point2: (f32, f32, f32),
        radius: f32
    },
    Capsule {
        point1: (f32, f32, f32),
        point2: (f32, f32, f32),
        radius: f32
    }
}

impl Flatten for Primitive {
    fn flatten(self) -> Vec<f32> {
        use Primitive::*;
        match self {
            Sphere { center, radius } => vec![
                SPHERE_ID,
                center.0,
                center.1,
                center.2,
                radius
            ],
            RectPrism { center, extents } => vec![
                RECT_PRISM_ID,
                center.0,
                center.1,
                center.2,
                extents.0,
                extents.1,
                extents.2
            ],
            Cylinder { point1, point2, radius } => vec![
                CYLINDER_ID,
                point1.0,
                point1.1,
                point1.2,
                point2.0,
                point2.1,
                point2.2,
                radius
            ],
            Capsule { point1, point2, radius } => vec![
                CAPSULE_ID,
                point1.0,
                point1.1,
                point1.2,
                point2.0,
                point2.1,
                point2.2,
                radius
            ]
        }
    }
}