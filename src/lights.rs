use crate::math::Vector3;

pub struct Sun {
    pub color: Vector3,
    pub direction: Vector3,
}

impl Sun {
    pub fn new<T: Into<Vector3>, U: Into<Vector3>>(color: T, direction: U) -> Self {
        Self {
            color: color.into(),
            direction: direction.into().normalize(),
        }
    }
}
