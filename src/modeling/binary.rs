use super::{flatten::Flatten, primitive::Primitive};

const INTERSECTION_ID: f32 = 200.;
const SMOOTH_UNION_ID: f32 = 201.;

pub enum Binary {
    Intersection {
        model1: Primitive,
        model2: Primitive
    },
    SmoothUnion {
        k: f32,
        model1: Primitive,
        model2: Primitive
    }
}

impl Flatten for Binary {
    fn flatten(self) -> Vec<f32> {
        use Binary::*;
        match self {
            Intersection { model1, model2 } => {
                let mut buf = Vec::new();
                buf.push(INTERSECTION_ID);
                buf.extend(model1.flatten());
                buf.extend(model2.flatten());
                buf
            },
            SmoothUnion { k, model1, model2 } => {
                let mut buf = Vec::new();
                buf.push(SMOOTH_UNION_ID);
                buf.push(k);
                buf.extend(model1.flatten());
                buf.extend(model2.flatten());
                buf
            }
        }
    }
}