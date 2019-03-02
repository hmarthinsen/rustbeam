//! Module containing mathematical structs.
use std::ops::{Add, Mul, Neg, Sub};

/// A closed interval in the set of real numbers.
pub struct Interval {
    endpoints: (f64, f64),
}

impl Interval {
    pub fn new(first_endpoint: f64, second_endpoint: f64) -> Self {
        if first_endpoint <= second_endpoint {
            Self {
                endpoints: (first_endpoint, second_endpoint),
            }
        } else {
            Self {
                endpoints: (second_endpoint, first_endpoint),
            }
        }
    }

    pub fn get_endpoints(self) -> (f64, f64) {
        self.endpoints
    }

    pub fn intersection(self, other: Interval) -> Option<Interval> {
        let lower = self.endpoints.0.max(other.endpoints.0);
        let upper = self.endpoints.1.min(other.endpoints.1);

        if upper < lower {
            None
        } else {
            Some(Interval::new(lower, upper))
        }
    }
}

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnitQuaternion {
    real: f64,
    imag: Vector3,
}

impl UnitQuaternion {
    fn new<T: Into<Vector3>>(real: f64, imag: T) -> Self {
        Self {
            real,
            imag: imag.into(),
        }
    }

    pub fn from_axis_angle<T: Into<Vector3>>(rotation_axis: T, angle: f64) -> Self {
        let (sin, cos) = (0.5 * angle).sin_cos();
        Self {
            real: cos,
            imag: rotation_axis.into().normalize() * sin,
        }
    }

    pub fn id() -> Self {
        Self::new(1.0, (0.0, 0.0, 0.0))
    }

    pub fn i() -> Self {
        Self::new(0.0, (1.0, 0.0, 0.0))
    }

    pub fn j() -> Self {
        Self::new(0.0, (0.0, 1.0, 0.0))
    }

    pub fn k() -> Self {
        Self::new(0.0, (0.0, 0.0, 1.0))
    }

    fn invert(mut self) -> Self {
        self.imag = -self.imag;
        self
    }
}

impl Mul for UnitQuaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.real * other.real - self.imag.dot(other.imag),
            self.real * other.imag + other.real * self.imag + self.imag.cross(other.imag),
        )
    }
}

/// A 3D vector
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn i() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn j() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn k() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Dot product of two vectors.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    ///Square of norm of the vector.
    pub fn norm2(self) -> f64 {
        self.dot(self)
    }

    /// Norm of the vector.
    fn norm(self) -> f64 {
        self.norm2().sqrt()
    }

    pub fn normalize(self) -> Self {
        if !self.is_zero() {
            let recip_norm = 1.0 / self.norm();
            self * recip_norm
        } else {
            self
        }
    }

    fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn rotate(self, rotation: UnitQuaternion) -> Self {
        let q = UnitQuaternion::new(0.0, self);
        let q_rotated = rotation * q * rotation.invert();

        q_rotated.imag
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(coordinates: (f64, f64, f64)) -> Self {
        Self::new(coordinates.0, coordinates.1, coordinates.2)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from((self.x + other.x, self.y + other.y, self.z + other.z))
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}
