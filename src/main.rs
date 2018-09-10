extern crate cgmath;
extern crate pathtracer;
extern crate png;
extern crate rand;
use cgmath::{InnerSpace, Vector3};
use pathtracer::camera::Camera;
use pathtracer::hitable::{HitRecord, Hitable};
use pathtracer::hitable_list::HitableList;
use pathtracer::material::{Dielectric, Lambertian, Metal};
use pathtracer::ray::Ray;
use pathtracer::sphere::Sphere;
use png::HasParameters;
use rand::prelude::*;
use std::f32;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn color<T: Hitable>(ray: &Ray, world: &T, depth: usize) -> Vector3<f32> {
    let mut hit_record = HitRecord::new(f32::MAX);
    if world.hit(ray, 0.001, f32::MAX, &mut hit_record) {
        match hit_record.material.clone() {
            Some(mat) => {
                let (scattered, attenuation, valid) = mat.scatter(ray, &hit_record);
                if valid && depth < 100 {
                    let col = color(&scattered, world, depth + 1);
                    return Vector3::new(
                        col.x * attenuation.x,
                        col.y * attenuation.y,
                        col.z * attenuation.z,
                    );
                } else {
                    return Vector3::new(0.0, 0.0, 0.0);
                }
            }
            None => {}
        }
    }
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn trace(
    vec: &mut Vec<u8>,
    width: usize,
    height: usize,
    min_height: usize,
    max_height: usize,
    num_samples: usize,
) {
    let look_from = Vector3::new(0.0, 0.0, 5.0);
    let look_at = Vector3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        width as f32 / height as f32,
        aperture,
        dist_to_focus,
    );
    let mut hitable_list = HitableList::new(Vec::new());

    //red
    let red = Rc::new(Lambertian::new(Vector3::new(1.0, 0.0, 0.0)));
    //gold
    let gold = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3));
    //silver
    let silver = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.2));

    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        red.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        gold.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        gold.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        silver.clone(),
    )));

    for y in (min_height..max_height).rev() {
        for x in 0..width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u: f32 = (x as f32 + random::<f32>()) / width as f32;
                let v: f32 = (y as f32 + random::<f32>()) / height as f32;
                let ray = camera.get_ray(u, v);
                col = col + color(&ray, &hitable_list, 0);
            }
            col = col / num_samples as f32;
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            vec.push((col.x * 255.99) as u8);
            vec.push((col.y * 255.99) as u8);
            vec.push((col.z * 255.99) as u8);
            vec.push(255);
        }
    }
}

fn main() {
    let width: usize = 2560;
    let height: usize = 1280;
    let num_samples: usize = 1000;

    let mut vec: Vec<u8> = Vec::with_capacity(width * height * 3);

    let amount_threads = 4;
    let mut image_slices: Vec<Arc<Mutex<Vec<u8>>>> = Vec::new();
    for _ in 0..amount_threads {
        image_slices.push(Arc::new(Mutex::new(Vec::with_capacity(
            width * height * 3 / amount_threads,
        ))));
    }
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in 0..amount_threads {
        let threadslice = image_slices[i].clone();
        let t1 = thread::spawn(move || {
            let mut vec = threadslice.lock().unwrap();
            trace(
                &mut vec,
                width,
                height,
                i * height / amount_threads,
                (i + 1) * height / amount_threads,
                num_samples,
            );
        });
        handles.push(t1);
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => {}
            Err(e) => println!("Panicked {:?}", e),
        }
    }

    for slice in image_slices.into_iter().rev() {
        let mut guard = match slice.lock() {
            Ok(guard) => guard,
            Err(e) => e.into_inner(),
        };
        let v: Vec<u8> = guard.to_vec();
        for elem in v {
            vec.push(elem);
        }
    }

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&vec).unwrap(); // Save
}
