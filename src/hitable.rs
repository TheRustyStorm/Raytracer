extern crate cgmath;
use self::cgmath::Vector3;
use aabb::Aabb;
use material::Material;
use ray::Ray;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Option<Rc<Material>>,
}

impl HitRecord {
    pub fn new(t: f64) -> HitRecord {
        HitRecord {
            t,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, b: &mut Aabb) -> bool;
}
