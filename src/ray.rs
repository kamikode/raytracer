use crate::primitives::float::Float;
use crate::Sphere;
use crate::{Intersection, Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position(&self, t: Float) -> Point {
        self.origin + t * self.direction
    }

    // TODO: Later this function should work with more things than spheres.
    pub fn intersect(&self, object: Sphere) -> Vec<Intersection> {
        // Note: Spheres are hardcoded to be at the origin for now.
        let sphere_to_ray = self.origin - Point::new(0.0, 0.0, 0.0);
        let a = self.direction.squared_length();
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.squared_length() - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let sqrt = Float::sqrt(discriminant);
            let div = 1.0 / (2.0 * a);
            vec![
                Intersection {
                    t: (-b - sqrt) * div,
                    object,
                },
                Intersection {
                    t: (-b + sqrt) * div,
                    object,
                },
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let ray = Ray { origin, direction };
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_position_along_ray() {
        let ray = Ray {
            origin: Point::new(2.0, 3.0, 4.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };
        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().object, sphere);
        assert_eq!(intersections.last().unwrap().object, sphere);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let ts = ray.intersect(sphere);
        assert_eq!(ts.len(), 2);
        assert_eq!(ts.first().unwrap().t, 4.0);
        assert_eq!(ts.last().unwrap().t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let ts = ray.intersect(sphere);
        assert_eq!(ts.len(), 2);
        assert_eq!(ts.first().unwrap().t, 5.0);
        assert_eq!(ts.last().unwrap().t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let ts = ray.intersect(sphere);
        assert_eq!(ts.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let ts = ray.intersect(sphere);
        assert_eq!(ts.len(), 2);
        assert_eq!(ts.first().unwrap().t, -1.0);
        assert_eq!(ts.last().unwrap().t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let ts = ray.intersect(sphere);
        assert_eq!(ts.len(), 2);
        assert_eq!(ts.first().unwrap().t, -6.0);
        assert_eq!(ts.last().unwrap().t, -4.0);
    }
}
