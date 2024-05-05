use raytracer::*;

fn main() {
    let p = Point::new(3.0, -2.0, 5.0);
    let v = Vector::new(-2.0, 3.0, 1.0);
    println!("{:.2}", p + v);
    println!("{:+.3}", -v);
}
