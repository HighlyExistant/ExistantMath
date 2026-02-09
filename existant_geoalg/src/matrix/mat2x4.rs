use bytemuck::{Pod, Zeroable};
use existant_core::{Ring, Semiring};

use crate::{matrix::{Matrix, Matrix3x4}, vectors::Vector4};

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
    type TransposeMatrix = Self;
    fn transpose(&self) -> Self::TransposeMatrix {
        Self::new(Vector4::forward(), Vector4::forward())
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