use core::ops::Index;

use bytemuck::{Pod, Zeroable};
use existant_core::{Addition, AssociativeOver, ClosedUnder, CommutativeOver, Identity, Inverse, Multiplication, Ring, Semimodule, Semiring, Subtraction};
use existant_geoalg_macros::matrix_multiplication;
use crate::{matrix::{Matrix, Matrix3x2, SquareMatrix}, vectors::Vector2};

/// Represents a matrix with 2 columns and 2 rows.
/// ```
/// ┌a, c┐
/// └b, d┘
/// ```
#[matrix_multiplication]
#[matrix_multiplication(columns(x, y, z), self_rows(x, y), ty(Matrix3x2), output(Matrix3x2))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix2x2<T: Semiring> {
    /// First column of the matrix
    pub x: Vector2<T>,
    /// Second column of the matrix
    pub y: Vector2<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix2x2<T> {
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix2x2<T> {
    
}

impl<T: Semiring> Index<usize> for Matrix2x2<T> {
    type Output = Vector2<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = self.as_slice();
        &val[index]
    }
}
impl<T: Semiring> core::ops::IndexMut<usize> for Matrix2x2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = self.as_mut_slice();
        &mut val[index]
    }
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Div<Output = T> + core::ops::Neg<Output = T> + core::ops::Sub<Output = T>> Inverse<Multiplication> for Matrix2x2<T> {
    
    fn inverse(self) -> Self {
        self.adjoint()*(<T as Identity<Multiplication>>::IDENTITY/self.determinant())
    }
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Neg<Output = T> + core::ops::Sub<Output = T>> SquareMatrix for Matrix2x2<T> {
    fn cofactor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar {
        // check if the sum of columns and rows is even
        if column + row & 1 == 0 {
            self.minor(column, row)
        } else {
            -self.minor(column, row)
        }
    }
    fn cofactor_matrix(&self) -> Self {
        Self::new(
            Vector2::new(self.cofactor(0, 0), self.cofactor(0, 1)), 
            Vector2::new(self.cofactor(1, 0), self.cofactor(1, 1)), 
        )
    }
    fn determinant(&self) -> <Self::Vector as Semimodule>::Scalar {
       self.x.x*self.y.y - self.x.y*self.y.x
    }
    
    fn minor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar {
        // 0bxy
        // x: row
        // y: column 
        match column|(row<<1) {
            0b00 => self.y.y,
            0b01 => self.x.y,
            0b10 => self.y.x,
            0b11 => self.x.x,
            _ => panic!("Outside of the range to calculate the minor.")
        }
    }
}

impl<T: Semiring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix2x2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix2x2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl<T: Semiring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix2x2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Ring + core::ops::Mul<Output = T>> Matrix for Matrix2x2<T> {
    type Vector = Vector2<T>;
    type TransposeMatrix = Self;
    fn transpose(&self) -> Self {
        Self::new(
            Vector2::new(self.x.x, self.y.x), 
            Vector2::new(self.x.y, self.y.y)
        )
    }
}

impl<T: Semiring> Matrix2x2<T> {
    pub const fn new(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self { x, y }
    }
    pub const fn empty() -> Self {
        <Self as Identity<Addition>>::IDENTITY
    }
    pub const fn from_diagonal(diagonal: Vector2<T>) -> Self {
        Self::new(
            Vector2::new(diagonal.x, <T as Identity<Addition>>::IDENTITY), 
            Vector2::new(<T as Identity<Addition>>::IDENTITY, diagonal.y), 
        )
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
    pub fn as_slice(&self) -> &[Vector2<T>] {
        unsafe { 
            core::slice::from_raw_parts(self as *const _ as _, 2) 
        }
    }
    pub fn as_mut_slice(&mut self) -> &mut [Vector2<T>] {
        unsafe { 
            core::slice::from_raw_parts_mut(self as *mut _ as _, 2) 
        }
    }
}
impl<T: Semiring + ClosedUnder<Addition>> ClosedUnder<Addition> for Matrix2x2<T> {
    
}
impl<T: Semiring + ClosedUnder<Subtraction>> ClosedUnder<Subtraction> for Matrix2x2<T> {
    
}
impl<T: Semiring + ClosedUnder<Multiplication>> ClosedUnder<Multiplication> for Matrix2x2<T> {
    
}
impl<T: Semiring + AssociativeOver<Addition>> AssociativeOver<Addition> for Matrix2x2<T> {
    
}
impl<T: Semiring + CommutativeOver<Addition>> CommutativeOver<Addition> for Matrix2x2<T> {

}

impl<T: Ring> Identity<Multiplication> for Matrix2x2<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector2<T> as Identity<Multiplication>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Multiplication>>::is_identity(&self.x.x) && 
        <T as Identity<Multiplication>>::is_identity(&self.y.y) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.x.y)
    }
}
impl<T: Semiring> Identity<Addition> for Matrix2x2<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector2<T> as Identity<Addition>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Addition>>::is_identity(&self.x.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.y) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.x.y)
    }
}

pub type I8Mat2 = Matrix2x2<i8>;
pub type I16Mat2 = Matrix2x2<i16>;
pub type I32Mat2 = Matrix2x2<i32>;
pub type I64Mat2 = Matrix2x2<i64>;
pub type I128Mat2 = Matrix2x2<i128>;
pub type U8Mat2 = Matrix2x2<u8>;
pub type U16Mat2 = Matrix2x2<u16>;
pub type U32Mat2 = Matrix2x2<u32>;
pub type U64Mat2 = Matrix2x2<u64>;
pub type U128Mat2 = Matrix2x2<u128>;
pub type F32Mat2 = Matrix2x2<f32>;
pub type F64Mat2 = Matrix2x2<f64>;