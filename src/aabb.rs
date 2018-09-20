extern crate cgmath;
use self::cgmath::{InnerSpace, Vector3};
use ray::Ray;
use std::cmp;

pub struct Aabb {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl Aabb {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> Aabb {
        Aabb { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: &mut f64, t_max: &mut f64) -> bool {
        for i in 0..3 {
            let invD = 1.0 / r.direction[i];
            let mut t0 = (self.min[i] - r.origin[i]) * invD;
            let mut t1 = (self.max[i] - r.origin[i]) * invD;
            if invD < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            *t_min = t0.max(*t_min);
            *t_max = t1.min(*t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
