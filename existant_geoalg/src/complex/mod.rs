use existant_core::{Absorption, Addition, AssociativeOver, BasicField, ClosedUnder, CommutativeOver, Distributive, Identity, Inverse, Multiplication, Operator};

use crate::vectors::{Vector2};
/// Represents a complex number with 
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
    const BASIS_X: Vector2<T> = Vector2::right();
    const BASIS_Y: Vector2<T> = Vector2::top();
    pub const fn new(real: T, imaginary: T) -> Self {
        Self { inner: Vector2::new(real, imaginary) }
    }
    #[inline]
    pub const fn real(&self) -> T {
        self.inner.x
    }
    #[inline]
    pub const fn imaginary(&self) -> T {
        self.inner.y
    }
}

impl<T: BasicField + core::ops::Add<Output = T>> core::ops::Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.x.add(rhs.inner.x), self.inner.y.add(rhs.inner.y))
    }
}
impl<T: BasicField + core::ops::Mul<Output = T>> core::ops::Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.inner.x*rhs.inner.x - self.inner.y*rhs.inner.y,
            self.inner.x*rhs.inner.y + rhs.inner.x*self.inner.y,
        )
    }
}