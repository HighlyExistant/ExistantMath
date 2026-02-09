use bytemuck::{Pod, Zeroable};
use existant_core::{Addition, Field, Identity, Multiplication, Ring};
use crate::{matrix::{Matrix, Matrix3x3, Matrix3x4, SolveEquations, SquareMatrix}, vectors::{Vector3, Vector4}};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix4x3<T: Ring> {
    /// First column of the matrix
    pub x: Vector3<T>,
    /// Second column of the matrix
    pub y: Vector3<T>,
    /// Third column of the matrix
    pub z: Vector3<T>,
    /// Fourth column of the matrix
    pub w: Vector3<T>,
}
unsafe impl<T: Ring + Zeroable> Zeroable for Matrix4x3<T> {
    
}
unsafe impl<T: Ring + Zeroable + 'static> Pod for Matrix4x3<T> {
    
}

impl<T: Ring> Matrix for Matrix4x3<T> {
    type Vector = Vector3<T>;
    type TransposeMatrix = Matrix3x4<T>;
    fn transpose(&self) -> Self::TransposeMatrix {
        Matrix3x4::new(
            self.get_x_row(), 
            self.get_y_row(), 
            self.get_z_row()
        )
    }
}

impl<T: Ring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix4x3<T> {
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

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix4x3<T> {
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
impl<T: Ring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix4x3<T> {
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
impl<T: Field + core::ops::Div<Output = T> + core::ops::Mul<Output = T> + core::ops::Sub<Output = T> + core::ops::Add<Output = T> + core::ops::Neg<Output = T> + PartialEq> SolveEquations for Matrix4x3<T> {
    fn solve_system(&self) -> Option<Self> {
        let det = Matrix3x3::new(
            Vector3::new(self.x.x, self.x.y, self.x.z),
            Vector3::new(self.y.x, self.y.y, self.y.z),
            Vector3::new(self.z.x, self.z.y, self.z.z),
        ).determinant();
        if det == <T as Identity<Addition>>::IDENTITY {
            return None;
        }
        let detx = Matrix3x3::new(
            Vector3::new(self.w.x, self.w.y, self.w.z),
            Vector3::new(self.y.x, self.y.y, self.y.z),
            Vector3::new(self.z.x, self.z.y, self.z.z),
        ).determinant();
        let dety = Matrix3x3::new(
            Vector3::new(self.x.x, self.x.y, self.x.z),
            Vector3::new(self.w.x, self.w.y, self.w.z),
            Vector3::new(self.z.x, self.z.y, self.z.z),
        ).determinant();
        let detz = Matrix3x3::new(
            Vector3::new(self.x.x, self.x.y, self.x.z),
            Vector3::new(self.y.x, self.y.y, self.y.z),
            Vector3::new(self.w.x, self.w.y, self.w.z),
        ).determinant();
        
        Some(Self::new(
            Vector3::new(<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector3::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector3::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY), 
            Vector3::new(detx/det, dety/det, detz/det), 
        ))
    }
}

impl<T: Ring> Matrix4x3<T>  {
    pub const fn new(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>, w: Vector3<T>) -> Self {
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
    pub const fn set_z_row(&mut self, row: Vector4<T>) {
        self.x.z = row.x;
        self.y.z = row.y;
        self.z.z = row.z;
        self.w.z = row.w;
    }
    #[inline]
    pub const fn get_x_row(&self) -> Vector4<T> {
        Vector4::new(self.x.x,self.y.x,self.z.x, self.w.x)
    }
    #[inline]
    pub const fn get_y_row(&self) -> Vector4<T> {
        Vector4::new(self.x.y,self.y.y,self.z.y, self.w.y)
    }
    #[inline]
    pub const fn get_z_row(&self) -> Vector4<T> {
        Vector4::new(self.x.z,self.y.z,self.z.z, self.w.z)
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

pub type I8Mat4x3 = Matrix4x3<i8>;
pub type I16Mat4x3 = Matrix4x3<i16>;
pub type I32Mat4x3 = Matrix4x3<i32>;
pub type I64Mat4x3 = Matrix4x3<i64>;
pub type I128Mat4x3 = Matrix4x3<i128>;
pub type U8Mat4x3 = Matrix4x3<u8>;
pub type U16Mat4x3 = Matrix4x3<u16>;
pub type U32Mat4x3 = Matrix4x3<u32>;
pub type U64Mat4x3 = Matrix4x3<u64>;
pub type U128Mat4x3 = Matrix4x3<u128>;
pub type FMat4x3 = Matrix4x3<f32>;
pub type DMat4x3 = Matrix4x3<f64>;