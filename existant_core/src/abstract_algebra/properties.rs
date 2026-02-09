use core::ops::{Add, Sub};

use crate::Groupoid;

macro_rules! impl_properties {
    ($structure:tt, $op:tt, $($property:tt),+) => {
        $(
            impl $property<$op> for $structure {

            }
        )+
    };
}
macro_rules! impl_distributive {
    ($op:tt, $over:tt, $($structure:tt),*) => {
        $(
            impl Distributive<$op, $over> for $structure {

            }
        )*
    };
}
macro_rules! impl_inverse_add {
    ($op:tt, $($structure:tt),*) => {
        $(
            impl Inverse<$op> for $structure {
                fn inverse(self) -> Self {
                    -self
                }
            }
        )*
    };
}
macro_rules! impl_inverse_mul {
    ($op:tt, $one:tt, $($structure:tt),*) => {
        $(
            impl Inverse<$op> for $structure {
                fn inverse(self) -> Self {
                    $one/self
                }
            }
        )*
    };
}
macro_rules! impl_ty_properties {
    ($op:tt, $($structure:tt),*) => {
        $(
            impl_properties!($structure, $op, AssociativeOver, CommutativeOver, ClosedUnder);
        )*
    };
    ($op:tt; $($structure:tt),*) => {
        $(
            impl_properties!($structure, $op, AssociativeOver, ClosedUnder);
        )*
    };
}


macro_rules! impl_identity {
    ($zero:tt, $op:tt, $($structure:tt),*) => {
        $(
            impl Identity<$op> for $structure {
                const IDENTITY: Self = $zero;
                fn is_identity(&self) -> bool {
                    *self == $zero
                }
            }
        )*
    };
}

macro_rules! impl_absorption {
    ($absorb:tt, $op:tt, $($structure:tt),*) => {
        $(
            impl Absorption<$op> for $structure {
                const ABSORBING: Self = $absorb;
                fn is_absorber(&self) -> bool {
                    *self == $absorb
                }
            }
        )*
    };
}

pub struct Addition;
pub struct Subtraction;
pub struct Multiplication;
pub struct Division;
/// Denotes a binary operator that can be used
/// over some set. The operators themselves are
/// denoted by sizeless structures, which can be
/// used in conjunction with the various property
/// traits which are:
/// * [`Absorption`]
/// * [`AssociativeOver`]
/// * [`ClosedUnder`]
/// * [`CommutativeOver`]
/// * [`Distributive`]
/// * [`Identity`]
/// * [`Inverse`]
/// 
/// # Warning
/// Don't use any of the property traits, unless it has
/// already been proven that those categories have been
/// satisfied, as the traits dont check whether operations
/// actually satisfy those properties, they are just there
/// as dummies.
pub trait Operator: Sized {
    
}
/// Denotes that an operator ⨁ has the property that
/// a ⨁ (b ⨁ c) = (a ⨁ b) ⨁ c.
pub trait AssociativeOver<Op: Operator> {
    
}
/// Denotes that an operator ⨁ has the property that
/// a ⨁ b = b ⨁ a 
pub trait CommutativeOver<Op: Operator> {
    
}
/// Denotes that an operator ⨁ on a set where this trait
/// is being implemented, will have the property that for all
/// a and b in the trait a ⨁ b = c will also be a value
/// in that set.
pub trait ClosedUnder<Op: Operator> {
    
}
/// Denotes that an operator * over the operator ⨁ will have
/// the following property where c*(a⨁b) = c*a ⨁ c*b.
pub trait Distributive<Distribute: Operator, Over: Operator> {
    
}
/// Denotes that an operator ⨁ has a value in this set which
/// undoes the operation ⨁.
pub trait Inverse<Op: Operator> {
    fn inverse(self) -> Self;
}
/// denotes the value of the set that does nothing under operator 
/// ⨁. a ⨁ IDENTITY = a.
pub trait Identity<Op: Operator> {
    const IDENTITY: Self;
    fn is_identity(&self) -> bool;
}
/// denotes the value of the set that converts it into that very value 
/// under operator ⨁ for all values in the set. a ⨁ ABSORBING = ABSORBING.
pub trait Absorption<Op: Operator> {
    const ABSORBING: Self;
    fn is_absorber(&self) -> bool;
}

impl Operator for Addition {}
impl Operator for Subtraction {}
impl Operator for Multiplication {}
impl Operator for Division {}

impl_ty_properties!(Addition, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_ty_properties!(Multiplication, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_ty_properties!(Subtraction; i8, i16, i32, i64, i128, f32, f64);

impl_inverse_add!(Addition, i8, i16, i32, i64, i128, f32, f64);
impl_inverse_add!(Subtraction, i8, i16, i32, i64, i128, f32, f64);
impl_inverse_mul!(Multiplication, 1.0, f32, f64);


impl ClosedUnder<Division> for f32 {}
impl ClosedUnder<Division> for f64 {}

impl_distributive!(Multiplication, Addition, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_distributive!(Multiplication, Subtraction, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_distributive!(Division, Addition, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_distributive!(Division, Subtraction, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl_identity!(0, Addition, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_identity!(0.0, Addition, f32, f64);
impl_identity!(0, Subtraction, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_identity!(0.0, Subtraction, f32, f64);
impl_identity!(1, Multiplication, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_identity!(1.0, Multiplication, f32, f64);
impl_identity!(1, Division, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_identity!(1.0, Division, f32, f64);

impl_absorption!(0, Multiplication, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_absorption!(0.0, Multiplication, f32, f64);


// impl_semigroup!(u8, u16, u32, u64, u128);
// impl_ring!(i8, i16, i32, i64, i128);
// impl_field!(f32, f64);

