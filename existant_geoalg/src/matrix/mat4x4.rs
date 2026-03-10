use core::ops::Index;


use bytemuck::{Pod, Zeroable};
use existant_core::{Addition, AssociativeOver, BasicField, ClosedUnder, CommutativeOver, FloatingPoint, FromPrimitive, Identity, Inverse, Multiplication, Ring, Semimodule, Subtraction};
use existant_geoalg_macros::matrix_multiplication;

use crate::{matrix::{Matrix, Matrix2x4, Matrix3x3, Matrix3x4, SquareMatrix}, vectors::{InnerProductSpace, Vector3, Vector4}};

/// Represents a matrix with 4 columns and 4 rows.
/// ```
/// ┌a, e, i, m┐
/// │b, f, j, n│
/// │c, g, k, o│
/// └d, h, l, p┘
/// ```
#[matrix_multiplication]
#[matrix_multiplication(columns(x, y), self_rows(x, y, z, w), ty(Matrix2x4), output(Matrix2x4))]
#[matrix_multiplication(columns(x, y, z), self_rows(x, y, z, w), ty(Matrix3x4), output(Matrix3x4))]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix4x4<T: Ring> {
    /// First column of the matrix
    pub x: Vector4<T>,
    /// Second column of the matrix
    pub y: Vector4<T>,
    /// Third column of the matrix
    pub z: Vector4<T>,
    /// Third column of the matrix
    pub w: Vector4<T>,
}
unsafe impl<T: Ring + Zeroable> Zeroable for Matrix4x4<T> {
    
}
unsafe impl<T: Ring + Zeroable + 'static> Pod for Matrix4x4<T> {
    
}

