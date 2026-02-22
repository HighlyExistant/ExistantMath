mod rect2d;
use existant_core::BasicField;
pub use rect2d::*;

use crate::vectors::Vector2;

/// Represents an n-dimensional cube.
pub trait HyperCube {
    
}

pub trait BoundingBox {
    type Boundary: HyperCube;
    /// Returns a tight non-oriented bounding box which 
    /// covers the entire shape.
    fn bounding_box(&self) -> Self::Boundary;
}

// impl<T: BasicField> BoundingBox for [Vector2<T>] {
//     type Boundary = Rect2D<T>;
//     fn bounding_box(&self) -> Self::Boundary {
        
//     }
// }