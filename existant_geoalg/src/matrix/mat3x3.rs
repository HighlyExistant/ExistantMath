use core::ops::Index;

use bytemuck::{Pod, Zeroable};
use existant_core::{Addition, AssociativeOver, ClosedUnder, CommutativeOver, Identity, Inverse, Multiplication, Ring, Semimodule, Semiring, Subtraction};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix2x2, Matrix2x3, SquareMatrix}, vectors::Vector3};

/// Represents a matrix with 3 columns and 3 rows.
/// ```
/// ┌a, d, g┐
/// │b, e, h│
/// └c, f, i┘
/// ```
#[matrix_multiplication]
#[matrix_multiplication(columns(x, y), self_rows(x, y, z), ty(Matrix2x3), output(Matrix2x3))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix3x3<T: Semiring> {
    /// First column of the matrix
    pub x: Vector3<T>,
    /// Second column of the matrix
    pub y: Vector3<T>,
    /// Third column of the matrix
    pub z: Vector3<T>,
}
unsafe impl<T: Semiring + Zeroable> Zeroable for Matrix3x3<T> {
    
}
unsafe impl<T: Semiring + Zeroable + 'static> Pod for Matrix3x3<T> {
    
}

impl<T: Semiring> Index<usize> for Matrix3x3<T> {
    type Output = Vector3<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = self.as_slice();
        &val[index]
    }
}
impl<T: Semiring> core::ops::IndexMut<usize> for Matrix3x3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut val = self.as_mut_slice();
        &mut val[index]
    }
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Neg<Output = T> + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> SquareMatrix for Matrix3x3<T> {
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
            Vector3::new(self.cofactor(0, 0), self.cofactor(0, 1), self.cofactor(0, 2)), 
            Vector3::new(self.cofactor(1, 0), self.cofactor(1, 1), self.cofactor(1, 2)), 
            Vector3::new(self.cofactor(2, 0), self.cofactor(2, 1), self.cofactor(2, 2)), 
        )
    }
    fn determinant(&self) -> <Self::Vector as Semimodule>::Scalar {
        ((self.x.x*self.y.y*self.z.z) + (self.y.x*self.z.y*self.x.z) + (self.z.x*self.x.y*self.y.z)) -
        ((self.x.z*self.y.y*self.z.x) + (self.y.z*self.z.y*self.x.x) + (self.z.z*self.x.y*self.y.x))
    }
    fn minor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar {
        let mut mat2 = Matrix2x2::empty();
        let mut idx_y = 0;
        for i in 0..2usize {
            if idx_y == row {
                idx_y += 1;
            }
            let mut idx_x = 0;
            for j in 0..2usize {
                if idx_x == column {
                    idx_x += 1;
                }
                mat2[j][i] = self[idx_x][idx_y];
                idx_x += 1;
            }
            idx_y += 1;
        }
        mat2.determinant()
    }
}


impl<T: Semiring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix3x3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix3x3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: Semiring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix3x3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Div<Output = T> + core::ops::Neg<Output = T> + core::ops::Add<Output = T> + core::ops::Sub<Output = T>> Inverse<Multiplication> for Matrix3x3<T> {
    fn inverse(self) -> Self {
        self.adjoint()*(<T as Identity<Multiplication>>::IDENTITY/self.determinant())
    }
}

impl<T: Ring + core::ops::Mul<Output = T>> Matrix for Matrix3x3<T> {
    type Vector = Vector3<T>;
    type TransposeMatrix = Self;
   fn transpose(&self) -> Self {
        Self::new(
            Vector3::new(self.x.x, self.y.x, self.z.x), 
            Vector3::new(self.x.y, self.y.y, self.z.y),
            Vector3::new(self.x.z, self.y.z, self.z.z),
        )
    }
}

impl<T: Semiring> Matrix3x3<T> {
    pub const fn new(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self { x, y, z }
    }
    pub const fn empty() -> Self {
        <Self as Identity<Addition>>::IDENTITY
    }
    pub const fn from_diagonal(diagonal: Vector3<T>) -> Self {
        Self::new(
            Vector3::new(diagonal.x, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector3::new(<T as Identity<Addition>>::IDENTITY, diagonal.y, <T as Identity<Addition>>::IDENTITY), 
            Vector3::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, diagonal.z), 
        )
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
    pub const fn get_x_row(&self) -> Vector3<T> {
        Vector3::new(self.x.x, self.y.x, self.z.x)
    }
    #[inline]
    pub const fn get_y_row(&self) -> Vector3<T> {
        Vector3::new(self.x.y, self.y.y, self.z.y)
    }
    #[inline]
    pub const fn get_z_row(&self) -> Vector3<T> {
        Vector3::new(self.x.z, self.y.z, self.z.z)
    }
    pub fn as_slice(&self) -> &[Vector3<T>] {
        unsafe { 
            core::slice::from_raw_parts(self as *const _ as _, 3) 
        }
    }
    pub fn as_mut_slice(&mut self) -> &mut [Vector3<T>] {
        unsafe { 
            core::slice::from_raw_parts_mut(self as *mut _ as _, 3) 
        }
    }
}
impl<T: Semiring + ClosedUnder<Addition>> ClosedUnder<Addition> for Matrix3x3<T> {
    
}
impl<T: Semiring + ClosedUnder<Subtraction>> ClosedUnder<Subtraction> for Matrix3x3<T> {
    
}
impl<T: Semiring + ClosedUnder<Multiplication>> ClosedUnder<Multiplication> for Matrix3x3<T> {
    
}
impl<T: Semiring + AssociativeOver<Addition>> AssociativeOver<Addition> for Matrix3x3<T> {
    
}
impl<T: Semiring + CommutativeOver<Addition>> CommutativeOver<Addition> for Matrix3x3<T> {

}

impl<T: Ring> Identity<Multiplication> for Matrix3x3<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector3<T> as Identity<Multiplication>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Multiplication>>::is_identity(&self.x.x) && 
        <T as Identity<Multiplication>>::is_identity(&self.y.y) && 
        <T as Identity<Multiplication>>::is_identity(&self.z.z) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.z) && 
        <T as Identity<Addition>>::is_identity(&self.x.y) &&
        <T as Identity<Addition>>::is_identity(&self.x.z) &&
        <T as Identity<Addition>>::is_identity(&self.z.y) &&
        <T as Identity<Addition>>::is_identity(&self.z.x)
    }
}
impl<T: Semiring> Identity<Addition> for Matrix3x3<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector3<T> as Identity<Addition>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Addition>>::is_identity(&self.x.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.y) && 
        <T as Identity<Addition>>::is_identity(&self.z.z) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.z) && 
        <T as Identity<Addition>>::is_identity(&self.x.y) &&
        <T as Identity<Addition>>::is_identity(&self.x.z) &&
        <T as Identity<Addition>>::is_identity(&self.z.y) &&
        <T as Identity<Addition>>::is_identity(&self.z.x)
    }
}

pub type I8Mat3 = Matrix3x3<i8>;
pub type I16Mat3 = Matrix3x3<i16>;
pub type I32Mat3 = Matrix3x3<i32>;
pub type I64Mat3 = Matrix3x3<i64>;
pub type I128Mat3 = Matrix3x3<i128>;
pub type U8Mat3 = Matrix3x3<u8>;
pub type U16Mat3 = Matrix3x3<u16>;
pub type U32Mat3 = Matrix3x3<u32>;
pub type U64Mat3 = Matrix3x3<u64>;
pub type U128Mat3 = Matrix3x3<u128>;
pub type FMat3 = Matrix3x3<f32>;
pub type DMat3 = Matrix3x3<f64>;