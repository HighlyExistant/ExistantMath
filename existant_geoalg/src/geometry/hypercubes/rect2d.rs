use std::ops::Div;

use existant_core::{Addition, BasicField, Bounds, FloatingPoint, FromPrimitive, Identity, Semigroup, Semimodule, Semiring, UniversalOperationsOn, VectorSpace};

use crate::{geometry::{Centroid, HyperCube, LinearSegment2D, Polygon, Sphere2D, VertexShape}, vectors::{NormedVectorSpace, Vector2}};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect2D<T: UniversalOperationsOn<T>> {
    pos: Vector2<T>,
    dimensions: Vector2<T>,
}

impl<T: Semigroup<Addition> +  UniversalOperationsOn<T>> Rect2D<T> {
    pub fn new(pos: Vector2<T>, dimensions: Vector2<T>) -> Self {
        Self { pos, dimensions }
    }
    pub fn dimensionless() -> Self 
        where T: Identity<Addition> {
        Self::new(<Vector2<T> as Identity<Addition>>::IDENTITY, <Vector2<T> as Identity<Addition>>::IDENTITY)
    }
    pub fn from_bounds(a: Vector2<T>, b: Vector2<T>) -> Self 
        where T: Bounds + std::ops::Sub<Output = T> {
        let pos = a.min(b);
        let max = a.max(b);
        Self::new(pos, max-pos)
    }
    pub fn min(&self) -> Vector2<T> {
        self.pos
    }
    pub fn max(&self) -> Vector2<T> 
        where T: std::ops::Add<Output = T> {
        self.pos+self.dimensions
    }
    pub fn width(&self) -> T {
        self.dimensions.x
    }
    pub fn height(&self) -> T {
        self.dimensions.y
    }
    pub fn dimensions(&self) -> Vector2<T> {
        self.dimensions
    }
    pub fn is_dimensionless(&self) -> bool 
        where T: BasicField {
        self.dimensions == <Vector2<T> as Identity<Addition>>::IDENTITY
    }
    /// Attempts to preserve original size, but if a point
    /// lies outside of the [`Rect2D`], will tightly fit
    /// it into its area.
    pub fn fit_point(&self, point: Vector2<T>) -> Self 
        where T: BasicField {
        let mut this = self.clone();
        this.pos = this.pos.min(point);
        this.dimensions = self.max().max(point)-this.pos;
        this
    }
    pub fn fit_rect(&self, rect: Rect2D<T>) -> Self 
        where T: BasicField {
        if self.is_dimensionless() {
            return rect;
        }
        if rect.is_dimensionless() {
            return *self;
        }
        Self::from_bounds(self.pos.min(rect.min()), self.max().max(rect.max()))
    }
    pub fn is_point_inside(&self, point: Vector2<T>) -> bool 
        where T: PartialOrd {
        let min = self.pos;
        let max = self.max();
        min.x <= point.x && point.x <= max.x && min.y <= point.y && point.y <= max.y
    }
    pub fn normalize_dimensions(&self) -> Self 
        where T: FloatingPoint + BasicField {
        Self { pos: self.pos, dimensions: self.dimensions.normalize() }
    }
}

impl<T: FloatingPoint + BasicField> VertexShape for Rect2D<T> {
    fn vertices(&self) -> Vec<Self::Vertex> {
        vec![
            Vector2::new(self.pos.x, self.pos.y),
            Vector2::new(self.pos.x+self.dimensions.x, self.pos.y),
            Vector2::new(self.pos.x+self.dimensions.x, self.pos.y+self.dimensions.y),
            Vector2::new(self.pos.x, self.pos.y+self.dimensions.y),
        ]
    }
    fn indices(&self, ordering: crate::geometry::PolygonOrdering) -> Vec<u32> {
        match ordering {
            crate::geometry::PolygonOrdering::Clockwise => {
                vec![
                    0, 2, 1,
                    2, 0, 3,
                ]
            }
            crate::geometry::PolygonOrdering::CounterClockwise => {
                vec![
                    0, 1, 2,
                    2, 3, 0
                ]
            }
        }
    }
}

impl<T: FloatingPoint + BasicField> From<LinearSegment2D<T>> for Rect2D<T> {
    fn from(value: LinearSegment2D<T>) -> Self {
        Self::from_bounds(value.points[0], value.points[1])
    }
}

impl<T: FloatingPoint + BasicField> From<Sphere2D<T>> for Rect2D<T> {
    fn from(value: Sphere2D<T>) -> Self {
        let d = value.radius*value.radius;
        Self::new(
            value.center-Vector2::new(value.radius, value.radius), 
            Vector2::new(d, d)
        )
    }
}
impl<T: Semiring + UniversalOperationsOn<T>> Polygon for Rect2D<T> {
    type Vertex = Vector2<T>;
}
impl<T: Semiring + UniversalOperationsOn<T> + FromPrimitive> Centroid for Rect2D<T> {
    fn center(&self) -> Self::Vertex {
        self.pos+self.dimensions.scalar_multiplication(T::from_f64(0.5))
    }
}
impl<T: BasicField + UniversalOperationsOn<T>> HyperCube for Rect2D<T> {
    
}