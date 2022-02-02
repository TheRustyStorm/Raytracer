extern crate cgmath;
use hitable::HitRecord;
use hitable::Hitable;
use ray::Ray;

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    #[must_use]
    pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
        Self { list }
    }
}


impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(t_max);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            let element = &self.list[i];
            if element.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.material = temp_rec.material.clone();
            }
        }
        hit_anything
    }

}
