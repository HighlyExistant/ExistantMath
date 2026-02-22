use std::usize;

use existant_core::{Addition, BasicField, FloatingPoint, Identity};
use existant_geoalg::{geometry::{Centroid, Rect2D}, mappings::MortonU32, vectors::Vector2};

use crate::cpu::RawHeap;

pub enum BVH2DNodeTraverse<'a, Object, T: FloatingPoint + BasicField> {
    Node(&'a Rect2D<T>),
    Leaf(&'a Object),
}
impl<'a, Object, T: FloatingPoint + BasicField> BVH2DNodeTraverse<'a, Object, T> {
    pub fn is_node(&self) -> bool {
        if let BVH2DNodeTraverse::Node(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_leaf(&self) -> bool {
        if let BVH2DNodeTraverse::Leaf(_) = self {
            true
        } else {
            false
        }
    }
}
pub struct BVH2DNode {

}

#[derive(Debug, Clone)]
pub struct BVH2D<Object: Into<Rect2D<T>>, T: FloatingPoint + BasicField> {
    pub rects: RawHeap<Rect2D<T>>,
    pub objects: Vec<Object>,
    /// Index to the objects. There are as many indices
    /// as there are rects in the last depth level of the heap.
    pub indices: Vec<(MortonU32, u32)>
}

impl<Object: Into<Rect2D<T>> + Copy, T: FloatingPoint + BasicField> BVH2D<Object, T> {
    pub fn new(objects: Vec<Object>) -> Self {
        let rects = objects.iter().map(|obj|{
            Into::<Rect2D<T>>::into(*obj)
        }).collect::<Vec<Rect2D<T>>>();
        // Calculate largest rect
        let bbox = rects.iter().fold(Rect2D::dimensionless(), |a, b|{
            a.fit_rect(*b)
        });
        // Calculate all Centroids and Organize the points into morton codes
        let centroids = rects.iter().map(|rect|{
            rect.center()/bbox.dimensions()
        }).collect::<Vec<_>>();
        let mut morton = centroids.iter().enumerate().map(|(idx, center)|{
            // We are assuming that we wont exceed 2^32 rects
            (MortonU32::encode_xy(
                (center.x*T::from_f32(1024.0)).to_u16(), 
                (center.y*T::from_f32(1024.0)).to_u16()
            ), idx as u32)
        }).collect::<Vec<_>>();
        // Sort rect indices using morton codes
        morton.sort_by(|a, b|{
            a.0.cmp(&b.0)
        });
        
        // Recursively combine all rects bottom up.
        let indexed = std::iter::once(&rects).cycle().zip(morton.iter()).map(|(rect, (m, i))|{
            rect[*i as usize]
        });
        let rects = RawHeap::<Rect2D<T>>::bottom_up(indexed, |a, b|{
            a.fit_rect(*b)
        });

        Self { rects, objects, indices: morton }
    }
    fn get_node(&self, index: usize) -> BVH2DNodeTraverse<'_, Object, T> {
        if self.rects.is_leaf(index) {
            let index = self.indices[index-(self.rects.len()/2)];
            let obj = &self.objects[index.1 as usize];
            BVH2DNodeTraverse::Leaf(obj)
        } else {
            let rect = &self.rects[index];
            BVH2DNodeTraverse::Node(rect)
        }
    }
    /// F returns an f32 saying how far an object is, and None otherwise.
    fn traverse_inner<F>(&self, mut idx: usize, mut f: F) -> Option<&Object> 
        where F: FnMut(BVH2DNodeTraverse<'_, Object, T>) -> Option<f32> {
        let mut stack = vec![idx];
        let mut closest = 0.0;
        let mut index = usize::MAX;
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let node = self.get_node(top);
            let is_leaf = node.is_leaf();
            if let Some(dist) = f(node) {
                if dist < closest && is_leaf {
                    closest = dist;
                    index = top;
                } else {
                    stack.push(RawHeap::<T>::left_child(index));
                    stack.push(RawHeap::<T>::right_child(index));
                }
            }
        }
        if index == usize::MAX {
            None
        } else {
            let index = self.indices[index-(self.rects.len()/2)];
            let obj = &self.objects[index.1 as usize];
            Some(obj)
        }
    }
    pub fn traverse<F>(&self, mut f: F) -> Option<&Object> 
        where F: FnMut(BVH2DNodeTraverse<'_, Object, T>) -> Option<f32> {
        self.traverse_inner(0, f)
    }
}