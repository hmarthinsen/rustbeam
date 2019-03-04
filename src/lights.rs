//! Module containing different light sources.

use crate::math::Vector3;

/// A light source emitting parallel light rays from a specified direction.
pub struct Sun {
    /// The color of the light rays, in linear RGB.
    pub color: Vector3,
    /// The direction the rays point in. Must be a unit vector.
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
