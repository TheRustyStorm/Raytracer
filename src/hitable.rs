extern crate cgmath;
use self::cgmath::Vector3;
use material::Material;
use ray::Ray;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    #[must_use]
    pub fn new(t: f64) -> Self {
        Self {
            t,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
