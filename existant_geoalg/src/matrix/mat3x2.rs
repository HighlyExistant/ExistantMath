use bytemuck::{Pod, Zeroable};
use existant_core::{Addition, Field, Identity, Multiplication, Ring, Semiring};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix2x2, Matrix2x3, SolveEquations, SquareMatrix}, vectors::{Vector2, Vector3}};

#[matrix_multiplication(columns(x, y), self_rows(x, y), ty(Matrix2x3), output(Matrix2x2))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix3x2<T: Semiring> {
    /// First column of the matrix
    pub x: Vector2<T>,
    /// Second column of the matrix
    pub y: Vector2<T>,
    /// Third column of the matrix
    pub z: Vector2<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix3x2<T> {
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix3x2<T> {
    
}

impl<T: Semiring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix3x2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix3x2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: Semiring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix3x2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl<T: Ring + core::ops::Mul<Output = T>> Matrix for Matrix3x2<T> {
    type Vector = Vector2<T>;
    type TransposeMatrix = Matrix2x3<T>;
    fn transpose(&self) -> Self::TransposeMatrix {
        Self::TransposeMatrix::new(
            Vector3::new(self.x.x, self.y.x, self.z.x), 
            Vector3::new(self.x.y, self.y.y, self.z.y),
        )
    }
}

impl<T: Semiring + core::ops::Add<Output = T> + core::ops::Mul<Output = T>> core::ops::Mul<Vector3<T>> for Matrix3x2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector2::new(
            self.x.x*rhs.x + self.y.x*rhs.y + self.z.x*rhs.z, 
            self.x.y*rhs.x + self.y.y*rhs.y + self.z.y*rhs.z, 
        )
    }
}

impl<T: Field + core::ops::Div<Output = T> + core::ops::Mul<Output = T> + core::ops::Sub<Output = T> + core::ops::Neg<Output = T> + PartialEq> SolveEquations for Matrix3x2<T> {
    fn solve_system(&self) -> Option<Self> {
        let det = Matrix2x2::new(
            Vector2::new(self.x.x, self.x.y),
            Vector2::new(self.y.x, self.y.y)
        ).determinant();
        if det == <T as Identity<Addition>>::IDENTITY {
            return None;
        }
        let detx = Matrix2x2::new(
            Vector2::new(self.z.x, self.z.y),
            Vector2::new(self.y.x, self.y.y)
        ).determinant();
        let dety = Matrix2x2::new(
            Vector2::new(self.x.x, self.x.y),
            Vector2::new(self.z.x, self.z.y)
        ).determinant();

        Some(Self::new(
            Vector2::new(<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector2::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY), 
            Vector2::new(detx/det, dety/det)
        ))
    }
}

impl<T: Semiring> Matrix3x2<T> {
    pub const fn new(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>) -> Self {
        Self { x, y, z }
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
}

pub type I8Mat3x2 = Matrix3x2<i8>;
pub type I16Mat3x2 = Matrix3x2<i16>;
pub type I32Mat3x2 = Matrix3x2<i32>;
pub type I64Mat3x2 = Matrix3x2<i64>;
pub type I128Mat3x2 = Matrix3x2<i128>;
pub type U8Mat3x2 = Matrix3x2<u8>;
pub type U16Mat3x2 = Matrix3x2<u16>;
pub type U32Mat3x2 = Matrix3x2<u32>;
pub type U64Mat3x2 = Matrix3x2<u64>;
pub type U128Mat3x2 = Matrix3x2<u128>;
pub type FMat3x2 = Matrix3x2<f32>;
pub type DMat3x2 = Matrix3x2<f64>;