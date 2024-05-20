use crate::Float;
use crate::Matrix;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

trait Tuple {}

macro_rules! impl_tuple {
    ($name:ident, $w:literal) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct $name {
            pub x: Float,
            pub y: Float,
            pub z: Float,
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

impl Point {
    pub fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

macro_rules! impl_add {
    ($Lhs:ident, $Rhs:ident, $Out:ident) => {
        impl Add<$Rhs> for $Lhs {
            type Output = $Out;

            fn add(self, rhs: $Rhs) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
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
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
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
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Float> for Vector {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Float> for Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector {
    pub fn squared_length(&self) -> Float {
        self.dot(*self)
    }

    pub fn length(&self) -> Float {
        self.squared_length().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.length()
    }

    pub fn dot(&self, rhs: Vector) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
        let p = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
        };
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }

    #[test]
    fn create_origin() {
        let p = Point::origin();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn create_vector() {
        let v = Vector {
            x: 4.3,
            y: -4.2,
            z: 3.1,
        };
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
    }

    #[test]
    fn matrix_from_point() {
        let p = Point {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        };
        let m = Matrix::from(p);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]]));
    }

    #[test]
    fn matrix_from_vector() {
        let v = Vector {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        };
        let m = Matrix::from(v);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]]));
    }

    #[test]
    fn point_from_matrix() {
        let m = Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]]);
        let p = Point::try_from(m).unwrap();
        assert_eq!(
            p,
            Point {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            }
        );
        assert!(Point::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]])).is_err());
        assert!(Point::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.5]])).is_err());
    }

    #[test]
    fn vector_from_matrix() {
        let m = Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]]);
        let v = Vector::try_from(m).unwrap();
        assert_eq!(
            v,
            Vector {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            }
        );
        assert!(Vector::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]])).is_err());
        assert!(Vector::try_from(Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.5]])).is_err());
    }

    #[test]
    fn point_to_string() {
        let p = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
        };
        assert_eq!(p.to_string(), "Point [4.3, -4.2, 3.1]");
        assert_eq!(format!("{:+.2}", p), "Point [+4.30, -4.20, +3.10]");
    }

    #[test]
    fn vector_to_string() {
        let v = Vector {
            x: 4.3,
            y: -4.2,
            z: 3.1,
        };
        assert_eq!(v.to_string(), "Vector [4.3, -4.2, 3.1]");
        assert_eq!(format!("{:+.2}", v), "Vector [+4.30, -4.20, +3.10]");
    }

    #[test]
    fn add_vector_to_point() {
        let p = Point {
            x: 3.0,
            y: -4.0,
            z: 5.0,
        };
        let v = Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        };
        assert_eq!(
            p + v,
            Point {
                x: 1.0,
                y: -1.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn add_point_to_vector() {
        let v = Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        };
        let p = Point {
            x: 3.0,
            y: -4.0,
            z: 5.0,
        };
        assert_eq!(
            v + p,
            Point {
                x: 1.0,
                y: -1.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn add_vector_and_vector() {
        let a = Vector {
            x: 3.0,
            y: -4.0,
            z: 5.0,
        };
        let b = Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        };
        assert_eq!(
            a + b,
            Vector {
                x: 1.0,
                y: -1.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn subtract_point_from_point() {
        let a = Point {
            x: 3.0,
            y: 2.0,
            z: 7.0,
        };
        let b = Point {
            x: 5.0,
            y: 6.0,
            z: 1.0,
        };
        assert_eq!(
            a - b,
            Vector {
                x: -2.0,
                y: -4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point {
            x: 3.0,
            y: 2.0,
            z: 7.0,
        };
        let v = Vector {
            x: 5.0,
            y: 6.0,
            z: 1.0,
        };
        assert_eq!(
            p - v,
            Point {
                x: -2.0,
                y: -4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn subtract_vector_from_vector() {
        let a = Vector {
            x: 3.0,
            y: 2.0,
            z: 7.0,
        };
        let b = Vector {
            x: 5.0,
            y: 6.0,
            z: 1.0,
        };
        assert_eq!(
            a - b,
            Vector {
                x: -2.0,
                y: -4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn negate_vector() {
        let v = Vector {
            x: 1.0,
            y: -2.0,
            z: 3.0,
        };
        assert_eq!(
            -v,
            Vector {
                x: -1.0,
                y: 2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn scalar_multiplication_of_vector() {
        let v = Vector {
            x: 1.0,
            y: -2.0,
            z: 4.0,
        };
        assert_eq!(
            2.0 * v,
            Vector {
                x: 2.0,
                y: -4.0,
                z: 8.0
            }
        );
        assert_eq!(
            v * 2.0,
            Vector {
                x: 2.0,
                y: -4.0,
                z: 8.0
            }
        );
        assert_eq!(
            0.5 * v,
            Vector {
                x: 0.5,
                y: -1.0,
                z: 2.0
            }
        );
        assert_eq!(
            v * 0.5,
            Vector {
                x: 0.5,
                y: -1.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn scalar_division_of_vector() {
        let v = Vector {
            x: 1.0,
            y: -2.0,
            z: 4.0,
        };
        assert_eq!(
            v / 2.0,
            Vector {
                x: 0.5,
                y: -1.0,
                z: 2.0
            }
        );
        assert_eq!(
            v / 0.5,
            Vector {
                x: 2.0,
                y: -4.0,
                z: 8.0
            }
        );
    }

    #[test]
    fn dot_product() {
        let a = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(a.dot(b), 20.0);
        assert_eq!(b.dot(a), 20.0);
    }

    #[test]
    fn squared_length_calculation() {
        let x = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(x.squared_length(), 1.0);
        let y = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(y.squared_length(), 1.0);
        let z = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        assert_eq!(z.squared_length(), 1.0);
        let a = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.squared_length(), 14.0);
        let b = Vector {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        };
        assert_eq!(b.squared_length(), 14.0);
    }

    #[test]
    fn length_calculation() {
        let x = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(x.length(), 1.0);
        let y = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(y.length(), 1.0);
        let z = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        assert_eq!(z.length(), 1.0);
        let a = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.length(), Float::sqrt(14.0));
        let b = Vector {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        };
        assert_eq!(b.length(), Float::sqrt(14.0));
    }

    #[test]
    fn normalization() {
        let v = Vector {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };
        let u = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(v.normalize(), u);
        let v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let u = v.normalize();
        let norm: Float = Float::sqrt(14.0);
        assert_approx_eq!(
            u,
            Vector {
                x: 1.0 / norm,
                y: 2.0 / norm,
                z: 3.0 / norm
            }
        );
    }

    #[test]
    fn cross_product() {
        let a = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let axb = Vector {
            x: -1.0,
            y: 2.0,
            z: -1.0,
        };
        let bxa = Vector {
            x: 1.0,
            y: -2.0,
            z: 1.0,
        };
        assert_eq!(a.cross(b), axb);
        assert_eq!(b.cross(a), bxa);
    }
}
