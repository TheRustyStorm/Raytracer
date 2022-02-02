extern crate cgmath;
use self::cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction,
        }
    }
    #[must_use]
    pub fn get_pos_on_ray(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}
