use std::ops::Mul;

use super::Vector;

/// Matrix for 3D transformations
#[derive(Debug, Default, PartialEq)]
pub struct Matrix {
    pub mat: [[f64; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(vector: impl Into<Vector>) -> Self {
        let Vector { x, y, z, w } = vector.into();
        Self {
            mat: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, w],
            ],
        }
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut result = Matrix::default();

        for i in 0..4 {
            for j in 0..4 {
                result.mat[i][j] = {
                    let mut sum = 0.0;

                    for k in 0..4 {
                        sum += self.mat[i][k] * rhs.mat[k][j];
                    }

                    sum
                };
            }
        }

        result
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let mut result = Vector::default();

        for i in 0..4 {
            result[i] = {
                let mut sum = 0.0;

                for j in 0..4 {
                    sum += self.mat[i][j] * rhs[j];
                }

                sum
            }
        }

        result
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = Matrix::default();

        for i in 0..4 {
            for j in 0..4 {
                result.mat[i][j] = self.mat[i][j] * rhs;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat_mul_identity() {
        let mat_identity = Matrix::identity();
        let mat = Matrix {
            mat: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };

        let result = &mat * &mat_identity;

        assert_eq!(mat, result);
    }

    #[test]
    fn test_mat_mul_vector_identity() {
        let mat_identity = Matrix::identity();
        let vector = Vector::new(10.0, 20.0, 30.0, None);
        let result = &mat_identity * &vector;

        assert_eq!(vector, result);
    }

    #[test]
    fn test_mat_mul_vector_translation() {
        let translation = Matrix::translation((100.0, 200.0, 300.0));
        let vector = Vector::new(10.0, 20.0, 30.0, None);
        let result = &translation * &vector;

        assert_eq!(result, Vector::new(110.0, 220.0, 330.0, None));
    }
}
