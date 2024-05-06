use super::float::Float;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

macro_rules! impl_named_color {
    ($name:ident, $r:literal, $g:literal, $b:literal) => {
        impl Color {
            pub const fn $name() -> Self {
                Color {
                    r: $r,
                    g: $g,
                    b: $b,
                }
            }
        }
    };
}
impl_named_color!(black, 0.0, 0.0, 0.0);
impl_named_color!(gray, 0.5, 0.5, 0.5);
impl_named_color!(white, 1.0, 1.0, 1.0);
impl_named_color!(red, 1.0, 0.0, 0.0);
impl_named_color!(green, 0.0, 1.0, 0.0);
impl_named_color!(blue, 0.0, 0.0, 1.0);
impl_named_color!(cyan, 0.0, 1.0, 1.0);
impl_named_color!(magenta, 1.0, 0.0, 1.0);
impl_named_color!(yellow, 1.0, 1.0, 0.0);

macro_rules! impl_elementwise_op {
    ($Op:ident, $op_fn:ident, $op:tt) => {
        impl $Op<Color> for Color {
            type Output = Color;

            fn $op_fn(self, rhs: Color) -> Self::Output {
                let r = self.r $op rhs.r;
                let g = self.g $op rhs.g;
                let b = self.b $op rhs.b;
                Self::Output {r, g, b}
            }
        }
    };
}
impl_elementwise_op!(Add, add, +);
impl_elementwise_op!(Sub, sub, -);
impl_elementwise_op!(Mul, mul, *);

impl Mul<Color> for Float {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        let r = self * rhs.r;
        let g = self * rhs.g;
        let b = self * rhs.b;
        Self::Output { r, g, b }
    }
}

impl Mul<Float> for Color {
    type Output = Color;

    fn mul(self, rhs: Float) -> Self::Output {
        let r = self.r * rhs;
        let g = self.g * rhs;
        let b = self.b * rhs;
        Self::Output { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color() {
        let c = Color {
            r: -0.5,
            g: 0.4,
            b: 1.7,
        };
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn create_black() {
        let c = Color::black();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn create_white() {
        let c = Color::white();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 1.0);
    }

    #[test]
    fn create_red() {
        let c = Color::red();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn create_green() {
        let c = Color::green();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn create_blue() {
        let c = Color::blue();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 1.0);
    }

    #[test]
    fn create_cyan() {
        let c = Color::cyan();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 1.0);
    }

    #[test]
    fn create_magenta() {
        let c = Color::magenta();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 1.0);
    }

    #[test]
    fn create_yellow() {
        let c = Color::yellow();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn add_colors() {
        let lhs = Color {
            r: 0.9,
            g: 0.6,
            b: 0.75,
        };
        let rhs = Color {
            r: 0.7,
            g: 0.1,
            b: 0.25,
        };
        let res = lhs + rhs;
        assert!(approx_eq!(res.r, 1.6));
        assert!(approx_eq!(res.g, 0.7));
        assert!(approx_eq!(res.b, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let lhs = Color {
            r: 0.9,
            g: 0.6,
            b: 0.75,
        };
        let rhs = Color {
            r: 0.7,
            g: 0.1,
            b: 0.25,
        };
        let res = lhs - rhs;
        assert!(approx_eq!(res.r, 0.2));
        assert!(approx_eq!(res.g, 0.5));
        assert!(approx_eq!(res.b, 0.5));
    }

    #[test]
    fn multiply_colors() {
        let lhs = Color {
            r: 1.0,
            g: 0.2,
            b: 0.4,
        };
        let rhs = Color {
            r: 0.9,
            g: 1.0,
            b: 0.1,
        };
        let res = lhs * rhs;
        assert!(approx_eq!(res.r, 0.9));
        assert!(approx_eq!(res.g, 0.2));
        assert!(approx_eq!(res.b, 0.04));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color {
            r: -0.5,
            g: 0.4,
            b: 1.7,
        };
        let res = 2.0 * c;
        assert!(approx_eq!(res.r, -1.0));
        assert!(approx_eq!(res.g, 0.8));
        assert!(approx_eq!(res.b, 3.4));
        let res = c * 2.0;
        assert!(approx_eq!(res.r, -1.0));
        assert!(approx_eq!(res.g, 0.8));
        assert!(approx_eq!(res.b, 3.4));
    }
}
