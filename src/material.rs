use crate::{Color, Float};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Color,
    ambient: Float,
    diffuse: Float,
    specular: Float,
    shininess: Float,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
