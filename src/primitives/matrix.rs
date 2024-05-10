use super::float::Float;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[Float; N]; M],
}

pub type Matrix4x4 = Matrix<4, 4>;
pub type Matrix3x3 = Matrix<3, 3>;
pub type Matrix2x2 = Matrix<2, 2>;

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
        let rhs = rhs.into();
        let mut out = Matrix::<M, P>::zeros();
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    out[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        out
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut t = Matrix::<N, M>::zeros();
        for i in 0..M {
            for j in 0..N {
                t[j][i] = self[i][j];
            }
        }
        t
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

pub trait Invertible
where
    Self: Sized,
{
    fn determinant(&self) -> Float;
    fn inverse(&self) -> Option<Self>;
}

impl Invertible for Matrix2x2 {
    fn determinant(&self) -> Float {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    fn inverse(&self) -> Option<Self> {
        let inv_det = 1.0 / self.determinant();
        if inv_det.is_finite() {
            Some(Self::new([
                [self[1][1] * inv_det, -self[0][1] * inv_det],
                [-self[1][0] * inv_det, self[0][0] * inv_det],
            ]))
        } else {
            None
        }
    }
}

impl Invertible for Matrix3x3 {
    fn determinant(&self) -> Float {
        self[0][0] * self[1][1] * self[2][2]
            - self[0][0] * self[1][2] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            + self[0][1] * self[1][2] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][2] * self[1][1] * self[2][0]
    }

    fn inverse(&self) -> Option<Self> {
        let inv_det = 1.0 / self.determinant();
        if inv_det.is_finite() {
            Some(Matrix3x3::new([
                [
                    (self[1][1] * self[2][2] - self[1][2] * self[2][1]) * inv_det,
                    (-self[0][1] * self[2][2] + self[0][2] * self[2][1]) * inv_det,
                    (self[0][1] * self[1][2] - self[0][2] * self[1][1]) * inv_det,
                ],
                [
                    (-self[1][0] * self[2][2] + self[1][2] * self[2][0]) * inv_det,
                    (self[0][0] * self[2][2] - self[0][2] * self[2][0]) * inv_det,
                    (-self[0][0] * self[1][2] + self[0][2] * self[1][0]) * inv_det,
                ],
                [
                    (self[1][0] * self[2][1] - self[1][1] * self[2][0]) * inv_det,
                    (-self[0][0] * self[2][1] + self[0][1] * self[2][0]) * inv_det,
                    (self[0][0] * self[1][1] - self[0][1] * self[1][0]) * inv_det,
                ],
            ]))
        } else {
            None
        }
    }
}

impl Invertible for Matrix4x4 {
    fn determinant(&self) -> Float {
        let t2323 = self[2][2] * self[3][3] - self[2][3] * self[3][2];
        let t1323 = self[2][1] * self[3][3] - self[2][3] * self[3][1];
        let t1223 = self[2][1] * self[3][2] - self[2][2] * self[3][1];
        let t0323 = self[2][0] * self[3][3] - self[2][3] * self[3][0];
        let t0223 = self[2][0] * self[3][2] - self[2][2] * self[3][0];
        let t0123 = self[2][0] * self[3][1] - self[2][1] * self[3][0];
        self[0][0] * (self[1][1] * t2323 - self[1][2] * t1323 + self[1][3] * t1223)
            - self[0][1] * (self[1][0] * t2323 - self[1][2] * t0323 + self[1][3] * t0223)
            + self[0][2] * (self[1][0] * t1323 - self[1][1] * t0323 + self[1][3] * t0123)
            - self[0][3] * (self[1][0] * t1223 - self[1][1] * t0223 + self[1][2] * t0123)
    }

    fn inverse(&self) -> Option<Self> {
        let inv_det = 1.0 / self.determinant();
        if inv_det.is_finite() {
            let t2323 = self[2][2] * self[3][3] - self[2][3] * self[3][2];
            let t1323 = self[2][1] * self[3][3] - self[2][3] * self[3][1];
            let t1223 = self[2][1] * self[3][2] - self[2][2] * self[3][1];
            let t0323 = self[2][0] * self[3][3] - self[2][3] * self[3][0];
            let t0223 = self[2][0] * self[3][2] - self[2][2] * self[3][0];
            let t0123 = self[2][0] * self[3][1] - self[2][1] * self[3][0];
            let t2313 = self[1][2] * self[3][3] - self[1][3] * self[3][2];
            let t1313 = self[1][1] * self[3][3] - self[1][3] * self[3][1];
            let t1213 = self[1][1] * self[3][2] - self[1][2] * self[3][1];
            let t2312 = self[1][2] * self[2][3] - self[1][3] * self[2][2];
            let t1312 = self[1][1] * self[2][3] - self[1][3] * self[2][1];
            let t1212 = self[1][1] * self[2][2] - self[1][2] * self[2][1];
            let t0313 = self[1][0] * self[3][3] - self[1][3] * self[3][0];
            let t0213 = self[1][0] * self[3][2] - self[1][2] * self[3][0];
            let t0312 = self[1][0] * self[2][3] - self[1][3] * self[2][0];
            let t0212 = self[1][0] * self[2][2] - self[1][2] * self[2][0];
            let t0113 = self[1][0] * self[3][1] - self[1][1] * self[3][0];
            let t0112 = self[1][0] * self[2][1] - self[1][1] * self[2][0];
            Some(Matrix4x4::new([
                [
                    inv_det * (self[1][1] * t2323 - self[1][2] * t1323 + self[1][3] * t1223),
                    inv_det * (self[0][2] * t1323 - self[0][1] * t2323 - self[0][3] * t1223),
                    inv_det * (self[0][1] * t2313 - self[0][2] * t1313 + self[0][3] * t1213),
                    inv_det * (self[0][2] * t1312 - self[0][1] * t2312 - self[0][3] * t1212),
                ],
                [
                    inv_det * (self[1][2] * t0323 - self[1][0] * t2323 - self[1][3] * t0223),
                    inv_det * (self[0][0] * t2323 - self[0][2] * t0323 + self[0][3] * t0223),
                    inv_det * (self[0][2] * t0313 - self[0][0] * t2313 - self[0][3] * t0213),
                    inv_det * (self[0][0] * t2312 - self[0][2] * t0312 + self[0][3] * t0212),
                ],
                [
                    inv_det * (self[1][0] * t1323 - self[1][1] * t0323 + self[1][3] * t0123),
                    inv_det * (self[0][1] * t0323 - self[0][0] * t1323 - self[0][3] * t0123),
                    inv_det * (self[0][0] * t1313 - self[0][1] * t0313 + self[0][3] * t0113),
                    inv_det * (self[0][1] * t0312 - self[0][0] * t1312 - self[0][3] * t0112),
                ],
                [
                    inv_det * (self[1][1] * t0223 - self[1][0] * t1223 - self[1][2] * t0123),
                    inv_det * (self[0][0] * t1223 - self[0][1] * t0223 + self[0][2] * t0123),
                    inv_det * (self[0][1] * t0213 - self[0][0] * t1213 - self[0][2] * t0113),
                    inv_det * (self[0][0] * t1212 - self[0][1] * t0212 + self[0][2] * t0112),
                ],
            ]))
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};
    use std::iter::zip;

    macro_rules! assert_approx_eq {
        ($mat1:expr, $mat2:expr) => {
            for (row1, row2) in zip($mat1.data, $mat2.data) {
                for (x1, x2) in zip(row1, row2) {
                    assert!(approx_eq!(x1, x2));
                }
            }
        };
    }

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
        let o = Matrix::<4, 1>::new([[18.0], [24.0], [33.0], [1.0]]);
        assert_eq!(m.matmul(p), o);
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
        let o = Matrix::<4, 1>::new([[14.0], [22.0], [32.0], [0.0]]);
        assert_eq!(m.matmul(v), o);
    }

    #[test]
    fn matrix_multiplication_with_identity() {
        let m = Matrix2x2::new([[0.0, 1.0], [2.0, 3.0]]);
        let id = Matrix2x2::identity();
        assert_eq!(m.matmul(id), m);
    }

    #[test]
    fn matrix_transposition() {
        let m = Matrix::<3, 2>::new([[0.0, 0.1], [1.0, 1.1], [2.0, 2.1]]);
        let t = Matrix::<2, 3>::new([[0.0, 1.0, 2.0], [0.1, 1.1, 2.1]]);
        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn determinant_of_matrix2x2() {
        assert_eq!(
            Matrix2x2::new([[1.0, 5.0], [-3.0, 2.0]]).determinant(),
            17.0
        );
    }

    #[test]
    fn inverse_of_matrix2x2() {
        let mat = Matrix2x2::new([[3.0, 2.0], [1.0, 1.0]]);
        let inv = mat.inverse().unwrap();
        let eye = Matrix2x2::identity();
        assert_approx_eq!(mat.matmul(inv), eye);
        assert_approx_eq!(eye.inverse().unwrap(), eye);
        assert_eq!(Matrix2x2::ones().inverse(), None);
    }

    #[test]
    fn determinant_of_matrix3x3() {
        assert_eq!(
            Matrix3x3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]).determinant(),
            -196.0
        );
    }

    #[test]
    fn inverse_of_matrix3x3() {
        let mat = Matrix3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 7.0], [8.0, 9.0, 12.0]]);
        let inv = mat.inverse().unwrap();
        let eye = Matrix3x3::identity();
        assert_approx_eq!(mat.matmul(inv), eye);
        assert_approx_eq!(eye.inverse().unwrap(), eye);
        assert_eq!(Matrix3x3::ones().inverse(), None);
    }

    #[test]
    fn determinant_of_matrix4x4() {
        assert_eq!(
            Matrix4x4::new([
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0]
            ])
            .determinant(),
            -4071.0
        );
    }

    #[test]
    fn inverse_of_matrix4x4() {
        let mat = Matrix4x4::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let inv = mat.inverse().unwrap();
        let eye = Matrix4x4::identity();
        assert_approx_eq!(mat.matmul(inv), eye);
        assert_approx_eq!(eye.inverse().unwrap(), eye);
        assert_eq!(Matrix4x4::ones().inverse(), None);
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
