use std::{
    f64::INFINITY,
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    geometry::{Hit, Objects, Ray, Sphere},
    vec3::Vec3,
};

mod geometry;
mod vec3;

/// RGB values, between 0.0 and 1.0
type Color = Vec3<f64>;

fn ray_color(ray: Ray, world: &Objects) -> Color {
    if let Some(hit) = world.find_ray_hit(ray, 0., INFINITY) {
        let n = hit.normal();
        return (n + Vec3(1., 1., 1.)) / 2.;
    }

    // t = 0 at y = -1, t = 1 at y = 1
    let t = (ray.dir().1 + 1.) / 2.;
    // Linear blend of white and blue
    Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let width = 1000_usize;
    let height = (width as f64 / aspect_ratio) as usize;

    // World

    let mut world = Objects::new();
    world.add(Sphere::new(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new(Vec3(0., -100.5, -1.), 100.));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0., 0., 0.);
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    // Render

    let mut writer = BufWriter::new(File::create("image.ppm")?);

    writeln!(&mut writer, "P3\n{} {}\n255", width, height)?;

    for y in (0..height).rev() {
        // println!("Lines remaining: {}", y);

        for x in 0..width {
            let u = (x as f64) / (width - 1) as f64;
            let v = (y as f64) / (height - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let color = ray_color(ray, &world);

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
