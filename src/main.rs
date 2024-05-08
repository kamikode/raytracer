use raytracer::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("test.ppm")?;
    let mut canvas = Canvas::<3, 2>::new();
    canvas.set_pixel(0, 0, Color::red())?;
    canvas.set_pixel(1, 0, Color::green())?;
    canvas.set_pixel(2, 0, Color::blue())?;
    canvas.set_pixel(0, 1, Color::yellow())?;
    canvas.set_pixel(1, 1, Color::white())?;
    canvas.set_pixel(2, 1, Color::black())?;
    canvas.write_ppm(&mut file)?;
    Ok(())
}
