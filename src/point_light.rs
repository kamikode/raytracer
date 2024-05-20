use crate::{Color, Point};

#[derive(Debug)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point_light() {
        let position = Point::origin();
        let intensity = Color::white();
        let light = PointLight {
            position,
            intensity,
        };
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
