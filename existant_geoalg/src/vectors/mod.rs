use std::ops::{Deref, DerefMut, Sub};

use existant_core::{Addition, BasicField, Bounds, Field, FloatingPoint, FromPrimitive, Identity, Multiplication, Number, Operator, Semimodule, Signed, VectorSpace};

mod vec2d;
mod vec3d;
mod vec4d;
pub use vec2d::*;
pub use vec3d::*;
pub use vec4d::*;

pub trait InnerProductSpace<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: NormedVectorSpace<DistributiveOp, Op>
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    /// Returns the similarity between the two vectors.
    /// # Properties
    /// * The inner product of two perpendicular vectors
    fn inner_product(&self, other: Self) -> Self::Scalar;
    /// Returns the squared length of a [`VectorSpace`].
    /// Getting the squareroot gives you the actual length.
    fn squared_length(self) -> Self::Scalar  
        where Self: Sized + Copy {
        self.inner_product(self)
    }
}

pub trait NormedVectorSpace<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: VectorSpace<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    /// Fixes a vectors length to 1.
    fn normalize(&self) -> Self;
    /// Gets the length of the vector.
    fn magnitude(&self) -> Self::Scalar;
}

pub trait GrassmanAlgebra<DistributiveOp: Operator = Multiplication, Op: Operator = Addition>: VectorSpace<DistributiveOp, Op> 
    where <Self as Semimodule<DistributiveOp, Op>>::Scalar: Field<DistributiveOp, Op> {
    type Bivector;
    /// This will return a bivector with an orientation and area of
    /// a parralelogram formed by the two vectors it utilizes.
    /// # Properties
    /// * The wedge product of two parralel vectors (That is two vectors
    /// with the same direction) is 0.
    /// * The wedge product is anticommutative, that is to say
    /// `x.wedge_product(y) = -y.wedge_product(x)`
    /// * The wedge product is distributive over addition, that is to say
    /// `(x+y).wedge_product(z) = x.wedge_product(z) + y.wedge_product(z)`
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

/// calculates the direction of reflection of an incident vector, where `incident` is the incident vector and
/// `normal` is the normal of the surface it is reflecting on. Important to note that
/// both the incident and normal should be normalized vectors. The following snippet was
/// retrieved from https://thebookofshaders.com/glossary/?search=reflect
pub fn reflect<V: VectorSpace + Copy + std::ops::Sub<Output = V>>(incident: V, normal: V) -> V 
    where V: InnerProductSpace, V::Scalar: BasicField + FromPrimitive {
    incident.clone() - (normal.clone()).scalar_multiplication(V::Scalar::from_f64(2.0)*incident.inner_product(normal))
}
/// calculates the refraction of an incident vector, where `incident` is the incident vector,
/// `normal` is the normal of the surface it is reflecting on and `eta` is the ratio of indices of
/// refraction. Important to note that both the incident and normal should be normalized vectors.
/// retrieved from https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/refraction
pub fn refract<V: VectorSpace + std::ops::Add<Output = V> + std::ops::Mul<Output = V> + std::ops::Neg<Output = V> + Copy>(incident: V, normal: V, eta: V::Scalar) -> V 
    where V: InnerProductSpace + NormedVectorSpace, V::Scalar: BasicField + FloatingPoint {
    // commented code from https://thebookofshaders.com/glossary/?search=refract
    // let ni = normal.dot(incident);
    // let k = V::Scalar::ONE - eta*eta*(V::Scalar::ONE - ni*ni);
    // if k < V::Scalar::ZERO {
    //     V::ZERO
    // } else {
    //     (incident.clone()*eta)-(normal.clone()*(eta*ni + k.sqrt()))
    // }
    let cos_theta: <V as Semimodule<Multiplication, Addition>>::Scalar = (-incident.clone()).inner_product(normal).min(<V::Scalar as Identity<Multiplication>>::IDENTITY);
    let r_out_perp = (incident.clone()+(normal.clone().scalar_multiplication(cos_theta))).scalar_multiplication(eta);
    let r_out_parallel = normal.clone().scalar_multiplication(-(<V::Scalar as Identity<Multiplication>>::IDENTITY.sub(r_out_perp.squared_length()).abs()).sqrt());
    r_out_perp+r_out_parallel
}

pub fn project<V: VectorSpace + InnerProductSpace + Copy>(vector: V, project_onto: V) -> V 
    where V::Scalar: BasicField {
    use core::ops::{Mul, Div};
    project_onto.scalar_multiplication(vector.inner_product(project_onto).div(project_onto.squared_length()))
}