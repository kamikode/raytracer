use crate::{Invertible, Material, Matrix4x4, Point, Vector};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub transform: Matrix4x4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix4x4::identity(),
            material: Material::default(),
        }
    }
}

impl Sphere {
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let inv_transform = self
            .transform
            .inverse()
            .expect("transform should be invertible");

        let object_point = inv_transform.matmul(world_point);
        let mut object_normal = object_point;
        object_normal.data[3][0] = 0.0;
        let mut world_normal = inv_transform.transpose().matmul(object_normal);
        world_normal.data[3][0] = 0.0;
        Vector::try_from(world_normal)
            .expect("should be convertible to Vector")
            .normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, Float};
    use std::iter::zip;

    macro_rules! assert_matrix_approx_eq {
        ($mat1:expr, $mat2:expr) => {
            for (row1, row2) in zip($mat1.data, $mat2.data) {
                for (x1, x2) in zip(row1, row2) {
                    assert!(approx_eq!(x1, x2));
                }
            }
        };
    }

    macro_rules! assert_tuple_approx_eq {
        ($tuple1:expr, $tuple2:expr) => {
            assert!(approx_eq!($tuple1.x, $tuple2.x));
            assert!(approx_eq!($tuple1.y, $tuple2.y));
            assert!(approx_eq!($tuple1.z, $tuple2.z));
        };
    }

    #[test]
    fn sphere_default_constructor() {
        let sphere = Sphere::default();
        assert_eq!(sphere.transform, Matrix4x4::identity());
        assert_eq!(sphere.material, Material::default());
    }

    #[test]
    fn sphere_with_non_default_transform() {
        let transform = Matrix4x4::translation(2.0, 3.0, 4.0);
        let material = Material::default();
        let sphere = Sphere {
            transform,
            material,
        };
        assert_matrix_approx_eq!(sphere.transform, transform);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_x_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (1.0, 0.0, 0.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_tuple_approx_eq!(normal, Vector { x, y, z });
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_y_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (0.0, 1.0, 0.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_tuple_approx_eq!(normal, Vector { x, y, z });
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_z_axis() {
        let sphere = Sphere::default();
        let (x, y, z) = (0.0, 0.0, 1.0);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_tuple_approx_eq!(normal, Vector { x, y, z });
    }

    #[test]
    fn normal_on_a_sphere_at_a_non_axial_point() {
        let sphere = Sphere::default();
        let val = Float::sqrt(3.0) / 3.0;
        let (x, y, z) = (val, val, val);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_tuple_approx_eq!(normal, Vector { x, y, z });
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let sphere = Sphere::default();
        let val = Float::sqrt(3.0) / 3.0;
        let (x, y, z) = (val, val, val);
        let normal = sphere.normal_at(Point { x, y, z });
        assert_tuple_approx_eq!(normal, normal.normalize());
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let sphere = Sphere {
            transform: Matrix4x4::translation(0.0, 1.0, 0.0),
            material: Material::default(),
        };
        let normal = sphere.normal_at(Point {
            x: 0.0,
            y: 1.0 + std::f64::consts::FRAC_1_SQRT_2 as Float,
            z: -std::f64::consts::FRAC_1_SQRT_2 as Float,
        });
        assert_tuple_approx_eq!(
            normal,
            Vector {
                x: 0.0,
                y: std::f64::consts::FRAC_1_SQRT_2 as Float,
                z: -std::f64::consts::FRAC_1_SQRT_2 as Float
            }
        );
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let sphere = Sphere {
            transform: Matrix4x4::scaling(1.0, 0.5, 1.0)
                .matmul(Matrix4x4::rotation_z(std::f64::consts::PI as Float / 5.0)),
            material: Material::default(),
        };
        let normal = sphere.normal_at(Point {
            x: 0.0,
            y: std::f64::consts::FRAC_1_SQRT_2 as Float,
            z: -std::f64::consts::FRAC_1_SQRT_2 as Float,
        });
        assert_tuple_approx_eq!(
            normal,
            Vector {
                x: 0.0,
                y: 0.9701425001453319,
                z: -0.24253562503633294,
            }
        );
    }
}
