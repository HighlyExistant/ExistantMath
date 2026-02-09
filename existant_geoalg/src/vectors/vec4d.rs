use std::ops::{Index, IndexMut};

use existant_core::{Absorption, Addition, AssociativeOver, BasicField, ClosedUnder, CommutativeOver, Distributive, FloatingPoint, Groupoid, Identity, Inverse, Multiplication, Operator, Semimodule, Semiring};

use crate::vectors::{InnerProductSpace, MetricSpace, NormedVectorSpace};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Index<usize> for Vector4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let slice = self.as_slice();
        &slice[index]
    }
}
impl<T> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice = self.as_mut_slice();
        &mut slice[index]
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{}, {}, {}, {}>", self.x, self.y, self.z, self.w))
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .entry(&self.z)
            .entry(&self.w)
            .finish()
    }
}

impl<Op: Operator, T: ClosedUnder<Op>> ClosedUnder<Op> for Vector4<T> {}
impl<Op: Operator, T: Absorption<Op>> Absorption<Op> for Vector4<T> {
    const ABSORBING: Self = Self::new(T::ABSORBING, T::ABSORBING, T::ABSORBING, T::ABSORBING);
    fn is_absorber(&self) -> bool {
        self.x.is_absorber() && 
        self.y.is_absorber() &&
        self.z.is_absorber() &&
        self.w.is_absorber() 
    }
}
impl<Op: Operator, T: AssociativeOver<Op>> AssociativeOver<Op> for Vector4<T> {}
impl<Op: Operator, T: CommutativeOver<Op>> CommutativeOver<Op> for Vector4<T> {}
impl<DistributiveOp: Operator, Op: Operator, T: Distributive<DistributiveOp, Op>> Distributive<DistributiveOp, Op> for Vector4<T> {}
impl<Op: Operator, T: Inverse<Op>> Inverse<Op> for Vector4<T> {
    fn inverse(self) -> Self {
        Self::new(self.x.inverse(), self.y.inverse(), self.z.inverse(), self.w.inverse())
    }
}
impl<Op: Operator, T: Identity<Op>> Identity<Op> for Vector4<T> {
    const IDENTITY: Self = Self::new(T::IDENTITY, T::IDENTITY, T::IDENTITY, T::IDENTITY);
    fn is_identity(&self) -> bool {
        self.x.is_identity() && 
        self.y.is_identity() && 
        self.z.is_identity() && 
        self.w.is_identity()
    }
}

impl<DistributiveOp: Operator, Op: Operator, T: Semiring<DistributiveOp, Op>> Semimodule<DistributiveOp, Op> for Vector4<T> {
    type Scalar = T;
    fn scalar_multiplication(&self, rhs: Self::Scalar) -> Self {
        Self::new(
            <T as Groupoid<DistributiveOp>>::op(&self.x, &rhs), 
            <T as Groupoid<DistributiveOp>>::op(&self.y, &rhs),
            <T as Groupoid<DistributiveOp>>::op(&self.z, &rhs),
            <T as Groupoid<DistributiveOp>>::op(&self.w, &rhs),
        )
    }
}

impl<T: BasicField + FloatingPoint> InnerProductSpace for Vector4<T> {
    fn inner_product(&self, other: Self) -> Self::Scalar {
        self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w
    }
}

impl<T: BasicField + FloatingPoint> NormedVectorSpace for Vector4<T> {
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.x/magnitude, self.y/magnitude, self.z/magnitude, self.w/magnitude)
    }
    fn magnitude(&self) -> Self::Scalar {
        self.squared_length().sqrt()
    }
}

// impl<T: BasicField + FloatingPoint> GrassmanAlgebra for Vector4D<T> {
//     type Bivector = Vector4D<T>;
//     /// The 3d wedge product returns a vector which is perpendicular
//     /// to the 2 vectors provided.
//     fn wedge_product(&self, rhs: Self) -> Self::Bivector {
//         Self::new(
//             self.x*rhs.y - self.y*rhs.x,
//             self.z*rhs.x - self.x*rhs.z,
//             self.y*rhs.z - self.z*rhs.y,
//         )
//     }
// }

impl<T: BasicField + FloatingPoint> MetricSpace for Vector4<T> {
    type Distance = T;
    fn distance(&self, other: Self) -> Self::Distance {
        (self.clone() - other).magnitude()
    }
}

// impl<T: BasicField + FloatingPoint> GeometricAlgebra for Vector4D<T> {
//     fn geometric_product(&self, other: Self) -> (Self::Scalar, Self::Bivector) {
//         (self.inner_product(other), self.wedge_product(other))
//     }
// }

impl<T> Vector4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    pub const fn x(&self) -> T 
        where T: Copy {
        self.x
    }
    pub const fn y(&self) -> T 
        where T: Copy {
        self.y
    }
    
    pub const fn z(&self) -> T 
        where T: Copy {
        self.z
    }
    pub const fn w(&self) -> T 
        where T: Copy {
        self.w
    }
    
    /// Returns a vector pointing to the right of the graph <1, 0>
    pub const fn right() -> Self 
        where T: Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <-1, 0>
    pub fn left() -> Self 
        where T: core::ops::Neg<Output = T> + Identity<Multiplication> + Identity<Addition> {
        Self::new(-<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <0, 1>
    pub const fn top() -> Self 
        where T: Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <0, -1>
    pub fn bottom() -> Self 
        where T: core::ops::Neg<Output = T> + Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, -<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to forward of the graph <0, 0, 1>
    pub const fn forward() -> Self 
        where T: Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to backward of the graph <0, 0, -1>
    pub fn backward() -> Self 
        where T: core::ops::Neg<Output = T> + Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, -<T as Identity<Addition>>::IDENTITY, -<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        4
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, self.len()) }
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, self.len()) }
    }
}

impl<T: Semiring<Multiplication, Addition>> core::ops::Mul<T> for Vector4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_multiplication(rhs)
    }
}
impl<T: core::ops::Add<Output = T>> core::ops::Add for Vector4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x.add(rhs.x), self.y.add(rhs.y), self.z.add(rhs.z), self.w.add(rhs.w))
    }
}
impl<T: core::ops::Sub<Output = T>> core::ops::Sub for Vector4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x.sub(rhs.x), self.y.sub(rhs.y), self.z.sub(rhs.z), self.w.sub(rhs.w))
    }
}
impl<T: core::ops::Mul<Output = T>> core::ops::Mul for Vector4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x.mul(rhs.x), self.y.mul(rhs.y), self.z.mul(rhs.z), self.w.mul(rhs.w))
    }
}
impl<T: core::ops::Div<Output = T>> core::ops::Div for Vector4<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x.div(rhs.x), self.y.div(rhs.y), self.z.div(rhs.z), self.w.div(rhs.w))
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}
