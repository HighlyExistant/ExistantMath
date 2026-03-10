use existant_core::{BasicField, Bounds, FloatingPoint};

use crate::{geometry::{PolygonGeometry, PolygonOrdering, Shape, VertexShape}, vectors::Vector2};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearSegment2D<T: FloatingPoint + BasicField> {
    pub points: [Vector2<T>; 2],
}

impl<T: FloatingPoint + BasicField> LinearSegment2D<T> {
    pub fn new(start: Vector2<T>, end: Vector2<T>) -> Self {
        Self { points: [start, end] }
    }
    pub fn min(&self) -> Vector2<T> {
        self.points[0].min(self.points[1])
    }
    pub fn max(&self) -> Vector2<T> {
        self.points[0].max(self.points[1])
    }
}

impl<T: FloatingPoint + BasicField> Shape  for LinearSegment2D<T> {
    type Vertex = Vector2<T>;
}

impl<T: FloatingPoint + BasicField> VertexShape  for LinearSegment2D<T> {
    fn vertices(&self) -> Vec<Self::Vertex> {
        self.points.to_vec()
    }
    fn indices(&self, ordering: crate::geometry::PolygonOrdering, geometry: PolygonGeometry) -> Option<Vec<u32>> {
        if geometry == PolygonGeometry::Triangle {
            return None;
        }
        if ordering == PolygonOrdering::Clockwise {
            Some(vec![0, 1])
        } else {
            Some(vec![1, 0])
        }
    }
}