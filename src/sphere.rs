extern crate cgmath;
use self::cgmath::{InnerSpace, Vector3};
use aabb::Aabb;
use hitable::HitRecord;
use hitable::Hitable;
use material::Material;
use ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Rc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.get_pos_on_ray(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
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

    fn bounding_box(&self, t0: f32, t1: f32, b: &mut Aabb) -> bool {
        *b = Aabb::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
