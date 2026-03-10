mod rays;
mod hypercubes;
mod hypersphere;
mod segments;
mod shape;
use existant_core::Semimodule;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonGeometry {
    Line,
    Triangle,
}
pub trait Shape {
    type Vertex: Semimodule;
}
/// Represents the shape of an object as a list of vertices
/// and indices to those vertices. Every 3 vertices forms a
/// triangle.
pub trait VertexShape: Shape {
    fn vertices(&self) -> Vec<Self::Vertex>;
    /// Returns a list of indices, to be used alongside the 
    /// returned vertices.
    fn indices(&self, ordering: PolygonOrdering, geometry: PolygonGeometry) -> Option<Vec<u32>>;
}

impl<T: Semimodule> dyn VertexShape<Vertex = T>  {
    pub fn iter_triangle(&self, ordering: PolygonOrdering) -> Option<TriangleMeshIterator<T>> {
        let vertices = self.vertices();
        let indices = self.indices(ordering, PolygonGeometry::Triangle)?;
        Some(TriangleMeshIterator { vertices, indices, current: 0 })
    }
    pub fn iter_line(&self, ordering: PolygonOrdering) -> Option<LineMeshIterator<T>> {
        let vertices = self.vertices();
        let indices = self.indices(ordering, PolygonGeometry::Line)?;
        Some(LineMeshIterator { vertices, indices, current: 0 })
    }
}

pub trait Centroid: Shape {
    fn center(&self) -> Self::Vertex;
}

#[derive(Default)]
pub struct TriangleMeshIterator<Vertex: Semimodule> {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    current: usize
}

impl<Vertex: Semimodule> Iterator for TriangleMeshIterator<Vertex> {
    type Item = [Vertex; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.indices.len()-3 {
            return None;
        }
        let i = self.indices[self.current] as usize;
        let j = self.indices[self.current+1] as usize;
        let k = self.indices[self.current+2] as usize;
        self.current += 3;
        let get = [self.vertices[i], self.vertices[j], self.vertices[k]];
        Some(get)
    }
}

#[derive(Default)]
pub struct LineMeshIterator<Vertex: Semimodule> {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    current: usize
}

impl<Vertex: Semimodule> Iterator for LineMeshIterator<Vertex> {
    type Item = [Vertex; 2];
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.indices.len()-2 {
            return None;
        }
        let i = self.indices[self.current] as usize;
        let j = self.indices[self.current+1] as usize;
        self.current += 2;
        let get = [self.vertices[i], self.vertices[j]];
        Some(get)
    }
}
