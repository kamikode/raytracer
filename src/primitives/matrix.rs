use super::float::Float;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[Float; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(data: [[Float; N]; M]) -> Self {
        Matrix { data }
    }

    pub fn zeros() -> Self {
        Matrix {
            data: [[0.0; N]; M],
        }
    }

    pub fn ones() -> Self {
        Matrix {
            data: [[1.0; N]; M],
        }
    }

    pub fn matmul<const P: usize, T: Into<Matrix<N, P>>>(&self, rhs: T) -> Matrix<M, P> {
        matmul(*self, rhs)
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut m = Matrix {
            data: [[0.0; N]; N],
        };
        for i in 0..N {
            m.data[i][i] = 1.0;
        }
        m
    }
}

impl<const M: usize, const N: usize> Index<usize> for Matrix<M, N> {
    type Output = [Float; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const M: usize, const N: usize> IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..M {
            if i > 0 {
                f.write_str(" [")?;
            } else {
                f.write_str("[[")?;
            }
            for j in 0..N {
                fmt::Display::fmt(&self[i][j], f)?;
                if j < N - 1 {
                    f.write_str(", ")?;
                }
            }
            if i < M - 1 {
                f.write_str("]\n")?;
            } else {
                f.write_str("]]")?;
            }
        }
        Ok(())
    }
}

pub type Matrix4x4 = Matrix<4, 4>;
pub type Matrix3x3 = Matrix<3, 3>;
pub type Matrix2x2 = Matrix<2, 2>;

pub fn matmul<const M: usize, const N: usize, const P: usize, T: Into<Matrix<N, P>>>(
    lhs: Matrix<M, N>,
    rhs: T,
) -> Matrix<M, P> {
    let rhs = rhs.into();
    let mut out = Matrix::<M, P>::zeros();
    for i in 0..M {
        for j in 0..P {
            for k in 0..N {
                out[i][j] += lhs[i][k] * rhs[k][j];
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn create_matrix2x2() {
        let m = Matrix2x2::new([[0.0, 0.1], [1.0, 1.1]]);
        assert_eq!(m[0][1], 0.1);
        assert_eq!(m[1][0], 1.0);
    }

    #[test]
    fn create_matrix3x3() {
        let m = Matrix3x3::new([[0.0, 0.1, 0.2], [1.0, 1.1, 1.2], [2.0, 2.1, 2.2]]);
        assert_eq!(m[0][1], 0.1);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[2][2], 2.2);
    }

    #[test]
    fn create_matrix4x4() {
        let m = Matrix4x4::new([
            [0.0, 0.1, 0.2, 0.3],
            [1.0, 1.1, 1.2, 1.3],
            [2.0, 2.1, 2.2, 2.3],
            [3.0, 3.1, 3.2, 3.3],
        ]);
        assert_eq!(m[0][1], 0.1);
        assert_eq!(m[1][3], 1.3);
        assert_eq!(m[2][0], 2.0);
        assert_eq!(m[3][2], 3.2);
    }

    #[test]
    fn zeros_constructor() {
        let m = Matrix::<1, 2>::zeros();
        assert_eq!(m[0][0], 0.0);
        assert_eq!(m[0][1], 0.0);
    }

    #[test]
    fn ones_constructor() {
        let m = Matrix::<2, 1>::ones();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][0], 1.0);
    }

    #[test]
    fn identity_constructor() {
        let m = Matrix::<2, 2>::identity();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[1][0], 0.0);
        assert_eq!(m[1][1], 1.0);
    }

    #[test]
    fn matrix_comparison() {
        let m1 = Matrix::<2, 1>::new([[0.0], [1.0]]);
        let m2 = Matrix::<2, 1>::new([[0.0], [1.0]]);
        let m3 = Matrix::<2, 1>::new([[1.0], [0.0]]);
        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn matrix_matrix_multiplication() {
        let a = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4x4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = Matrix4x4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(a.matmul(b), c);
        assert_eq!(matmul(a, b), c);
    }

    #[test]
    fn from_vector() {
        let v = Vector::new(0.0, 1.0, 2.0);
        let m = Matrix::from(v);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [0.0]]));
    }

    #[test]
    fn from_point() {
        let p = Point::new(0.0, 1.0, 2.0);
        let m = Matrix::from(p);
        assert_eq!(m, Matrix::<4, 1>::new([[0.0], [1.0], [2.0], [1.0]]));
    }

    #[test]
    fn matrix_point_multiplication() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(
            m.matmul(p),
            Matrix::<4, 1>::new([[18.0], [24.0], [33.0], [1.0]])
        )
    }

    #[test]
    fn matrix_vector_multiplication() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(
            m.matmul(v),
            Matrix::<4, 1>::new([[14.0], [22.0], [32.0], [0.0]])
        )
    }

    #[test]
    fn matrix_multiplication_with_identity() {
        let m = Matrix2x2::new([[0.0, 1.0], [2.0, 3.0]]);
        assert_eq!(m.matmul(Matrix2x2::identity()), m)
    }

    #[test]
    fn matrix_to_string() {
        let m = Matrix::<3, 2>::new([[0.1, -1.0], [2.0, 3.09], [-4.0, 5.0]]);
        assert_eq!(m.to_string(), "[[0.1, -1]\n [2, 3.09]\n [-4, 5]]");
        assert_eq!(
            format!("{:+.2}", m),
            "[[+0.10, -1.00]\n [+2.00, +3.09]\n [-4.00, +5.00]]"
        );
    }
}
