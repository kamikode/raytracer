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
        }

        impl $name {
            pub fn new(x: Float, y: Float, z: Float) -> Self {
                Self { x, y, z }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(stringify!($name))?;
                f.write_str(" [")?;
                fmt::Display::fmt(&self.x, f)?;
                f.write_str(", ")?;
                fmt::Display::fmt(&self.y, f)?;
                f.write_str(", ")?;
                fmt::Display::fmt(&self.z, f)?;
                f.write_str("]")
            }
        }

        impl From<$name> for Matrix<4, 1> {
            fn from(value: $name) -> Self {
                Matrix::<4, 1>::new([[value.x], [value.y], [value.z], [$w]])
            }
        }

        impl TryFrom<Matrix<4, 1>> for $name {
            type Error = String;

            fn try_from(value: Matrix<4, 1>) -> Result<Self, Self::Error> {
                if value[3][0] == $w {
                    let x = value[0][0];
                    let y = value[1][0];
                    let z = value[2][0];
                    Ok($name { x, y, z })
                } else {
                    Err(format!(
                        "cannot convert matrix with entries x={}, y={}, z={}, w={} to {}, {}s must have w={}",
                        value[0][0],
                        value[1][0],
                        value[2][0],
                        value[3][0],
                        stringify!($name),
                        stringify!($name),
                        $w
                    ))
                }
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

impl Vector {
    pub fn length(&self) -> Float {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.length()
    }

    pub fn dot(&self, rhs: Vector) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
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
        };
    }

    #[test]
    fn create_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }

    #[test]
    fn create_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
    }

    #[test]
    fn matrix_from_point() {
        let p = Point::new(0.0, 1.0, 2.0);
        let m = Matrix::from(p);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]]));
    }

    #[test]
    fn matrix_from_vector() {
        let v = Vector::new(0.0, 1.0, 2.0);
        let m = Matrix::from(v);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]]));
    }

    #[test]
    fn point_from_matrix() {
        let m = Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]]);
        let p = Point::try_from(m).unwrap();
        assert_eq!(p, Point::new(0.0, 1.0, 2.0));
        assert!(Point::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]])).is_err());
        assert!(Point::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.5]])).is_err());
    }

    #[test]
    fn vector_from_matrix() {
        let m = Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]]);
        let v = Vector::try_from(m).unwrap();
        assert_eq!(v, Vector::new(0.0, 1.0, 2.0));
        assert!(Vector::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]])).is_err());
        assert!(Vector::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.5]])).is_err());
    }

    #[test]
    fn point_to_string() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.to_string(), "Point [4.3, -4.2, 3.1]");
        assert_eq!(format!("{:+.2}", p), "Point [+4.30, -4.20, +3.10]");
    }

    #[test]
    fn vector_to_string() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.to_string(), "Vector [4.3, -4.2, 3.1]");
        assert_eq!(format!("{:+.2}", v), "Vector [+4.30, -4.20, +3.10]");
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
        assert_eq!(a.dot(b), 20.0);
        assert_eq!(b.dot(a), 20.0);
    }

    #[test]
    fn length_calculation() {
        let x = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(x.length(), 1.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(y.length(), 1.0);
        let z = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(z.length(), 1.0);
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a.length(), Float::sqrt(14.0));
        let b = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(b.length(), Float::sqrt(14.0));
    }

    #[test]
    fn normalization() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let u = Vector::new(1.0, 0.0, 0.0);
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
        assert_eq!(a.cross(b), axb);
        assert_eq!(b.cross(a), bxa);
    }
}
