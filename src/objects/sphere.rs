use crate::{Float, Matrix4x4, Point, Vector};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub transform: Matrix4x4,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix4x4::identity(),
        }
    }
}

impl Sphere {
    pub fn normal_at(&self, point: Point) -> Vector {
        (point - Point::origin()).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_default_transform_is_identity() {
        let sphere = Sphere::default();
        assert_eq!(sphere.transform, Matrix4x4::identity());
    }

    #[test]
    fn sphere_with_non_default_transform() {
        let transform = Matrix4x4::translation(2.0, 3.0, 4.0);
        let sphere = Sphere { transform };
        assert_eq!(sphere.transform, transform);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_x_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (1.0, 0.0, 0.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_eq!(normal, Vector { x, y, z })
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_y_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (0.0, 1.0, 0.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_eq!(normal, Vector { x, y, z })
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_z_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (0.0, 0.0, 1.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_eq!(normal, Vector { x, y, z })
    }

    #[test]
    fn normal_on_a_sphere_at_a_non_axial_point() {
        let sphere = Sphere::default();
        let val = Float::sqrt(3.0) / 3.0;
        let (x, y, z) = (val, val, val);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_eq!(normal, Vector { x, y, z })
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let sphere = Sphere::default();
        let val = Float::sqrt(3.0) / 3.0;
        let (x, y, z) = (val, val, val);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_eq!(normal, normal.normalize());
    }
}
