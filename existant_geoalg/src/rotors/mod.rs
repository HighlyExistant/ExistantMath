use existant_core::{Absorption, Addition, AssociativeOver, BasicField, ClosedUnder, CommutativeOver, Distributive, FloatingPoint, Identity, Inverse, Multiplication, Operator, Semimodule};

use crate::vectors::{InnerProductSpace, NormedVectorSpace, Vector2};
mod quaternion;
pub use quaternion::*;
/// Represents a complex number a + bi.
/// # Axioms
/// The imaginary part of the complex number
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Complex<T: BasicField> {
    inner: Vector2<T>,
}

impl<T: BasicField + core::fmt::Display> core::fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} + {}i", self.inner.x, self.inner.y))
    }
}

impl<T: BasicField + core::fmt::Debug> core::fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.inner.x)
            .entry(&self.inner.y)
            .finish()
    }
}

impl<T: BasicField> Semimodule for Complex<T> {
    type Scalar = T;
    fn scalar_multiplication(&self, rhs: Self::Scalar) -> Self {
        Self::new(self.r()*rhs, self.i()*rhs)
    }
}

impl<T: BasicField + FloatingPoint> NormedVectorSpace for Complex<T> {
    fn magnitude(&self) -> Self::Scalar {
        self.squared_length().sqrt()
    }
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.r()/magnitude, self.i()/magnitude)
    }
}

impl<T: BasicField + FloatingPoint> InnerProductSpace for Complex<T> {
    fn inner_product(&self, other: Self) -> Self::Scalar {
        self.inner.inner_product(other.inner)
    }
}

impl<Op: Operator, T: BasicField + ClosedUnder<Op>> ClosedUnder<Op> for Complex<T> {}
impl<Op: Operator, T: BasicField + Absorption<Op>> Absorption<Op> for Complex<T> {
    const ABSORBING: Self = Self::new(T::ABSORBING, T::ABSORBING);
    fn is_absorber(&self) -> bool {
        self.inner.x.is_absorber() && 
        self.inner.y.is_absorber()
    }
}
impl<Op: Operator, T: BasicField + AssociativeOver<Op>> AssociativeOver<Op> for Complex<T> {}
impl<Op: Operator, T: BasicField + CommutativeOver<Op>> CommutativeOver<Op> for Complex<T> {}
impl<DistributiveOp: Operator, Op: Operator, T: BasicField + Distributive<DistributiveOp, Op>> Distributive<DistributiveOp, Op> for Complex<T> {}
impl<T: BasicField + Inverse<Addition>> Inverse<Addition> for Complex<T> {
    fn inverse(self) -> Self {
        Self { inner: <Vector2<T> as Inverse<Addition>>::inverse(self.inner) }
    }
}
impl<T: BasicField + Inverse<Multiplication>> Inverse<Multiplication> for Complex<T> {
    fn inverse(self) -> Self {
        Self { inner: <Vector2<T> as Inverse<Multiplication>>::inverse(self.inner) }
    }
}
impl<T: BasicField + Identity<Multiplication>> Identity<Multiplication> for Complex<T> {
    const IDENTITY: Self = Self::new(<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Multiplication>>::is_identity(&self.inner.x) && 
        <T as Identity<Addition>>::is_identity(&self.inner.y)
    }
}
impl<T: BasicField + Identity<Addition>> Identity<Addition> for Complex<T> {
    const IDENTITY: Self = Self::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY);
    fn is_identity(&self) -> bool {
        <T as Identity<Addition>>::is_identity(&self.inner.x) && 
        <T as Identity<Addition>>::is_identity(&self.inner.y)
    }
}

impl<T: BasicField> Complex<T>  {
    pub const fn new(real: T, imaginary: T) -> Self {
        Self { inner: Vector2::new(real, imaginary) }
    }
    #[inline]
    pub const fn r(&self) -> T {
        self.inner.x
    }
    #[inline]
    pub const fn i(&self) -> T {
        self.inner.y
    }
    pub fn conjugate(self) -> Self {
        Self::new(self.r(), -self.i())
    }
    pub fn from_angle(radians: T) -> Self 
        where T: FloatingPoint {
        Self::new(radians.cos(), radians.sin())
    }
}

impl<T: BasicField> core::ops::Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.x.add(rhs.inner.x), self.inner.y.add(rhs.inner.y))
    }
}
impl<T: BasicField> core::ops::Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.x.sub(rhs.inner.x), self.inner.y.sub(rhs.inner.y))
    }
}
impl<T: BasicField> core::ops::Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.inner.x*rhs,
            self.inner.y*rhs,
        )
    }
}
impl<T: BasicField> core::ops::Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.inner.x*rhs.inner.x - self.inner.y*rhs.inner.y,
            self.inner.x*rhs.inner.y + rhs.inner.x*self.inner.y,
        )
    }
}
impl<T: BasicField> core::ops::Mul<Quaternion<T>> for Complex<T> {
    type Output = Quaternion<T>;
    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        Quaternion::new(
            self.r()*rhs.r() - self.i()*rhs.i(), 
            self.r()*rhs.i() + self.i()*rhs.r(), 
            self.r()*rhs.j() - self.i()*rhs.k(), 
            self.r()*rhs.k() + self.i()*rhs.j()
        )
    }
}

impl<T: BasicField> From<Vector2<T>> for Complex<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y)
    }
}