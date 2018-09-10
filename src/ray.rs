extern crate cgmath;
use self::cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: &Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction,
        }
    }
    pub fn get_pos_on_ray(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}
