use crate::Matrix4x4;

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
}
