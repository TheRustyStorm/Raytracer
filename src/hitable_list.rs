extern crate cgmath;
use hitable::Hitable;
use hitable::HitRecord;
use ray::Ray;
use self::cgmath::{Vector3, InnerSpace};
use aabb::Aabb;
pub struct HitableList{
    pub list: Vec<Box<Hitable>>
}

impl  HitableList{
    pub fn new(list: Vec<Box<Hitable>>)->HitableList{
        HitableList{list}
    }
}

fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb{
    let small = Vector3::new(box0.min.x.min(box1.min.x),box0.min.y.min(box1.min.y),box0.min.z.min(box1.min.z));
    let big = Vector3::new(box0.max.x.min(box1.max.x),box0.max.y.min(box1.max.y),box0.max.z.min(box1.max.z));
    Aabb::new(small, big)
}

impl Hitable for HitableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(t_max);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len(){
            let element = &self.list[i];
            if element.hit(ray, t_min, closest_so_far,&mut temp_rec){
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

    fn bounding_box(&self, t0: f32, t1: f32, b: &mut Aabb) -> bool{
        if self.list.len() < 1{
            return false;
        }
        let mut temp_box = Aabb::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0));
        let first_true = self.list[0].bounding_box(t0, t1, &mut temp_box);
        if !first_true{
            return false;
        }else{
            *b = temp_box;
        }
        let mut temp_box = Aabb::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0));
        for i in &self.list{
            if self.list[0].bounding_box(t0, t1, &mut temp_box){
                *b = surrounding_box(&b, &temp_box);
            }
            else{
                return false;
            }
        }


        true
    }

    
}