impl<T: Ring> Index<usize> for Matrix4x4<T> {
    type Output = Vector4<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = self.as_slice();
        &val[index]
    }
}
impl<T: Ring> core::ops::IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = self.as_mut_slice();
        &mut val[index]
    }
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Neg<Output = T> + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> SquareMatrix for Matrix4x4<T> {
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
            Vector4::new(self.cofactor(0, 0), self.cofactor(0, 1), self.cofactor(0, 2), self.cofactor(0, 3)), 
            Vector4::new(self.cofactor(1, 0), self.cofactor(1, 1), self.cofactor(1, 2), self.cofactor(1, 3)), 
            Vector4::new(self.cofactor(2, 0), self.cofactor(2, 1), self.cofactor(2, 2), self.cofactor(2, 3)), 
            Vector4::new(self.cofactor(3, 0), self.cofactor(3, 1), self.cofactor(3, 2), self.cofactor(3, 3)), 
        )
    }
    fn determinant(&self) -> <Self::Vector as Semimodule>::Scalar {
        Matrix3x3::new(
            Vector3::new(self.y.y, self.z.y, self.w.y), 
            Vector3::new(self.y.z, self.z.z, self.w.z), 
            Vector3::new(self.y.w, self.z.w, self.w.w), 
        ).determinant()*self.x.x
        -
        Matrix3x3::new(
            Vector3::new(self.y.x, self.z.x, self.w.x), 
            Vector3::new(self.y.z, self.z.z, self.w.z), 
            Vector3::new(self.y.w, self.z.w, self.w.w), 
        ).determinant()*self.x.y
        +
        Matrix3x3::new(
            Vector3::new(self.y.x, self.z.x, self.w.x), 
            Vector3::new(self.y.y, self.z.y, self.w.y), 
            Vector3::new(self.y.w, self.z.w, self.w.w), 
        ).determinant()*self.x.z
        -
        Matrix3x3::new(
            Vector3::new(self.y.x, self.z.x, self.w.x), 
            Vector3::new(self.y.y, self.z.y, self.w.y), 
            Vector3::new(self.y.z, self.z.z, self.w.z), 
        ).determinant()*self.x.w
    }
    fn minor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar {
        let mut mat2 = Matrix3x3::empty();
        let mut idx_y = 0;
        for i in 0..3usize {
            if idx_y == row {
                idx_y += 1;
            }
            let mut idx_x = 0;
            for j in 0..3usize {
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


impl<T: Ring + core::ops::Add<Output = T>+ core::ops::Mul<Output = T>> core::ops::Add for Matrix4x4<T> {
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


impl<T: Ring + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::ops::Mul<Output = T>> core::ops::Sub for Matrix4x4<T> {
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
impl<T: Ring + core::ops::Mul<Output = T>> core::ops::Mul<T> for Matrix4x4<T> {
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

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Add<Output = T>> core::ops::Mul<Vector4<T>> for Matrix4x4<T>  {
    /// # Multiplying [`Matrix4x4`] with [`Vector4`]
    /// 
    /// when you multiply a [`Matrix4x4`] with a [`Vector4`] we treat the vector
    /// as a 1x4 matrix * 4x4 matrix.
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4::<T>::new(
            self.x.x() * rhs.x() + self.y.x() * rhs.y() + self.z.x() * rhs.z() + self.w.x() * rhs.w(),
            self.x.y() * rhs.x() + self.y.y() * rhs.y() + self.z.y() * rhs.z() + self.w.y() * rhs.w(),
            self.x.z() * rhs.x() + self.y.z() * rhs.y() + self.z.z() * rhs.z() + self.w.z() * rhs.w(),
            self.x.w() * rhs.x() + self.y.w() * rhs.y() + self.z.w() * rhs.z() + self.w.w() * rhs.w()
        )
    }
    type Output = Vector4<T>;
}
impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Add<Output = T>> core::ops::Mul<Matrix4x4<T>> for Vector4<T>  {
    /// # Multiplying [`Vector4`] with [`Matrix4x4`]
    /// 
    /// when you multiply a [`Vector4`] with a [`Matrix4x4`] we treat the vector
    /// as a 1x4 matrix * 4x4 matrix.
    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        Vector4::<T>::new(
            rhs.x.x() * self.x() + rhs.x.y() * self.y() + rhs.x.z() * self.z() + rhs.x.w() * self.w(),
            rhs.y.x() * self.x() + rhs.y.y() * self.y() + rhs.y.z() * self.z() + rhs.y.w() * self.w(),
            rhs.z.x() * self.x() + rhs.z.y() * self.y() + rhs.z.z() * self.z() + rhs.z.w() * self.w(),
            rhs.w.x() * self.x() + rhs.w.y() * self.y() + rhs.w.z() * self.z() + rhs.w.w() * self.w()
        )
    }
    type Output = Vector4<T>;
}

impl<T: Ring + core::ops::Mul<Output = T> + core::ops::Div<Output = T> + core::ops::Neg<Output = T> + core::ops::Add<Output = T> + core::ops::Sub<Output = T>> Inverse<Multiplication> for Matrix4x4<T> {
    fn inverse(self) -> Self {
        self.adjoint()*(<T as Identity<Multiplication>>::IDENTITY/self.determinant())
    }
}

impl<T: Ring + core::ops::Mul<Output = T>> Matrix for Matrix4x4<T> {
    type Vector = Vector4<T>;
    type TransposeMatrix = Self;
   fn transpose(&self) -> Self {
        Self::new(
            self.get_x_row(),
            self.get_y_row(),
            self.get_z_row(),
            self.get_w_row(),
        )
    }
}

impl<T: Ring> Matrix4x4<T> {
    pub const fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }
    /// When dealing with linear transformations such as these, the
    /// order in which you multiply matters. In this case, if you
    /// want to translate an object, after using this method, then
    /// you should multiply the matrix in the lefthand side.
    /// ``` no_run
    /// let matrix = Matrix4x4::from_translation(Vector4::new(10, 5, 3, 1));
    /// let vector = Vector4::new(5, 2, 4, 1);
    /// assert!(matrix*value==Vector4::new(15, 7, 7, 1));
    /// ```
    pub const fn from_translation(v: Vector4<T>) -> Self {
        Self::new(
            Vector4::right(),
            Vector4::top(),
            Vector4::forward(),
            v
        )
    }
    pub fn perspective(fov: T, aspect: T, far: T, near: T) -> Self 
        where T: FloatingPoint {
        let inv_length = (near-far).recip();
        let half_fov_tan = fov.mul(T::from_f64(0.5)).tan().recip();
        let a = half_fov_tan / aspect;
        let b = (near + far) * inv_length;
        let c = T::from_f64(2.0)*near*far*inv_length;
        Self::new(
            Vector4::new(a, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, half_fov_tan, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, b, <T as Identity<Multiplication>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, c, <T as Identity<Addition>>::IDENTITY), 
        )
        // Self::new(
        //     Vector4::new(
        //         (aspect*half_fov_tan).recip(), 
        //         <T as Identity<Addition>>::IDENTITY, 
        //         <T as Identity<Addition>>::IDENTITY, 
        //         <T as Identity<Addition>>::IDENTITY, 
        //     ), 
        //     Vector4::new(
        //         <T as Identity<Addition>>::IDENTITY, 
        //         half_fov_tan.recip(), 
        //         <T as Identity<Addition>>::IDENTITY, 
        //         <T as Identity<Addition>>::IDENTITY, 
        //     ), 
        //     Vector4::new(
        //         <T as Identity<Addition>>::IDENTITY, 
        //         <T as Identity<Addition>>::IDENTITY, 
        //         (far)*inv_length, 
        //         <T as Identity<Multiplication>>::IDENTITY, 
        //     ), 
        //     Vector4::new(
        //         <T as Identity<Addition>>::IDENTITY, 
        //         <T as Identity<Addition>>::IDENTITY, 
        //         -(far*near).div(far_near_diff), 
        //         <T as Identity<Addition>>::IDENTITY, 
        //     )
        // )
    }
    pub fn perspective_view(position: Vector3<T>, rotation: Vector3<T>) -> Self 
        where T: BasicField + FloatingPoint {
        let c3 = rotation.z().cos();
        let s3 = rotation.z().sin();
        let c2 = rotation.x().cos();
        let s2 = rotation.x().sin();
        let c1 = rotation.y().cos();
        let s1 = rotation.y().sin();
        let u = Vector3::new(c1 * c3 + s1 * s2 * s3, T::from_f64(2.0) * s3, c1 * s2 * s3 - c3 * s1);
        let v = Vector3::new(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = Vector3::new(c2 * s1, -s2, c1 * c2);
        Self::new(
            Vector4::new(u.x, u.y, u.z, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(v.x, v.y, v.z, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(w.x, w.y, w.z, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(-u.inner_product(position), -v.inner_product(position), -w.inner_product(position), <T as Identity<Multiplication>>::IDENTITY)
        )
    }
    pub const fn from_diagonal(diagonal: Vector4<T>) -> Self {
        Self::new(
            Vector4::new(diagonal.x, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, diagonal.y, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, diagonal.z, <T as Identity<Addition>>::IDENTITY), 
            Vector4::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, diagonal.w), 
        )
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
    pub const fn set_w_row(&mut self, row: Vector4<T>) {
        self.x.w = row.x;
        self.y.w = row.y;
        self.z.w = row.z;
        self.w.w = row.w;
    }
    #[inline]
    pub const fn get_x_row(&self) -> Vector4<T> {
        Vector4::new(self.x.x, self.y.x, self.z.x, self.w.x)
    }
    #[inline]
    pub const fn get_y_row(&self) -> Vector4<T> {
        Vector4::new(self.x.y, self.y.y, self.z.y, self.w.y)
    }
    #[inline]
    pub const fn get_z_row(&self) -> Vector4<T> {
        Vector4::new(self.x.z, self.y.z, self.z.z, self.w.z)
    }
    #[inline]
    pub const fn get_w_row(&self) -> Vector4<T> {
        Vector4::new(self.x.w, self.y.w, self.z.w, self.w.w)
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
    pub fn derivative_matrix() -> Self 
        where T: FromPrimitive {
        Matrix4x4::new(
            Vector4::new(T::from_u32(0), T::from_u32(0), T::from_u32(0), T::from_u32(0)), 
            Vector4::new(T::from_u32(0), T::from_u32(1), T::from_u32(0), T::from_u32(0)),
            Vector4::new(T::from_u32(0), T::from_u32(0), T::from_u32(2), T::from_u32(0)),
            Vector4::new(T::from_u32(0), T::from_u32(0), T::from_u32(0), T::from_u32(3)),
        )
    }
}
impl<T: Ring + ClosedUnder<Addition>> ClosedUnder<Addition> for Matrix4x4<T> {
    
}
impl<T: Ring + ClosedUnder<Subtraction>> ClosedUnder<Subtraction> for Matrix4x4<T> {
    
}
impl<T: Ring + ClosedUnder<Multiplication>> ClosedUnder<Multiplication> for Matrix4x4<T> {
    
}
impl<T: Ring + AssociativeOver<Addition>> AssociativeOver<Addition> for Matrix4x4<T> {
    
}
impl<T: Ring + CommutativeOver<Addition>> CommutativeOver<Addition> for Matrix4x4<T> {

}

impl<T: Ring> Identity<Multiplication> for Matrix4x4<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector4<T> as Identity<Multiplication>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Multiplication>>::is_identity(&self.x.x) && 
        <T as Identity<Multiplication>>::is_identity(&self.y.y) && 
        <T as Identity<Multiplication>>::is_identity(&self.z.z) && 
        <T as Identity<Multiplication>>::is_identity(&self.w.w) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.z) && 
        <T as Identity<Addition>>::is_identity(&self.y.w) && 
        <T as Identity<Addition>>::is_identity(&self.x.y) &&
        <T as Identity<Addition>>::is_identity(&self.x.z) &&
        <T as Identity<Addition>>::is_identity(&self.x.w) &&
        <T as Identity<Addition>>::is_identity(&self.z.w) &&
        <T as Identity<Addition>>::is_identity(&self.z.y) &&
        <T as Identity<Addition>>::is_identity(&self.z.x) &&
        <T as Identity<Addition>>::is_identity(&self.w.x) &&
        <T as Identity<Addition>>::is_identity(&self.w.y) &&
        <T as Identity<Addition>>::is_identity(&self.w.z) 
    }
}
impl<T: Ring> Identity<Addition> for Matrix4x4<T> {
    const IDENTITY: Self = Self::from_diagonal(<Vector4<T> as Identity<Addition>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Addition>>::is_identity(&self.x.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.y) && 
        <T as Identity<Addition>>::is_identity(&self.z.z) && 
        <T as Identity<Addition>>::is_identity(&self.w.w) && 
        <T as Identity<Addition>>::is_identity(&self.y.x) && 
        <T as Identity<Addition>>::is_identity(&self.y.z) && 
        <T as Identity<Addition>>::is_identity(&self.y.w) && 
        <T as Identity<Addition>>::is_identity(&self.x.y) &&
        <T as Identity<Addition>>::is_identity(&self.x.z) &&
        <T as Identity<Addition>>::is_identity(&self.x.w) &&
        <T as Identity<Addition>>::is_identity(&self.z.w) &&
        <T as Identity<Addition>>::is_identity(&self.z.y) &&
        <T as Identity<Addition>>::is_identity(&self.z.x) &&
        <T as Identity<Addition>>::is_identity(&self.w.x) &&
        <T as Identity<Addition>>::is_identity(&self.w.y) &&
        <T as Identity<Addition>>::is_identity(&self.w.z) 
    }
}

pub type I8Mat4 = Matrix4x4<i8>;
pub type I16Mat4 = Matrix4x4<i16>;
pub type I32Mat4 = Matrix4x4<i32>;
pub type I64Mat4 = Matrix4x4<i64>;
pub type I128Mat4 = Matrix4x4<i128>;
pub type U8Mat4 = Matrix4x4<u8>;
pub type U16Mat4 = Matrix4x4<u16>;
pub type U32Mat4 = Matrix4x4<u32>;
pub type U64Mat4 = Matrix4x4<u64>;
pub type U128Mat4 = Matrix4x4<u128>;
pub type FMat4 = Matrix4x4<f32>;
pub type DMat4 = Matrix4x4<f64>;