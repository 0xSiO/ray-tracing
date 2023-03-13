use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{ray::Ray, vec3::Vec3};

mod ray;
mod vec3;

/// Point in 3D space: x, y, z
type Point = Vec3<f64>;
/// RGB values, between 0.0 and 1.0
type Color = Vec3<f64>;

// Finds point at which ray hits sphere, using this equation:
// (t**2)(b⋅b) + 2tb⋅(A−C) + (A−C)⋅(A−C) - r**2 = 0
//
// A: position of the ray
// b: direction of the ray
// C: center of the sphere
// r: radius of the sphere
fn intersect_sphere(center: Point, radius: f64, ray: Ray) -> Option<Point> {
    let dir = ray.dir();
    let a = dir.dot(dir);
    let b = 2. * dir.dot(ray.pos() - center);
    let c = (ray.pos() - center).dot(ray.pos() - center) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        None
    } else {
        Some(ray.at((-b - discriminant.sqrt()) / (2. * a)))
    }
}

fn color_at(ray: Ray) -> Color {
    let sphere_center = Vec3(0., 0., -1.);
    if let Some(point) = intersect_sphere(sphere_center, 0.5, ray) {
        let n = (point - sphere_center).normalize();
        return Vec3(n.0 + 1., n.1 + 1., n.2 + 1.) / 2.;
    }

    // t = 0 at y = -1, t = 1 at y = 1
    let t = (ray.dir().1 + 1.) / 2.;
    // Linear blend of white and blue
    Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let width = 1000_usize;
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

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let color = color_at(ray);

            writeln!(
                &mut writer,
                "{} {} {}",
                (color.0 * 255.999) as u8,
                (color.1 * 255.999) as u8,
                (color.2 * 255.999) as u8,
            )?;
        }
    }

    writer.flush()?;

    Ok(())
}
