use crate::tracer::vector::Vector;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const DIM: usize> {
    table: [[f32; DIM]; DIM],
}

impl<const DIM: usize> Matrix<DIM> {
    pub const IDENTITY_MATRIX: Self = Self::generate_identity_matrix();

    pub const fn new() -> Self {
        Self {
            table: [[0.0; DIM]; DIM],
        }
    }

    pub fn from_slice(values: &[f32]) -> Self {
        let mut matrix = Self::new();
        let mut iter = values.iter();

        for i in 0..DIM {
            for j in 0..DIM {
                matrix.table[i][j] = *iter.next().unwrap_or(&0.0);
            }
        }

        matrix
    }

    const fn generate_identity_matrix() -> Self {
        let mut matrix = Self::new();

        let mut i = 0;
        while i < DIM {
            matrix.table[i][i] = 1.0;
            i += 1;
        }

        matrix
    }
}

impl<const DIM: usize> Index<usize> for Matrix<DIM> {
    type Output = [f32; DIM];

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl<const DIM: usize> IndexMut<usize> for Matrix<DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}

impl<const DIM: usize> Add for Matrix<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for i in 0..DIM {
            for j in 0..DIM {
                result.table[i][j] += rhs.table[i][j];
            }
        }
        result
    }
}

impl<const DIM: usize> AddAssign for Matrix<DIM> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..DIM {
            for j in 0..DIM {
                self.table[i][j] += rhs.table[i][j];
            }
        }
    }
}

impl<const DIM: usize> Mul for Matrix<DIM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    result.table[i][j] += self.table[i][k] * rhs.table[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Matrix<3>> for Vector {
    type Output = Self;

    fn mul(self, rhs: Matrix<3>) -> Self::Output {
        Vector(
            self.0 * rhs[0][0] + self.1 * rhs[1][0] + self.2 * rhs[2][0],
            self.0 * rhs[0][1] + self.1 * rhs[1][1] + self.2 * rhs[2][1],
            self.0 * rhs[0][2] + self.1 * rhs[1][2] + self.2 * rhs[2][2],
        )
    }
}

impl MulAssign<Matrix<3>> for Vector {
    fn mul_assign(&mut self, rhs: Matrix<3>) {
        *self = self.clone() * rhs;
    }
}
