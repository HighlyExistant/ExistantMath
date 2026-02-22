mod segment2d;
use existant_core::{BasicField, Field, VectorSpace};
pub use segment2d::*;

pub trait Segment<V: VectorSpace> 
    where V::Scalar: BasicField {
    fn point(&self, t: V::Scalar) -> V;
    /// Represents the order of the curve
    fn order(&self) -> usize;
    /// Gives the starting point of the [`Segment`]
    fn start(&self) -> V;
    /// Gives the end point of the [`Segment`]
    fn end(&self) -> V;
}