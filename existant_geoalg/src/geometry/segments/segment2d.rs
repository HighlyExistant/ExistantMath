use existant_core::{BasicField, Bounds, FloatingPoint};

use crate::vectors::Vector2;
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