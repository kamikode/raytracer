use crate::{Color, Float, Point, PointLight, Vector};

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

impl Material {
    fn lighting(&self, light: PointLight, point: Point, eye: Vector, normal: Vector) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normal);
        let mut diffuse = Color::black();
        let mut specular = Color::black();
        if light_dot_normal > 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect = (-lightv).reflect(normal);
            let reflect_dot_eye = reflect.dot(eye);
            if reflect_dot_eye > 0.0 {
                let factor = Float::powf(reflect_dot_eye, self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq;

    macro_rules! assert_color_approx_eq {
        ($color1:expr, $color2:expr) => {
            assert!(approx_eq!($color1.r, $color2.r));
            assert!(approx_eq!($color1.g, $color2.g));
            assert!(approx_eq!($color1.b, $color2.b));
        };
    }

    #[test]
    fn create_default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::origin();
        let eye = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let normal = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let light = PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            intensity: Color::white(),
        };
        assert_color_approx_eq!(
            m.lighting(light, position, eye, normal),
            Color {
                r: 1.9,
                g: 1.9,
                b: 1.9,
            }
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degree() {
        let m = Material::default();
        let position = Point::origin();
        let eye = Vector {
            x: 0.0,
            y: std::f64::consts::FRAC_1_SQRT_2 as Float,
            z: -std::f64::consts::FRAC_1_SQRT_2 as Float,
        };
        let normal = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let light = PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            intensity: Color::white(),
        };
        assert_color_approx_eq!(
            m.lighting(light, position, eye, normal),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_and_light_offset_45_degree() {
        let m = Material::default();
        let position = Point::origin();
        let eye = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let normal = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let light = PointLight {
            position: Point {
                x: 0.0,
                y: 10.0,
                z: -10.0,
            },
            intensity: Color::white(),
        };
        assert_color_approx_eq!(
            m.lighting(light, position, eye, normal),
            Color {
                r: 0.7364,
                g: 0.7364,
                b: 0.7364,
            }
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Point::origin();
        let eye = Vector {
            x: 0.0,
            y: -std::f64::consts::FRAC_1_SQRT_2 as Float,
            z: -std::f64::consts::FRAC_1_SQRT_2 as Float,
        };
        let normal = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let light = PointLight {
            position: Point {
                x: 0.0,
                y: 10.0,
                z: -10.0,
            },
            intensity: Color::white(),
        };
        assert_color_approx_eq!(
            m.lighting(light, position, eye, normal),
            Color {
                r: 1.6364,
                g: 1.6364,
                b: 1.6364,
            }
        );
    }

    #[test]
    fn lighting_with_eye_behind_the_surface() {
        let m = Material::default();
        let position = Point::origin();
        let eye = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let normal = Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let light = PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            intensity: Color::white(),
        };
        assert_color_approx_eq!(
            m.lighting(light, position, eye, normal),
            Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
            }
        );
    }
}
