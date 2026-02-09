use bytemuck::{Pod, Zeroable};
use existant_core::{Ring, Semiring};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix3x3, Matrix2x2, Matrix3x2}, vectors::{Vector2, Vector3}};
#[matrix_multiplication(columns(x, y), self_rows(x, y), ty(Matrix2x2), output(Matrix2x2))]
#[matrix_multiplication(self_rows(x, y, z), columns(x, y, z), ty(Matrix3x2), output(Matrix3x3))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix2x3<T: Semiring> {
    /// First column of the matrix
    pub x: Vector3<T>,
    /// Second column of the matrix
    pub y: Vector3<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix2x3<T> { 
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix2x3<T> {
    
}

impl<T: Semiring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix2x3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix2x3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl<T: Semiring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix2x3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl<T: Semiring + core::ops::Add<Output = T> + core::ops::Mul<Output = T>> core::ops::Mul<Vector2<T>> for Matrix2x3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector3::new(
            self.x.x*rhs.x + self.y.x*rhs.y, 
            self.x.y*rhs.x + self.y.y*rhs.y, 
            self.x.z*rhs.x + self.y.z*rhs.y, 
        )
    }
}

impl<T: Ring + core::ops::Mul<Output = T>> Matrix for Matrix2x3<T> {
    type Vector = Vector3<T>;
    type TransposeMatrix = Matrix3x2<T>;
    fn transpose(&self) -> Self::TransposeMatrix {
        Self::TransposeMatrix::new(
            Vector2::new(self.x.x, self.y.x), 
            Vector2::new(self.x.y, self.y.y),
            Vector2::new(self.x.z, self.y.z),
        )
    }
}

impl<T: Semiring> Matrix2x3<T> {
    pub const fn new(x: Vector3<T>, y: Vector3<T>) -> Self {
        Self { x, y }
    }
    #[inline]
    pub const fn set_x_row(&mut self, row: Vector2<T>) {
        self.x.x = row.x;
        self.y.x = row.y;
    }
    #[inline]
    pub const fn set_y_row(&mut self, row: Vector2<T>) {
        self.x.y = row.x;
        self.y.y = row.y;
    }
    #[inline]
    pub const fn set_z_row(&mut self, row: Vector2<T>) {
        self.x.z = row.x;
        self.y.z = row.y;
    }
}

pub type I8Mat2x3 = Matrix2x3<i8>;
pub type I16Mat2x3 = Matrix2x3<i16>;
pub type I32Mat2x3 = Matrix2x3<i32>;
pub type I64Mat2x3 = Matrix2x3<i64>;
pub type I128Mat2x3 = Matrix2x3<i128>;
pub type U8Mat2x3 = Matrix2x3<u8>;
pub type U16Mat2x3 = Matrix2x3<u16>;
pub type U32Mat2x3 = Matrix2x3<u32>;
pub type U64Mat2x3 = Matrix2x3<u64>;
pub type U128Mat2x3 = Matrix2x3<u128>;
pub type FMat2x3 = Matrix2x3<f32>;
pub type DMat2x3 = Matrix2x3<f64>;