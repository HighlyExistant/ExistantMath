use existant_core::{BasicField, FloatingPoint, Semiring, UniversalOperationsOn};

use crate::{geometry::Polygon, vectors::{MetricSpace, NormedVectorSpace, Vector2}};

pub struct Sphere2D<T: FloatingPoint> {
    pub center: Vector2<T>,
    pub radius: T,
}

impl<T: FloatingPoint + BasicField> Sphere2D<T> {
    pub fn new(pos: Vector2<T>, radius: T) -> Self {
        Self { center: pos, radius }
    }
    pub fn center(&self) -> Vector2<T> {
        self.center
    }
    pub fn radius(&self) -> T {
        self.radius
    }
    pub fn distance_from_center(&self, point: Vector2<T>) -> T {
        self.center.distance(point)
    }
    /// If its intersecting, returns the 
    pub fn point_intersection(&self, point: Vector2<T>) -> Option<T> {
        let dist = self.distance_from_center(point);
        if dist <= self.radius.sqrt() {
            Some(dist)
        } else {
            None
        }
    }
}

impl<T: Semiring + FloatingPoint> Polygon for Sphere2D<T> {
    type Vertex = Vector2<T>;
}
