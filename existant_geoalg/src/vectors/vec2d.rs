use std::ops::{Index, IndexMut};

use bytemuck::{Pod, Zeroable};
use existant_core::{Absorption, Addition, AssociativeOver, BasicField, Bounds, ClosedUnder, CommutativeOver, Distributive, Field, FloatingPoint, FromPrimitive, Groupoid, Identity, Inverse, Multiplication, Operator, Semimodule, Semiring};

use crate::{rotors::Complex, vectors::{GeometricAlgebra, GrassmanAlgebra, InnerProductSpace, MetricSpace, NormedVectorSpace}};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

unsafe impl<T: Identity<Addition>> Zeroable for Vector2<T> {
    fn zeroed() -> Self {
        Self::new(<T as Identity<Addition>>::IDENTITY,<T as Identity<Addition>>::IDENTITY)
    }
}
unsafe impl<T: Identity<Addition> + Copy + 'static> Pod for Vector2<T> {

}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let slice = self.as_slice();
        &slice[index]
    }
}
impl<T> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice = self.as_mut_slice();
        &mut slice[index]
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{}, {}>", self.x, self.y))
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .finish()
    }
}

impl<Op: Operator, T: ClosedUnder<Op>> ClosedUnder<Op> for Vector2<T> {}
impl<Op: Operator, T: Absorption<Op>> Absorption<Op> for Vector2<T> {
    const ABSORBING: Self = Self::new(T::ABSORBING, T::ABSORBING);
    fn is_absorber(&self) -> bool {
        self.x.is_absorber() && 
        self.y.is_absorber()
    }
}
impl<Op: Operator, T: AssociativeOver<Op>> AssociativeOver<Op> for Vector2<T> {}
impl<Op: Operator, T: CommutativeOver<Op>> CommutativeOver<Op> for Vector2<T> {}
impl<DistributiveOp: Operator, Op: Operator, T: Distributive<DistributiveOp, Op>> Distributive<DistributiveOp, Op> for Vector2<T> {}
impl<Op: Operator, T: Inverse<Op>> Inverse<Op> for Vector2<T> {
    fn inverse(self) -> Self {
        Self::new(self.x.inverse(), self.y.inverse())
    }
}
impl<Op: Operator, T: Identity<Op>> Identity<Op> for Vector2<T> {
    const IDENTITY: Self = Self::new(T::IDENTITY, T::IDENTITY);
    fn is_identity(&self) -> bool {
        self.x.is_identity() && 
        self.y.is_identity()
    }
}

impl<DistributiveOp: Operator, Op: Operator, T: Semiring<DistributiveOp, Op>> Semimodule<DistributiveOp, Op> for Vector2<T> {
    type Scalar = T;
    fn scalar_multiplication(&self, rhs: Self::Scalar) -> Self {
        Self::new(
            <T as Groupoid<DistributiveOp>>::op(&self.x, &rhs), 
            <T as Groupoid<DistributiveOp>>::op(&self.y, &rhs)
        )
    }
}

impl<T: BasicField + FloatingPoint> InnerProductSpace for Vector2<T> {
    fn inner_product(&self, other: Self) -> Self::Scalar {
        self.x*other.x + self.y*other.y
    }
}

impl<T: BasicField + FloatingPoint> NormedVectorSpace for Vector2<T> {
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.x/magnitude, self.y/magnitude)
    }
    fn magnitude(&self) -> Self::Scalar {
        self.squared_length().sqrt()
    }
}

impl<T: BasicField + FloatingPoint> GrassmanAlgebra for Vector2<T> {
    type Bivector = T;
    fn wedge_product(&self, rhs: Self) -> Self::Bivector {
        self.x*rhs.y - self.y*rhs.x
    }
}

impl<T: BasicField + FloatingPoint> MetricSpace for Vector2<T> {
    type Distance = T;
    fn distance(&self, other: Self) -> Self::Distance {
        (self.clone() - other).magnitude()
    }
}

impl<T: BasicField + FloatingPoint> GeometricAlgebra for Vector2<T> {
    fn geometric_product(&self, other: Self) -> (Self::Scalar, Self::Bivector) {
        (self.inner_product(other), self.wedge_product(other))
    }
}

impl<T> Vector2<T> {
    /// Creates a new 2 Dimensional Vector type
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    /// x component of a vector <x, y>
    pub const fn x(&self) -> T 
        where T: Copy {
        self.x
    }
    /// y component of a vector <x, y>
    pub const fn y(&self) -> T 
        where T: Copy {
        self.y
    }
    
