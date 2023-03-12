use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 512;
    let height = 512;

    let mut writer = BufWriter::new(File::create("image.ppm")?);

    write!(&mut writer, "P3\n{} {}\n255\n", width, height)?;

    for y in (0..height).rev() {
        println!("Lines remaining: {}", y);

        for x in 0..width {
            let r = (x as f64) / (width - 1) as f64;
            let g = (y as f64) / (height - 1) as f64;
            let b = 0.25;

            write!(
                &mut writer,
                "{} {} {}\n",
                (r * 255.999) as i32,
                (g * 255.999) as i32,
                (b * 255.999) as i32,
            )?;
        }
    }

    writer.flush()?;

    Ok(())
}
