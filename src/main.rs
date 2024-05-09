use raytracer::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Write test.ppm file.
    let mut file = File::create("test.ppm")?;
    let mut canvas = Canvas::<3, 2>::new();
    canvas.set_pixel(0, 0, Color::red())?;
    canvas.set_pixel(1, 0, Color::green())?;
    canvas.set_pixel(2, 0, Color::blue())?;
    canvas.set_pixel(0, 1, Color::yellow())?;
    canvas.set_pixel(1, 1, Color::white())?;
    canvas.set_pixel(2, 1, Color::black())?;
    canvas.write_ppm(&mut file)?;

    let v = Vector::new(0.0, 1.0, 2.0);
    let p = Point::new(-1.0, 0.0, 0.5);
    let m = Matrix4x4::new([
        [0.0, 0.1, 0.2, 0.3],
        [1.0, 1.1, 1.2, 1.3],
        [2.0, 2.1, 2.2, 2.3],
        [3.0, 3.1, 3.2, 3.3],
    ]);
    println!("{:+.3}", p);
    println!("{:+.3}", v);
    println!("{:+.3}", m);
    Ok(())
}
