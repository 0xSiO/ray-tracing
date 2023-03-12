use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{ray::Ray, vec3::Vec3};

mod ray;
mod vec3;

// Determines whether there are 2 real solutions for t in the equation:
// (t**2)(b⋅b) + 2tb⋅(A−C) + (A−C)⋅(A−C) - r**2 = 0
//
// A: position of the ray
// b: direction of the ray
// C: center of the sphere
// r: radius of the sphere
fn hits_sphere(center: Vec3<f64>, radius: f64, ray: &Ray<f64>) -> bool {
    let dir = ray.dir.normalize();
    let a = dir.dot(dir);
    let b = 2. * dir.dot(ray.pos - center);
    let c = (ray.pos - center).dot(ray.pos - center) - radius * radius;

    (b * b - 4. * a * c) > 0.
}

fn color_at(ray: &Ray<f64>) -> Vec3<f64> {
    if hits_sphere(Vec3(0., 0., -1.), 0.5, ray) {
        return Vec3(1., 0., 0.) * 255.999;
    }

    let dir = ray.dir.normalize();
    // t = 0 at y = -1, t = 1 at y = 1
    let t = (dir.1 + 1.) / 2.;
    // Linear blend of white and blue
    (Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t) * 255.999
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let width = 1200 as usize;
    let height = (width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0., 0., 0.);
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    let mut writer = BufWriter::new(File::create("image.ppm")?);

    writeln!(&mut writer, "P3\n{} {}\n255", width, height)?;

    for y in (0..height).rev() {
        println!("Lines remaining: {}", y);

        for x in 0..width {
            let u = (x as f64) / (width - 1) as f64;
            let v = (y as f64) / (height - 1) as f64;

            let ray = Ray {
                pos: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };

            let color = color_at(&ray);

            writeln!(
                &mut writer,
                "{} {} {}",
                color.0 as i32, color.1 as i32, color.2 as i32,
            )?;
        }
    }

    writer.flush()?;

    Ok(())
}
