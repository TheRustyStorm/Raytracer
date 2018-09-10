extern crate cgmath;
use self::cgmath::Vector3;
use aabb::Aabb;
use material::Material;
use ray::Ray;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: Option<Rc<Material>>,
}

impl HitRecord {
    pub fn new(t: f32) -> HitRecord {
        HitRecord {
            t,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut Aabb) -> bool;
}
