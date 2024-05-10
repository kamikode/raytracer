use super::float::Float;
use crate::Matrix;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

trait Tuple {}

macro_rules! impl_tuple {
    ($name:ident, $w:literal) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct $name {
            x: Float,
            y: Float,
            z: Float,
            w: Float,
        }

        impl $name {
            pub fn new(x: Float, y: Float, z: Float) -> Self {
                Self { x, y, z, w: $w }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("[")?;
                fmt::Display::fmt(&self.x, f)?;
                f.write_str(", ")?;
                fmt::Display::fmt(&self.y, f)?;
                f.write_str(", ")?;
                fmt::Display::fmt(&self.z, f)?;
                f.write_str(", ")?;
                fmt::Display::fmt(&self.w, f)?;
                f.write_str("]")
            }
        }

        impl From<$name> for Matrix<4, 1> {
            fn from(value: $name) -> Self {
                Matrix::<4, 1>::new([[value.x], [value.y], [value.z], [$w]])
            }
        }

        impl Tuple for $name {}
    };
}
impl_tuple!(Point, 1.0);
impl_tuple!(Vector, 0.0);

macro_rules! impl_add {
    ($Lhs:ident, $Rhs:ident, $Out:ident) => {
        impl Add<$Rhs> for $Lhs {
            type Output = $Out;

            fn add(self, rhs: $Rhs) -> Self::Output {
                Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }
    };
}
impl_add!(Point, Vector, Point);
impl_add!(Vector, Point, Point);
impl_add!(Vector, Vector, Vector);

macro_rules! impl_sub {
    ($Lhs:ty, $Rhs:ty, $Out:ty) => {
        impl Sub<$Rhs> for $Lhs {
            type Output = $Out;

            fn sub(self, rhs: $Rhs) -> Self::Output {
                Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }
    };
}
impl_sub!(Point, Point, Vector);
impl_sub!(Point, Vector, Point);
impl_sub!(Vector, Vector, Vector);

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<Float> for Vector {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float> for Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub fn length(v: Vector) -> Float {
    dot(v, v).sqrt()
}

pub fn normalize(v: Vector) -> Vector {
    v / length(v)
}

pub fn dot(a: Vector, b: Vector) -> Float {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    let x = a.y * b.z - a.z * b.y;
    let y = a.z * b.x - a.x * b.z;
    let z = a.x * b.y - a.y * b.x;
    Vector::new(x, y, z)
}

impl Vector {
    pub fn length(&self) -> Float {
        length(*self)
    }

    pub fn normalize(&self) -> Vector {
        normalize(*self)
    }

    pub fn dot(&self, rhs: Vector) -> Float {
        dot(*self, rhs)
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        cross(*self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($tuple1:expr, $tuple2:expr) => {
            assert!(approx_eq!($tuple1.x, $tuple2.x));
            assert!(approx_eq!($tuple1.y, $tuple2.y));
            assert!(approx_eq!($tuple1.z, $tuple2.z));
            assert!(approx_eq!($tuple1.w, $tuple2.w));
        };
    }

    #[test]
    fn create_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn create_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn point_to_string() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.to_string(), "[4.3, -4.2, 3.1, 1]");
        assert_eq!(format!("{:+.2}", p), "[+4.30, -4.20, +3.10, +1.00]");
    }

    #[test]
    fn vector_to_string() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.to_string(), "[4.3, -4.2, 3.1, 0]");
        assert_eq!(format!("{:+.2}", v), "[+4.30, -4.20, +3.10, +0.00]");
    }

    #[test]
    fn add_vector_to_point() {
        let p = Point::new(3.0, -4.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Point::new(1.0, -1.0, 6.0));
    }

    #[test]
    fn add_point_to_vector() {
        let v = Vector::new(-2.0, 3.0, 1.0);
        let p = Point::new(3.0, -4.0, 5.0);
        assert_eq!(v + p, Point::new(1.0, -1.0, 6.0));
    }

    #[test]
    fn add_vector_and_vector() {
        let a = Vector::new(3.0, -4.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(a + b, Vector::new(1.0, -1.0, 6.0));
    }

    #[test]
    fn subtract_point_from_point() {
        let a = Point::new(3.0, 2.0, 7.0);
        let b = Point::new(5.0, 6.0, 1.0);
        assert_eq!(a - b, Vector::new(-2.0, -4.0, 6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 7.0);
        let v = Vector::new(5.0, 6.0, 1.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, 6.0));
    }

    #[test]
    fn subtract_vector_from_vector() {
        let a = Vector::new(3.0, 2.0, 7.0);
        let b = Vector::new(5.0, 6.0, 1.0);
        assert_eq!(a - b, Vector::new(-2.0, -4.0, 6.0));
    }

    #[test]
    fn negate_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scalar_multiplication_of_vector() {
        let v = Vector::new(1.0, -2.0, 4.0);
        assert_eq!(2.0 * v, Vector::new(2.0, -4.0, 8.0));
        assert_eq!(v * 2.0, Vector::new(2.0, -4.0, 8.0));
        assert_eq!(0.5 * v, Vector::new(0.5, -1.0, 2.0));
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 2.0));
    }

    #[test]
    fn scalar_division_of_vector() {
        let v = Vector::new(1.0, -2.0, 4.0);
        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 2.0));
        assert_eq!(v / 0.5, Vector::new(2.0, -4.0, 8.0));
    }

    #[test]
    fn dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(dot(a, b), 20.0);
        assert_eq!(dot(b, a), 20.0);
        assert_eq!(a.dot(b), 20.0);
        assert_eq!(b.dot(a), 20.0);
    }

    #[test]
    fn length_calculation() {
        let x = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(length(x), 1.0);
        assert_eq!(x.length(), 1.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(length(y), 1.0);
        assert_eq!(y.length(), 1.0);
        let z = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(length(z), 1.0);
        assert_eq!(z.length(), 1.0);
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(length(a), Float::sqrt(14.0));
        assert_eq!(a.length(), Float::sqrt(14.0));
        let b = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(length(b), Float::sqrt(14.0));
        assert_eq!(b.length(), Float::sqrt(14.0));
    }

    #[test]
    fn normalization() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let u = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(normalize(v), u);
        assert_eq!(v.normalize(), u);
        let v = Vector::new(1.0, 2.0, 3.0);
        let u = v.normalize();
        let norm: Float = Float::sqrt(14.0);
        assert_approx_eq!(u, Vector::new(1.0 / norm, 2.0 / norm, 3.0 / norm));
    }

    #[test]
    fn cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let axb = Vector::new(-1.0, 2.0, -1.0);
        let bxa = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(cross(a, b), axb);
        assert_eq!(cross(b, a), bxa);
        assert_eq!(a.cross(b), axb);
        assert_eq!(b.cross(a), bxa);
    }
}
