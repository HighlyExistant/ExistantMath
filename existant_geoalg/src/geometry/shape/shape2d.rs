use existant_core::{BasicField, VectorSpace};

use crate::geometry::{PolygonOrdering, VertexShape};

pub struct Shape2D<V: VectorSpace> 
    where V::Scalar: BasicField {
    vertices: Vec<V>,
    /// I use u32 for space reasons, and I trust
    /// I wont need a polygon with 2^32 vertices.
    indices: Vec<u32>,
}

impl<V: VectorSpace> Shape2D<V> 
    where V::Scalar: BasicField {
    pub fn new(vertices: impl Into<Vec<V>>, indices: impl Into<Vec<u32>>) -> Self 
        where{
        Self { vertices: vertices.into(), indices: indices.into() }
    }
    pub fn from_vertices(shape: impl VertexShape<Vertex = V>, ordering: PolygonOrdering) -> Self {
        Self { vertices: shape.vertices(), indices: shape.indices(ordering) }
    }
}