    /// Returns a vector pointing to the right of the graph <1, 0>
    pub const fn right() -> Self 
        where T: Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <-1, 0>
    pub fn left() -> Self 
        where T: core::ops::Neg<Output = T> + Identity<Multiplication> + Identity<Addition> {
        Self::new(-<T as Identity<Multiplication>>::IDENTITY, <T as Identity<Addition>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <0, 1>
    pub const fn top() -> Self 
        where T: Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, <T as Identity<Multiplication>>::IDENTITY)
    }
    /// Returns a vector pointing to the left of the graph <0, -1>
    pub fn bottom() -> Self 
        where T: core::ops::Neg<Output = T> + Identity<Multiplication> + Identity<Addition> {
        Self::new(<T as Identity<Addition>>::IDENTITY, -<T as Identity<Multiplication>>::IDENTITY)
    }
    // returns the current vector rotated 90 degrees
    pub fn perpendicular(self) -> Self 
        where T: core::ops::Neg<Output = T> {
        Self::new(-self.y, self.x)
    }
    /// The dimension of the vector type
    #[inline(always)]
    pub const fn len(&self) -> usize {
        2
    }
    /// converts the vector type into a slice
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, self.len()) }
    }
    
    /// converts the vector type into a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, self.len()) }
    }
}

impl<T: BasicField + FloatingPoint> Vector2<T> {
    pub fn cos2(&self, other: Self) -> T {
        self.inner_product(other)/self.magnitude()
    }
    pub fn cos(&self)-> T {
        self.cos2(Self::right())
    }
    pub fn sin2(&self, other: Self)-> T {
        other.wedge_product(*self)/self.magnitude()
    }
    pub fn sin(&self)-> T {
        self.sin2(Self::right())
    }
    pub fn tan(&self)-> T {
        self.y.div(self.x)
    }
    pub fn cot(&self)-> T {
        self.x.div(self.y)
    }
    /// Returns the slope formed by the two points, if
    /// the x directions aren't equal. Returns None otherwise.
    pub fn direction(self, p: Self)-> Option<T> {
        if self.x == p.x {
            return None;
        }
        Some((self-p).tan())
    }
    pub fn angle(&self) -> T {
        self.cos().acos()
    }
    pub fn from_angle(angle: T) -> Self {
        Self::new(angle.cos(), angle.sin())
    }
}

impl<T: Semiring<Multiplication, Addition>> core::ops::Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_multiplication(rhs)
    }
}
impl<T: core::ops::Add<Output = T>> core::ops::Add for Vector2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x.add(rhs.x), self.y.add(rhs.y))
    }
}
impl<T: core::ops::Sub<Output = T>> core::ops::Sub for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x.sub(rhs.x), self.y.sub(rhs.y))
    }
}
impl<T: core::ops::Mul<Output = T>> core::ops::Mul for Vector2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x.mul(rhs.x), self.y.mul(rhs.y))
    }
}
impl<T: core::ops::Div<Output = T>> core::ops::Div for Vector2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x.div(rhs.x), self.y.div(rhs.y))
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T: BasicField> From<Complex<T>> for Vector2<T> {
    fn from(value: Complex<T>) -> Self {
        Self::new(value.r(), value.i())
    }
}

impl<T: BasicField> core::ops::Mul<Complex<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Vector2::from(Complex::from(self)*rhs)
    }
}

impl<T: Bounds> Bounds for Vector2<T> {
    const MIN: Self = Vector2::new(T::MIN, T::MIN);
    const MAX: Self = Vector2::new(T::MAX, T::MAX);
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x), 
            self.y.min(other.y), 
        )
    }
    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x), 
            self.y.max(other.y), 
        )
    }
}

pub type I8Vec2 = Vector2<i8>;
pub type I16Vec2 = Vector2<i16>;
pub type I32Vec2 = Vector2<i32>;
pub type I64Vec2 = Vector2<i64>;
pub type I128Vec2 = Vector2<i128>;
pub type U8Vec2 = Vector2<u8>;
pub type U16Vec2 = Vector2<u16>;
pub type U32Vec2 = Vector2<u32>;
pub type U64Vec2 = Vector2<u64>;
pub type U128Vec2 = Vector2<u128>;
pub type FVec2 = Vector2<f32>;
pub type DVec2 = Vector2<f64>;