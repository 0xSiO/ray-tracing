use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::vec3::Vec3;

mod vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 512;
    let height = 512;

    let mut writer = BufWriter::new(File::create("image.ppm")?);

    writeln!(&mut writer, "P3\n{} {}\n255", width, height)?;

    for y in (0..height).rev() {
        println!("Lines remaining: {}", y);

        for x in 0..width {
            let color = Vec3(
                (x as f64) / (width - 1) as f64,
                (y as f64) / (height - 1) as f64,
                0.25,
            ) * 255.999;

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
