extern crate cgmath;
extern crate rand;
use self::cgmath::InnerSpace;
use self::cgmath::Vector3;
use self::rand::prelude::*;
use ray::Ray;
use std::f64;

fn random_in_unit_sphere() -> Vector3<f64> {
    Vector3::new(random::<f64>(), random::<f64>(), random::<f64>()).normalize()
}

pub struct Camera {
    pub lower_left: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub origin: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
    pub lens_radius: f64,
}

impl Camera {
    #[must_use]
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let (u, v, w);
        let lens_radius = aperture / 2.0;
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        w = (look_from - look_at).normalize();
        u = vup.cross(w).normalize();
        v = w.cross(u);

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Self {
            lower_left: lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    #[must_use]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset: Vector3<f64> = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
