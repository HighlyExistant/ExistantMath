mod rays;
mod hypercubes;
mod hypersphere;
mod segments;
mod shape;
use existant_core::{Field, Semimodule, VectorSpace};
pub use rays::*;
pub use hypercubes::*;
pub use hypersphere::*;
pub use segments::*;
pub use shape::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonOrdering {
    Clockwise,
    CounterClockwise
}
pub trait Polygon {
    type Vertex: Semimodule;
}
/// Represents the shape of an object as a list of vertices
/// and indices to those vertices. Every 3 vertices forms a
/// triangle.
pub trait VertexShape: Polygon {
    fn vertices(&self) -> Vec<Self::Vertex>;
    /// Returns a list of indices, to be used alongside the 
    /// returned vertices.
    fn indices(&self, ordering: PolygonOrdering) -> Vec<u32>;
}
pub trait Centroid: Polygon {
    fn center(&self) -> Self::Vertex;
}