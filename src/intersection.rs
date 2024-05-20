use crate::{primitives::float::Float, Sphere};

#[derive(Debug)]
pub struct Intersection {
    pub t: Float,
    pub object: Sphere,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_intersection() {
        let sphere = Sphere {};
        let intersection = Intersection {
            t: 3.5,
            object: sphere,
        };
        assert_eq!(intersection.t, 3.5);
        assert_eq!(intersection.object, sphere);
    }

    #[test]
    fn aggregate_intersections() {
        let sphere = Sphere {};
        let i1 = Intersection {
            t: 1.0,
            object: sphere,
        };
        let i2 = Intersection {
            t: 2.0,
            object: sphere,
        };
        let intersections = [i1, i2];
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.first().unwrap().t, 1.0);
        assert_eq!(intersections.last().unwrap().t, 2.0);
    }
}
