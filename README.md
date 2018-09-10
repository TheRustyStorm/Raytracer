# Raytracer
A Raytracer written in pure Rust.
Mostly based on "Raytracing in a weekend" from Peter Shirley.
Added support for multithreading and improved speed a little.

## Configuration
In main.rs search for the creation of spheres, you have three possible materials for usage:
- Lambertian
- Metal
- Dielectric

Adapt the spheres or add new ones if you like.

## Exection
Run with 
```
cargo run --release
```

## Why another?

The point in writing another Raytracer for me was to check out the [cgmath](https://crates.io/crates/cgmath) crate.

## Examples

Here are some sample pictures:

![](https://github.com/TheSovietStorm/RayTracer/blob/master/image%202.png)
![](https://github.com/TheSovietStorm/RayTracer/blob/master/image%209.png)
![](https://github.com/TheSovietStorm/RayTracer/blob/master/image%2010.png)





