extern crate cgmath;
extern crate rand;
use self::cgmath::InnerSpace;
use self::cgmath::Vector3;
use self::rand::prelude::*;
use hitable::HitRecord;
use ray::Ray;
use std::f64;

fn random_in_unit_sphere() -> Vector3<f64> {
    Vector3::new(
        2.0 * random::<f64>() - 1.0,
        2.0 * random::<f64>() - 1.0,
        2.0 * random::<f64>() - 1.0,
    )
    .normalize()
}

fn random_in_unit_sphere_new() -> Vector3<f64> {
    let mut v;
    loop {
        v = Vector3::new(
            2.0 * random::<f64>() - 1.0,
            2.0 * random::<f64>() - 1.0,
            2.0 * random::<f64>() - 1.0,
        );
        if v.magnitude() < 1.0 {
            break;
        }
    }
    v
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(*n) * n
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        return Some(ni_over_nt * (uv - n * dt) - n * d.sqrt());
    }
    None
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f64>, bool);
}

pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64,
}

pub struct Dielectric {
    pub ref_idx: f64,
}

pub struct Emitting {
    pub emission: Vector3<f64>,
    pub brightness: f64,
}

impl Lambertian {
    #[must_use]
    pub const fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Metal {
    #[must_use]
    pub const fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Dielectric {
    #[must_use]
    pub const fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
    }
}

impl Emitting {
    #[must_use]
    pub const fn new(emission: Vector3<f64>, brightness: f64) -> Self {
        Self {
            emission,
            brightness,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f64>, bool) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere_new();
        (
            Ray::new(hit_record.p, target - hit_record.p),
            self.albedo,
            true,
        )
    }
}

impl Material for Emitting {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f64>, bool) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere_new();
        (
            Ray::new(hit_record.p, target - hit_record.p),
            self.emission * self.brightness,
            true,
        )
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f64>, bool) {
        let reflected = reflect(&r_in.direction.normalize(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        let valid = scattered.direction.dot(hit_record.normal) > 0.0;
        (scattered, self.albedo, valid)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (Ray, Vector3<f64>, bool) {
        let outward_normal: Vector3<f64>;
        let reflected = reflect(&r_in.direction, &hit_record.normal);
        let ni_over_nt: f64;
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let refracted: Vector3<f64>;
        let scattered;
        let reflected_prob: f64;
        let cosine = if r_in.direction.dot(hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            self.ref_idx * r_in.direction.dot(hit_record.normal) / r_in.direction.magnitude()
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            -(r_in.direction.dot(hit_record.normal) / r_in.direction.magnitude())
        };
        if let Some(refract) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
            refracted = refract;
            reflected_prob = schlick(cosine, self.ref_idx);
            if random::<f64>() < reflected_prob {
                scattered = Ray::new(hit_record.p, reflected);
            } else {
                scattered = Ray::new(hit_record.p, refracted);
            }
        } else {
            scattered = Ray::new(hit_record.p, reflected);
        }
        (scattered, attenuation, true)
    }
}
