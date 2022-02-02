extern crate cgmath;
extern crate indicatif;
extern crate pathtracer;
extern crate png;
extern crate rand;
use cgmath::{InnerSpace, Vector3};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pathtracer::camera::Camera;
use pathtracer::hitable::{HitRecord, Hitable};
use pathtracer::hitable_list::HitableList;
use pathtracer::material::{Dielectric, Lambertian, Metal, Emitting};
use pathtracer::ray::Ray;
use pathtracer::sphere::Sphere;
use png::HasParameters; use rand::prelude::*;
use std::f64;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn color<T: Hitable>(ray: &Ray, world: &T, depth: usize) -> Vector3<f64> {
    let mut hit_record = HitRecord::new(f64::MAX);
    if world.hit(ray, 0.001, f64::MAX, &mut hit_record) {
        if let Some(ref mat) = hit_record.material {
            let (scattered, attenuation, valid) = mat.scatter(ray, &hit_record);
            if valid && depth < 10 {
                let col = color(&scattered, world, depth + 1);
                return Vector3::new(
                    col.x * attenuation.x,
                    col.y * attenuation.y,
                    col.z * attenuation.z,
                );
            } else {
                return Vector3::new(1.0, 1.0, 1.0);
            }
        }
    }
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

#[allow(clippy::redundant_clone)]
fn trace(
    vec: &mut Vec<u8>,
    width: usize,
    height: usize,
    min_height: usize,
    max_height: usize,
    num_samples: usize,
    bar: ProgressBar,
) {
    let look_from = Vector3::new(0.0, 2.0, 15.0);
    let look_at = Vector3::new(0.0, 0.0, -5.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        width as f64 / height as f64,
        aperture,
        dist_to_focus,
    );
    let mut hitable_list = HitableList::new(Vec::new());

    //red
    let _red = Rc::new(Lambertian::new(Vector3::new(1.0, 0.0, 0.0)));
    let _red_light = Rc::new(Emitting::new(Vector3::new(1.0, 0.0, 0.0), 20.0));
    //green
    let _green = Rc::new(Lambertian::new(Vector3::new(0.0, 1.0, 0.0)));
    //blue
    let _blue = Rc::new(Lambertian::new(Vector3::new(0.0, 0.0, 1.0)));
    //white
    let _white = Rc::new(Lambertian::new(Vector3::new(1.0, 1.0, 1.0)));
    //red
    let _red_metal = Rc::new(Metal::new(Vector3::new(1.0, 0.0, 0.0), 0.1));
    //green
    let _green_metal = Rc::new(Metal::new(Vector3::new(0.0, 1.0, 0.0), 0.1));
    //blue
    let _blue_metal = Rc::new(Metal::new(Vector3::new(0.0, 0.0, 1.0), 0.1));
    //white
    let _white_metal = Rc::new(Metal::new(Vector3::new(1.0, 1.0, 1.0), 0.01));
    //black \m/
    let _black_metal = Rc::new(Metal::new(Vector3::new(0.0, 0.0, 0.0), 0.1));
    //black
    let _black = Rc::new(Lambertian::new(Vector3::new(0.0, 0.0, 0.0)));
    //gold
    let _gold = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3));
    //silver
    let _silver = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.01));
    let _metal = Rc::new(Metal::new(Vector3::new(0.9, 0.9, 0.9), 0.01));
    //glass
    let _glass = Rc::new(Dielectric::new(1.52));

    //jules farben
    let _color1 = Rc::new(Lambertian::new(Vector3::new(0.5176, 0.4392, 1.0)));
    let _color2 = Rc::new(Lambertian::new(Vector3::new(0.8039, 0.3607, 0.3607)));
    let _color3 = Rc::new(Lambertian::new(Vector3::new(0.6, 0.1960, 0.8)));
    let _color4 = Rc::new(Lambertian::new(Vector3::new(0.8666, 0.6274, 0.8666)));
    let _color5 = Rc::new(Lambertian::new(Vector3::new(0.6901, 0.8862, 1.0)));
    let _color6 = Rc::new(Metal::new(Vector3::new(0.0, 1.0, 0.6039), 0.39));
    let _color7 = Rc::new(Metal::new(Vector3::new(0.6039, 1.0, 0.6039), 0.67));
    let _floor =  Rc::new(Lambertian::new(Vector3::new(0.2117, 0.2117, 0.2117)));

    /*
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-0.7, 0.5, 0.5),
        0.1,
        _red_light.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.7, -0.5, 0.5),
        0.1,
        _green.clone(),
    )));

    hitable_list.list.push(Box::new(Triangle::new(
        Vector3::new(1.0, 0.0, 1.0),
        Vector3::new(-1.0, 0.0, 1.0),
        Vector3::new(-1.0, 0.0, -1.0),
        _white_metal.clone(),
    )));
    */


    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -104.5),
        100.0,
        _white.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        _white.clone(),
    )));
    
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-103.5, 0.0, -1.0),
        100.0,
        _red.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(103.5, 0.0, -1.0),
        100.0,
        _blue.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 103.5, -1.0),
        100.0,
        _white.clone(),
    )));

    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.75, 0.5),
        1.0,
        _white_metal.clone(),
    )));

    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(1.0, 0.75, 2.5),
        1.0,
        _glass.clone(),
    )));

    /*
    for y in 0..15 {
        for x in 0..15 {
            let num = y * 15 + x;
            hitable_list.list.push(Box::new(Sphere::new(
                Vector3::new(x as f64 - 15.0/2.0, 0.0, -(y + 1) as f64),
                0.5,
                match num % 7{
                0 => _red.clone(),
                1 => _white_metal.clone(),
                2 => _green.clone(),
                3 => _white_metal.clone(),
                4 => _white_metal.clone(),
                5 => _blue.clone(),
                _ => _white_metal.clone(),
                
            },
            )));
        }
    }
    */
    
    
    /*
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        _white.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        _glass.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        _white_metal.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        _metal.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(2.0, 0.0, -4.0),
        0.5,
        _red.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -4.0),
        0.5,
        _green.clone(),
    )));
    hitable_list.list.push(Box::new(Sphere::new(
        Vector3::new(-2.0, 0.0, -4.0),
        0.5,
        _blue.clone(),
    )));
    */
    for y in (min_height..max_height).rev() {
        for x in 0..width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u: f64 = (x as f64 + random::<f64>()) / width as f64;
                let v: f64 = (y as f64 + random::<f64>()) / height as f64;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &hitable_list, 0);
            }
            col /= num_samples as f64;
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            vec.push((col.x * 255.99) as u8);
            vec.push((col.y * 255.99) as u8);
            vec.push((col.z * 255.99) as u8);
            vec.push(255);
        }
        bar.inc(1);
    }
    bar.finish();
}

