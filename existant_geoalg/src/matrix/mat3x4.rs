use bytemuck::{Pod, Zeroable};
use existant_core::{Ring, Semiring};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix2x3, Matrix2x4}, vectors::{Vector3, Vector4}};

/// Represents a matrix with 3 columns and 4 rows.
/// ```
/// ┌a, e, i┐
/// │b, f, j│
/// │c, g, k│
/// └d, h, l┘
/// ```
#[matrix_multiplication(columns(x, y), self_rows(x, y, z, w), ty(Matrix2x3), output(Matrix2x4))]
#[matrix_multiplication(columns(x, y, z), self_rows(x, y, z, w), ty(Matrix3x4), output(Matrix3x4))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix3x4<T: Semiring> {
    /// First column of the matrix
    pub x: Vector4<T>,
    /// Second column of the matrix
    pub y: Vector4<T>,
    /// Third column of the matrix
    pub z: Vector4<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix3x4<T> {
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix3x4<T> {
    
}

impl<T: Ring> Matrix for Matrix3x4<T> {
    type Vector = Vector4<T>;
    type TransposeMatrix = Self;
    fn transpose(&self) -> Self::TransposeMatrix {
        Self::new(Vector4::forward(), Vector4::forward(), Vector4::forward())
    }
}

impl<T: Semiring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix3x4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix3x4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: Semiring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix3x4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl<T: Semiring> Matrix3x4<T>  {
    pub const fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    #[inline]
    pub const fn set_x_row(&mut self, row: Vector3<T>) {
        self.x.x = row.x;
        self.y.x = row.y;
        self.z.x = row.z;
    }
    #[inline]
    pub const fn set_y_row(&mut self, row: Vector3<T>) {
        self.x.y = row.x;
        self.y.y = row.y;
        self.z.y = row.z;
    }
    #[inline]
    pub const fn set_z_row(&mut self, row: Vector3<T>) {
        self.x.z = row.x;
        self.y.z = row.y;
        self.z.z = row.z;
    }
    #[inline]
    pub const fn set_w_row(&mut self, row: Vector3<T>) {
        self.x.w = row.x;
        self.y.w = row.y;
        self.z.w = row.z;
    }
    #[inline]
    pub const fn get_x_row(&self) -> Vector3<T> {
        Vector3::new(self.x.x,self.y.x,self.z.x)
    }
    #[inline]
    pub const fn get_y_row(&self) -> Vector3<T> {
        Vector3::new(self.x.y,self.y.y,self.z.y)
    }
    #[inline]
    pub const fn get_z_row(&self) -> Vector3<T> {
        Vector3::new(self.x.z,self.y.z,self.z.z)
    }
    #[inline]
    pub const fn get_w_row(&self) -> Vector3<T> {
        Vector3::new(self.x.z,self.y.z,self.z.z)
    }
    pub fn as_slice(&self) -> &[Vector4<T>] {
        unsafe { 
            core::slice::from_raw_parts(self as *const _ as _, 3) 
        }
    }
    pub fn as_mut_slice(&mut self) -> &mut [Vector4<T>] {
        unsafe { 
            core::slice::from_raw_parts_mut(self as *mut _ as _, 3) 
        }
    }
}


pub type I8Mat3x4 = Matrix3x4<i8>;
pub type I16Mat3x4 = Matrix3x4<i16>;
pub type I32Mat3x4 = Matrix3x4<i32>;
pub type I64Mat3x4 = Matrix3x4<i64>;
pub type I128Mat3x4 = Matrix3x4<i128>;
pub type U8Mat3x4 = Matrix3x4<u8>;
pub type U16Mat3x4 = Matrix3x4<u16>;
pub type U32Mat3x4 = Matrix3x4<u32>;
pub type U64Mat3x4 = Matrix3x4<u64>;
pub type U128Mat3x4 = Matrix3x4<u128>;
pub type FMat3x4 = Matrix3x4<f32>;
pub type DMat3x4 = Matrix3x4<f64>;