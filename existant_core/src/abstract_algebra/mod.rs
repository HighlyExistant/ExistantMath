mod properties;
mod util;
mod addition;
pub use properties::*;
pub use util::*;
pub use addition::*;

/// Algebraic structure which denotes a set (which is the implementor)
/// that is closed under an operator ⨁.
pub trait Groupoid<Op: Operator>: ClosedUnder<Op> + Copy {
    fn op(&self, rhs: &Self) -> Self;
}

/// [`Groupoid`] where the operator ⨁ is associative.
pub trait Semigroup<Op: Operator>: 
    Groupoid<Op> + 
    AssociativeOver<Op> {
    
}

impl<Op: Operator, T> Semigroup<Op> for T
    where T: 
    Groupoid<Op> + 
    AssociativeOver<Op>{
    
}

/// [`Semigroup`] where the operator ⨁ contains an identity.
pub trait Monoid<Op: Operator>: 
    Semigroup<Op> + 
    Identity<Op> {
    
}

impl<Op: Operator, T> Monoid<Op> for T
    where T: 
    Semigroup<Op> + 
    Identity<Op> {
    
}
/// A [`Monoid`] where the operator ⨁ is commutative
pub trait CommutativeMonoid<Op: Operator>:
    Monoid<Op> +
    CommutativeOver<Op> {
    
}

impl<Op: Operator, T> CommutativeMonoid<Op> for T
    where T: 
    Monoid<Op> +
    CommutativeOver<Op> {
    
}

/// [`Monoid`] where the operator ⨁ has a value `a` for all `b`
/// such that `a ⨁ b` = I where I is the identity.
pub trait Group<Op: Operator>: 
    Monoid<Op> + 
    Identity<Op> + 
    Sized + 
    Inverse<Op> {
    
}

impl<Op: Operator, T> Group<Op> for T
    where T: 
    Monoid<Op> + 
    Identity<Op> +
    Inverse<Op> {
    
}

/// [`Group`] where the operator ⨁ is commutative.
pub trait AbelianGroup<Op: Operator>: 
    Group<Op> + 
    CommutativeOver<Op> {
    
}

impl<Op: Operator, T> AbelianGroup<Op> for T
    where T: 
    Group<Op> + 
    CommutativeOver<Op> {
    
}

/// [`Semigroup`] with another operator * such that * is also a [`Semigroup`] 
/// under this set and is distributive over ⨁. 
pub trait Semiring<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>:
    CommutativeMonoid<Op> + 
    Monoid<DistributiveOp> +
    Identity<DistributiveOp> + 
    Distributive<DistributiveOp, Op> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T> Semiring<DistributiveOp, Op> for T
    where T: 
    CommutativeMonoid<Op> + 
    Monoid<DistributiveOp> +
    Identity<DistributiveOp> + 
    Distributive<DistributiveOp, Op> {
    
}

/// [`AbelianGroup`] with another operator * such that * is a [`Semigroup`] 
/// under this set and is distributive over ⨁. 
pub trait Rong<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>:
    AbelianGroup<Op> +
    Semigroup<DistributiveOp> + 
    Distributive<DistributiveOp, Op> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T> Rong<DistributiveOp, Op> for T
    where T: 
    AbelianGroup<Op> + 
    Semigroup<DistributiveOp> + 
    Distributive<DistributiveOp, Op> {
    
}

/// [`Rong`] where * has an identity.
pub trait Ring<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: 
    Rong<DistributiveOp, Op> + 
    Identity<DistributiveOp> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T> Ring<DistributiveOp, Op> for T
    where T: 
    Rong<DistributiveOp, Op> + 
    Identity<DistributiveOp> {
    
}

/// [`Ring`] such that * is commutative.
pub trait CommutativeRing<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: 
    Ring<DistributiveOp, Op> + 
    CommutativeOver<DistributiveOp> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T> CommutativeRing<DistributiveOp, Op> for T
    where T: 
    Ring<DistributiveOp, Op> + 
    CommutativeOver<DistributiveOp> {
    
}

/// [`CommutativeRing`] where the operator * has a value `a` for all `b`
/// such that `a * b` = I where I is the identity.
pub trait Field<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: 
    CommutativeRing<DistributiveOp, Op> +
    Inverse<DistributiveOp> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T> Field<DistributiveOp, Op> for T
    where T: 
    CommutativeRing<DistributiveOp, Op> +
    Inverse<DistributiveOp>{
    
}
/// Represents an n dimensional tuple of scalars which correspond to a [`Semiring`], 
/// and has a distributive scalar multiplication operation ×, such that for semimodules 
/// `B` and `C`, and scalar `a`, then `a × (B + C) = aB + aC`.
pub trait Semimodule<DistributiveOp: Operator = Multiplication, Op: Operator = Addition> {
    type Scalar: Semiring<DistributiveOp, Op>;
    fn scalar_multiplication(&self, rhs: Self::Scalar) -> Self;
}

/// A [`Semimodule`] such that it's scalars are a [`Ring`]. 
pub trait Module<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: Semimodule<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Ring<DistributiveOp, Op> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T: Semimodule<DistributiveOp, Op>> Module<DistributiveOp, Op> for T 
    where T::Scalar: Ring<DistributiveOp, Op> {

}

/// A [`Module`] such that it's scalars are a [`Field`]. This is the space 
/// usually dealt with in areas such as Linear Algebra.
pub trait VectorSpace<DistributiveOp: Operator, Op: Operator>: Semimodule<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    
}

impl<DistributiveOp: Operator, Op: Operator, T: Module<DistributiveOp, Op>> VectorSpace<DistributiveOp, Op> for T 
    where T::Scalar: Field<DistributiveOp, Op> {

}