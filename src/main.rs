use raytracer::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Simple program to render a sphere (or rather circle).
    // Make a square canvas for simplicity.
    let mut canvas = Canvas::<256, 256>::new();
    assert_eq!(canvas.width(), canvas.height());

    // All rays are shot from the origin.
    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
    };

    // Simple unit sphere.
    let sphere = Sphere {
        material: Material {
            color: Color {
                r: 1.0,
                g: 0.2,
                b: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // Point light.
    let light = PointLight {
        position: Point {
            x: -10.0,
            y: 10.0,
            z: -10.0,
        },
        intensity: Color::white(),
    };

    // Parameters for the wall.
    let wall_z: Float = 10.0;
    let wall_size: Float = 7.0;
    let wall_half_size: Float = wall_size / 2.0;

    for x in 0..canvas.width() {
        let frac = (x as Float) / (canvas.width() as Float);
        let wall_x = frac * wall_size - wall_half_size;
        for y in 0..canvas.height() {
            let frac = (y as Float) / (canvas.height() as Float);
            let wall_y = -(frac * wall_size - wall_half_size);
            let wall_point = Point {
                x: wall_x,
                y: wall_y,
                z: wall_z,
            };
            let ray = Ray {
                origin,
                direction: (wall_point - origin).normalize(),
            };
            let intersections = ray.intersect(sphere);
            let hit = get_hit(&intersections);

            if hit.is_some() {
                let hit = hit.unwrap();
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;
                let color = hit.object.material.lighting(light, point, eye, normal);
                canvas.set_pixel(x, y, color)?;
            }
        }
    }
    let mut file = File::create("test.ppm")?;
    canvas.write_ppm(&mut file)?;
    Ok(())
}
