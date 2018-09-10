extern crate cgmath;
extern crate rand;
use self::cgmath::InnerSpace;
use self::cgmath::Vector3;
use self::rand::prelude::*;
use ray::Ray;
use std::f32;

fn random_in_unit_sphere() -> Vector3<f32> {
    Vector3::new(random::<f32>(), random::<f32>(), random::<f32>()).normalize()
}

pub struct Camera {
    pub lower_left: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub origin: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let (u, v, w);
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
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

        Camera {
            lower_left: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset: Vector3<f32> = self.u * rd.x + self.v * rd.y;
        Ray::new(
            &(self.origin + offset),
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
