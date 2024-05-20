use crate::primitives::float::Float;
use crate::{Intersection, Invertible, Matrix4x4, Point, Sphere, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position(&self, t: Float) -> Point {
        self.origin + t * self.direction
    }

    pub fn transform(&self, transform: Matrix4x4) -> Ray {
        Ray {
            origin: transform
                .matmul(self.origin)
                .try_into()
                .expect("origin should be convertible into Point after applying transform"),
            direction: transform
                .matmul(self.direction)
                .try_into()
                .expect("direction should be convertible into Vector after applying transform"),
        }
    }

    // TODO: Later this function should work with more things than spheres.
    pub fn intersect(&self, object: Sphere) -> Vec<Intersection> {
        let inverse_transform = match object.transform.inverse() {
            Some(m) => m,
            None => return vec![],
        };
        let ray = self.transform(inverse_transform);
        let sphere_to_ray = ray.origin - Point::origin();
        let a = ray.direction.squared_length();
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
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

pub fn get_hit(intersections: &[Intersection]) -> Option<Intersection> {
    let mut hit = None;
    let mut min_t = Float::INFINITY;
    for intersection in intersections {
        if intersection.t > 0.0 && intersection.t < min_t {
            hit = Some(*intersection);
            min_t = intersection.t;
        }
    }
    hit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Material;

    #[test]
    fn create_ray() {
        let origin = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let direction = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let ray = Ray { origin, direction };
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_position_along_ray() {
        let ray = Ray {
            origin: Point {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            direction: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        assert_eq!(
            ray.position(0.0),
            Point {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
        assert_eq!(
            ray.position(1.0),
            Point {
                x: 3.0,
                y: 3.0,
                z: 4.0
            }
        );
        assert_eq!(
            ray.position(-1.0),
            Point {
                x: 1.0,
                y: 3.0,
                z: 4.0
            }
        );
        assert_eq!(
            ray.position(2.5),
            Point {
                x: 4.5,
                y: 3.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn translate_ray() {
        let r = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let m = Matrix4x4::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(
            r2.origin,
            Point {
                x: 4.0,
                y: 6.0,
                z: 8.0
            }
        );
        assert_eq!(
            r2.direction,
            Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn scale_ray() {
        let r = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let m = Matrix4x4::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(
            r2.origin,
            Point {
                x: 2.0,
                y: 6.0,
                z: 12.0
            }
        );
        assert_eq!(
            r2.direction,
            Vector {
                x: 0.0,
                y: 3.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().object, sphere);
        assert_eq!(intersections.last().unwrap().object, sphere);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, 4.0);
        assert_eq!(intersections.last().unwrap().t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 1.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, 5.0);
        assert_eq!(intersections.last().unwrap().t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 2.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, -1.0);
        assert_eq!(intersections.last().unwrap().t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::default();
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, -6.0);
        assert_eq!(intersections.last().unwrap().t, -4.0);
    }

    #[test]
    fn ray_intersects_scaled_sphere() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere {
            transform: Matrix4x4::scaling(2.0, 2.0, 2.0),
            material: Material::default(),
        };
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, 3.0);
        assert_eq!(intersections.last().unwrap().t, 7.0);
    }

    #[test]
    fn ray_intersects_translated_sphere() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere {
            transform: Matrix4x4::translation(5.0, 0.0, 0.0),
            material: Material::default(),
        };
        let intersections = ray.intersect(sphere);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn get_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: 1.0, object: s };
        let i2 = Intersection { t: 2.0, object: s };
        let xs = vec![i1, i2];
        let i = get_hit(&xs);
        assert_eq!(i, Some(i1));
    }

    #[test]
    fn get_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: -1.0, object: s };
        let i2 = Intersection { t: 1.0, object: s };
        let xs = vec![i1, i2];
        let i = get_hit(&xs);
        assert_eq!(i, Some(i2));
    }

    #[test]
    fn get_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: -2.0, object: s };
        let i2 = Intersection { t: -1.0, object: s };
        let xs = vec![i1, i2];
        let i = get_hit(&xs);
        assert_eq!(i, None);
    }

    #[test]
    fn get_hit_is_always_lowest_non_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: 5.0, object: s };
        let i2 = Intersection { t: 7.0, object: s };
        let i3 = Intersection { t: -3.0, object: s };
        let i4 = Intersection { t: 2.0, object: s };
        let xs = vec![i1, i2, i3, i4];
        let i = get_hit(&xs);
        assert_eq!(i, Some(i4));
    }
}
