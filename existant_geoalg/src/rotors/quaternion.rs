use std::ops::Mul;

use existant_core::{BasicField, FloatingPoint, Semimodule};

use crate::{rotors::Complex, matrix::Matrix4x4, vectors::{InnerProductSpace, NormedVectorSpace, Vector3, Vector4}};

/// Represents a 4 dimensional complex object, which represents
/// the equation:
/// ```
/// i*i=j*j=k*k=ijk=-1
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Quaternion<T: BasicField> {
    inner: Vector4<T>,
}

impl<T: BasicField + core::fmt::Display> core::fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} + {}i + {}j + {}k", self.inner.x, self.inner.y, self.inner.z, self.inner.w))
    }
}

impl<T: BasicField + core::fmt::Debug> core::fmt::Debug for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.inner.x)
            .entry(&self.inner.y)
            .entry(&self.inner.z)
            .entry(&self.inner.w)
            .finish()
    }
}

impl<T: BasicField> Quaternion<T> {
    pub const fn new(scalar: T, i: T, j: T, k: T) -> Self {
        Self { inner: Vector4::new(scalar, i, j, k) }
    }
    /// Returns the real part of the Quaternion
    pub fn r(&self) -> T {
        self.inner.x
    }
    /// Returns the i'th imaginary component
    pub fn i(&self) -> T {
        self.inner.y
    }
    /// Returns the j'th imaginary component
    pub fn j(&self) -> T {
        self.inner.z
    }
    /// Returns the k'th imaginary component
    pub fn k(&self) -> T {
        self.inner.w
    }
    pub fn from_angle(axis: Vector3<T>, radians: T) -> Self 
        where T: FloatingPoint {
        let (c, s) = radians.mul(T::from_f64(0.5)).sin_cos();
        let norm = axis.normalize()*s;
        Self::new(c, norm.x, norm.y, norm.z)
    }
    pub fn conjugate(self) -> Self {
        Self::new(self.r(), -self.i(), -self.j(), -self.k())
    }
}

impl<T: BasicField> Semimodule for Quaternion<T> {
    type Scalar = T;
    fn scalar_multiplication(&self, rhs: Self::Scalar) -> Self {
        Self::new(self.r()*rhs, self.i()*rhs, self.j()*rhs, self.k()*rhs)
    }
}

impl<T: BasicField + FloatingPoint> NormedVectorSpace for Quaternion<T> {
    fn magnitude(&self) -> Self::Scalar {
        self.squared_length().sqrt()
    }
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.r()/magnitude, self.i()/magnitude, self.j()/magnitude, self.k()/magnitude)
    }
}

impl<T: BasicField + FloatingPoint> InnerProductSpace for Quaternion<T> {
    fn inner_product(&self, other: Self) -> Self::Scalar {
        self.inner.inner_product(other.inner)
    }
}
impl<T: BasicField> From<Quaternion<T>> for Matrix4x4<T> {
    /// ```
    /// ┌ 1, i, j, k┐
    /// │-i, 1,-k, j│
    /// │-j, k, 1,-i│
    /// └-k,-j, i, 1┘
    /// ```
    fn from(value: Quaternion<T>) -> Self {
        Matrix4x4::new(
            Vector4::new(value.r(),  -value.i(),  -value.j(),  -value.k()), 
            Vector4::new(value.i(), value.r(), value.k(),  -value.j()), 
            Vector4::new(value.j(),  -value.k(), value.r(), value.i()), 
            Vector4::new(-value.k(), -value.j(),  value.i(), value.r())
        )
    }
}

impl<T: BasicField> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.inner.x*rhs.inner.x - self.inner.y*rhs.inner.y - self.inner.z*rhs.inner.z - self.inner.w*rhs.inner.w, 
            self.inner.x*rhs.inner.y + self.inner.y*rhs.inner.x + self.inner.z*rhs.inner.w - self.inner.w*rhs.inner.z, 
            self.inner.x*rhs.inner.z - self.inner.y*rhs.inner.w + self.inner.z*rhs.inner.x + self.inner.w*rhs.inner.y, 
            self.inner.x*rhs.inner.w + self.inner.y*rhs.inner.z - self.inner.z*rhs.inner.y + self.inner.w*rhs.inner.x
        )
    }
}

impl<T: BasicField> Mul<T> for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_multiplication(rhs)
    }
}

impl<T: BasicField> Mul<Complex<T>> for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Self::new(
            self.inner.x*rhs.r() - self.inner.y*rhs.i(),
            self.inner.x*rhs.i() + self.inner.w*rhs.i(),
            self.inner.z*rhs.r() + self.inner.w*rhs.i(),
            self.inner.w*rhs.r() - self.inner.z*rhs.i(),
        )
    }
}

impl<T: BasicField> From<Vector4<T>> for Quaternion<T> {
    fn from(value: Vector4<T>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl<T: BasicField> Mul<Vector4<T>> for Quaternion<T> {
    type Output = Quaternion<T>;
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        self*Quaternion::from(rhs)
    }
}