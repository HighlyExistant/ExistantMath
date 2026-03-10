use bytemuck::{Pod, Zeroable};
use existant_core::{Ring, Semiring};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix4x2, Matrix4x4}, vectors::{Vector2, Vector4}};

/// Represents a matrix with 2 columns and 4 rows.
/// ```
/// ┌a, e┐
/// │b, f│
/// │c, g│
/// └d, h┘
/// ```
#[matrix_multiplication(columns(x, y, z, w), self_rows(x, y, z, w), ty(Matrix4x4), output(Matrix4x4))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix2x4<T: Semiring> {
    /// First column of the matrix
    pub x: Vector4<T>,
    /// Second column of the matrix
    pub y: Vector4<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix2x4<T> {
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix2x4<T> {
    
}

impl<T: Ring> Matrix for Matrix2x4<T> {
    type Vector = Vector4<T>;
    type TransposeMatrix = Matrix4x2<T>;
    fn transpose(&self) -> Self::TransposeMatrix {
        Matrix4x2::new(
            Vector2::new(self.x.x, self.y.x), 
            Vector2::new(self.x.y, self.y.y), 
            Vector2::new(self.x.z, self.y.z), 
            Vector2::new(self.x.y, self.y.y)
        )
    }
}

impl<T: Semiring> Matrix2x4<T>  {
    pub const fn new(x: Vector4<T>, y: Vector4<T>) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub type I8Mat2x4 = Matrix2x4<i8>;
pub type I16Mat2x4 = Matrix2x4<i16>;
pub type I32Mat2x4 = Matrix2x4<i32>;
pub type I64Mat2x4 = Matrix2x4<i64>;
pub type I128Mat2x4 = Matrix2x4<i128>;
pub type U8Mat2x4 = Matrix2x4<u8>;
pub type U16Mat2x4 = Matrix2x4<u16>;
pub type U32Mat2x4 = Matrix2x4<u32>;
pub type U64Mat2x4 = Matrix2x4<u64>;
pub type U128Mat2x4 = Matrix2x4<u128>;
pub type FMat2x4 = Matrix2x4<f32>;
pub type DMat2x4 = Matrix2x4<f64>;