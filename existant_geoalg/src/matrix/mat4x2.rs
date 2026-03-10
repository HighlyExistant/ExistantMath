use bytemuck::{Pod, Zeroable};
use existant_core::Ring;
use existant_geoalg_macros::matrix_multiplication;
use crate::{matrix::{Matrix, Matrix2x4, Matrix4x4}, vectors::{Vector2, Vector4}};

/// Represents a matrix with 4 columns and 2 rows.
/// ```
/// ┌a, c, e, g┐
/// └b, d, f, h┘
/// ```
#[matrix_multiplication(columns(x, y, z, w), self_rows(x, y), ty(Matrix4x4), output(Matrix4x2))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix4x2<T: Ring> {
    /// First column of the matrix
    pub x: Vector2<T>,
    /// Second column of the matrix
    pub y: Vector2<T>,
    /// Third column of the matrix
    pub z: Vector2<T>,
    /// Fourth column of the matrix
    pub w: Vector2<T>,
}
unsafe impl<T: Ring + Zeroable> Zeroable for Matrix4x2<T> {
    
}
unsafe impl<T: Ring + Zeroable + 'static> Pod for Matrix4x2<T> {
    
}

impl<T: Ring> Matrix for Matrix4x2<T> {
    type Vector = Vector2<T>;
    type TransposeMatrix = Matrix2x4<T>;
    fn transpose(&self) -> Self::TransposeMatrix {
        Matrix2x4::new(
            self.get_x_row(),
            self.get_y_row()
        )
    }
}

impl<T: Ring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix4x2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix4x2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl<T: Ring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix4x2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T: Ring> Matrix4x2<T>  {
    pub const fn new(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>, w: Vector2<T>) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
    #[inline]
    pub const fn set_x_row(&mut self, row: Vector4<T>) {
        self.x.x = row.x;
        self.y.x = row.y;
        self.z.x = row.z;
        self.w.x = row.w;
    }
    #[inline]
    pub const fn set_y_row(&mut self, row: Vector4<T>) {
        self.x.y = row.x;
        self.y.y = row.y;
        self.z.y = row.z;
        self.w.y = row.w;
    }
    #[inline]
    pub const fn get_x_row(&self) -> Vector4<T> {
        Vector4::new(self.x.x,self.y.x,self.z.x, self.w.x)
    }
    #[inline]
    pub const fn get_y_row(&self) -> Vector4<T> {
        Vector4::new(self.x.y,self.y.y,self.z.y, self.w.y)
    }
    pub fn as_slice(&self) -> &[Vector4<T>] {
        unsafe { 
            core::slice::from_raw_parts(self as *const _ as _, 4) 
        }
    }
    pub fn as_mut_slice(&mut self) -> &mut [Vector4<T>] {
        unsafe { 
            core::slice::from_raw_parts_mut(self as *mut _ as _, 4) 
        }
    }
}

pub type I8Mat4x2 = Matrix4x2<i8>;
pub type I16Mat4x2 = Matrix4x2<i16>;
pub type I32Mat4x2 = Matrix4x2<i32>;
pub type I64Mat4x2 = Matrix4x2<i64>;
pub type I128Mat4x2 = Matrix4x2<i128>;
pub type U8Mat4x2 = Matrix4x2<u8>;
pub type U16Mat4x2 = Matrix4x2<u16>;
pub type U32Mat4x2 = Matrix4x2<u32>;
pub type U64Mat4x2 = Matrix4x2<u64>;
pub type U128Mat4x2 = Matrix4x2<u128>;
pub type FMat4x2 = Matrix4x2<f32>;
pub type DMat4x2 = Matrix4x2<f64>;