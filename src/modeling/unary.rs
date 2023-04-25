use super::{flatten::Flatten, primitive::Primitive};

const ROUNDING_ID: f32 = 100.;

pub enum Unary {
    Rounding {
        model: Primitive,
        radius: f32
    }
}

impl Flatten for Unary {
    fn flatten(self) -> Vec<f32> {
        match self {
            Unary::Rounding { radius, model } => {
                let mut buf = Vec::new();
                buf.push(ROUNDING_ID);
                buf.push(radius);
                buf.extend(model.flatten());
                buf
            },
        }
    }
}