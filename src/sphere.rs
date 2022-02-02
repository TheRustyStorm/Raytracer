extern crate cgmath;
use self::cgmath::{InnerSpace, Vector3};
use hitable::HitRecord;
use hitable::Hitable;
use material::Material;
use ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b.powi(2) - a * c;

        if d > 0.0 {
            let mut temp = (-b - (b.powi(2) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.get_pos_on_ray(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            temp = (-b + (b.powi(2) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.get_pos_on_ray(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
        }
        false
    }

}
