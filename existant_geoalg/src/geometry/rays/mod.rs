mod ray2d;
mod ray3d;
use existant_core::BasicField;
pub use ray2d::*;
pub use ray3d::*;

use crate::vectors::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct RayIntersection<T> {
    pub point: Vector2<T>,
    pub distance: T,
}

pub trait Intersect<T> {
    type Scalar: BasicField;
    fn intersect(&self, with: &T) -> Option<RayIntersection<Self::Scalar>>;
}