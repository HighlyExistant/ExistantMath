use existant_core::{Addition, Field, Multiplication, Number, Operator, Semimodule, VectorSpace};

mod vec2d;
mod vec3d;
mod vec4d;
pub use vec2d::*;
pub use vec3d::*;
pub use vec4d::*;

pub struct ScalarMultiplication;
impl Operator for ScalarMultiplication {}

pub trait InnerProductSpace<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: NormedVectorSpace<DistributiveOp, Op>
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    fn inner_product(&self, other: Self) -> Self::Scalar;
    fn squared_length(self) -> Self::Scalar  
        where Self: Sized + Copy {
        self.inner_product(self)
    }
}

pub trait NormedVectorSpace<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: VectorSpace<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    fn normalize(&self) -> Self;
    fn magnitude(&self) -> Self::Scalar;
}

pub trait GrassmanAlgebra<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: VectorSpace<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    type Bivector;
    fn wedge_product(&self, rhs: Self) -> Self::Bivector;
}

pub trait GeometricAlgebra<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: GrassmanAlgebra<DistributiveOp, Op> + 
    InnerProductSpace<DistributiveOp, Op>
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    fn geometric_product(&self, other: Self) -> (Self::Scalar, Self::Bivector);
}

pub struct Euclidean;

pub trait MetricSpace<Metric = Euclidean, T = Self> {
    type Distance;
    fn distance(&self, other: T) -> Self::Distance;
}

impl<T: Number> MetricSpace for T {
    type Distance = T;
    fn distance(&self, other: Self) -> Self::Distance {
        self.clone() - other
    }
}