fn main() {
    let width: usize = 3840;
    let height: usize = 2160;
    let num_samples: usize = 10;

    let mut vec: Vec<u8> = Vec::with_capacity(width * height * 3);

    let amount_threads: usize = 15;
    let mut image_slices: Vec<Arc<Mutex<Vec<u8>>>> = Vec::new();
    let mbar: MultiProgress = MultiProgress::new();

    for _ in 0..amount_threads {
        image_slices.push(Arc::new(Mutex::new(Vec::with_capacity(
            width * height * 3 / amount_threads,
        ))));
    }
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for (i, _) in image_slices.iter().enumerate().take(amount_threads) {
        let threadslice = image_slices[i].clone();
        let pbar = ProgressBar::new((height / amount_threads) as u64);
        pbar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {bar:40.cyan/blue} [{eta:>5}] {pos:>7}/{len:7} ")
                .progress_chars("##-"),
        );
        let b = mbar.add(pbar);

        let t1 = thread::spawn(move || {
            let mut vec = threadslice.lock().unwrap();
            trace(
                &mut vec,
                width,
                height,
                i * height / amount_threads,
                (i + 1) * height / amount_threads,
                num_samples,
                b,
            );
        });
        handles.push(t1);
    }
    mbar.join_and_clear().unwrap();

    for handle in handles {
        match handle.join() {
            Ok(_) => {}
            Err(e) => println!("Panicked {:?}", e),
        }
    }

    for slice in image_slices.into_iter().rev() {
        let guard = match slice.lock() {
            Ok(guard) => guard,
            Err(e) => e.into_inner(),
        };
        let mut v: Vec<u8> = guard.to_vec();
        vec.append(&mut v);
    }

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&vec).unwrap(); // Save
}
