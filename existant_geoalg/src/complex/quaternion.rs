use std::ops::Mul;

use existant_core::BasicField;

use crate::{complex::Complex, vectors::Vector4};

/// Represents a 4 dimensional complex object, which represents
/// the equation:
/// ```
/// i*i=j*j=k*k=ijk=-1
/// ```
/// Multiplication between quaternions and quaternions is
/// represented by the matrix:
/// ```
/// ┌1,  i,  j,  k┐
/// │i, -1,  k, -j│
/// │k, -k, -1,  i│
/// └k,  j, -i, -1┘
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
    pub fn i(&self) -> T {
        self.inner.y
    }
    pub fn j(&self) -> T {
        self.inner.z
    }
    pub fn k(&self) -> T {
        self.inner.w
    }
}

impl<T: BasicField> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.inner.x*rhs.inner.x - self.inner.y*rhs.inner.y - self.inner.z * rhs.inner.z - self.inner.w * rhs.inner.w, 
            self.inner.x*rhs.inner.y + self.inner.y*rhs.inner.x + self.inner.z*rhs.inner.w - self.inner.w*rhs.inner.z, 
            self.inner.x*rhs.inner.z - self.inner.y*rhs.inner.w + self.inner.z*rhs.inner.x + self.inner.w*rhs.inner.y, 
            self.inner.x*rhs.inner.w + self.inner.y*rhs.inner.z - self.inner.z*rhs.inner.y + self.inner.w*rhs.inner.x
        )
    }
}

impl<T: BasicField> Mul<T> for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.r()*rhs,
            self.i()*rhs,
            self.j()*rhs,
            self.k()*rhs,
        )
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