extern crate cgmath;
extern crate rand;
use self::cgmath::InnerSpace;
use self::cgmath::Vector3;
use self::rand::prelude::*;
use hitable::HitRecord;
use ray::Ray;
use std::f32;

fn random_in_unit_sphere() -> Vector3<f32> {
    Vector3::new(
        2.0 * random::<f32>() - 1.0,
        2.0 * random::<f32>() - 1.0,
        2.0 * random::<f32>() - 1.0,
    ).normalize()
}

fn random_in_unit_sphere_new() -> Vector3<f32> {
    let mut v;
    loop {
        v = Vector3::new(
            2.0 * random::<f32>() - 1.0,
            2.0 * random::<f32>() - 1.0,
            2.0 * random::<f32>() - 1.0,
        );
        if v.magnitude() < 1.0 {
            break;
        }
    }
    v
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*n) * n
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        return Some(ni_over_nt * (uv - n * dt) - n * d.sqrt());
    }
    None
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f32>, bool);
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f32>, bool) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere_new();
        (
            Ray::new(&hit_record.p, target - hit_record.p),
            self.albedo,
            true,
        )
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f32>, bool) {
        let reflected = reflect(&r_in.direction.normalize(), &hit_record.normal);
        let scattered = Ray::new(
            &hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        let valid = scattered.direction.dot(hit_record.normal) > 0.0;
        (scattered, self.albedo, valid)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f32>, bool) {
        let outward_normal: Vector3<f32>;
        let reflected = reflect(&r_in.direction, &hit_record.normal);
        let ni_over_nt: f32;
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let refracted: Vector3<f32>;
        let scattered;
        let reflected_prob: f32;
        let cosine: f32;
        if r_in.direction.dot(hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine =
                self.ref_idx * r_in.direction.dot(hit_record.normal) / r_in.direction.magnitude();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -(r_in.direction.dot(hit_record.normal) / r_in.direction.magnitude());
        }
        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refract) => {
                refracted = refract;
                reflected_prob = schlick(cosine, self.ref_idx);
                if random::<f32>() < reflected_prob {
                    scattered = Ray::new(&hit_record.p, reflected);
                } else {
                    scattered = Ray::new(&hit_record.p, refracted);
                }
            }
            None => {
                reflected_prob = 1.0;
                scattered = Ray::new(&hit_record.p, reflected);
            }
        }
        (scattered, attenuation, true)
    }
}
