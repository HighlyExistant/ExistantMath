use existant_core::{BasicField, Semimodule};

use crate::vectors::{NormedVectorSpace, Vector3};

pub struct Ray3D<T> {
    pos: Vector3<T>,
    dir: Vector3<T>,
}

impl<T: BasicField> Ray3D<T> 
    where Vector3<T>: NormedVectorSpace + Semimodule<Scalar = T> {
    
    /// Creates a new [`Ray3D`] object. Assumes that the `dir`
    /// provided is normalized, for speed.
    pub unsafe fn new_unchecked(pos: Vector3<T>, dir: Vector3<T>) -> Self {
        Self { pos, dir }
    }
    pub fn new(pos: Vector3<T>, dir: Vector3<T>) -> Self {
        Self { dir: dir.normalize(), pos }
    }
    pub unsafe fn set_dir_unchecked(&mut self, dir: Vector3<T>) {
        self.dir = dir
    }
    pub unsafe fn set_dir(&mut self, dir: Vector3<T>) {
        self.dir = dir.normalize()
    }
    pub fn set_pos(&mut self, pos: Vector3<T>) {
        self.pos = pos;
    }
    #[inline]
    pub fn pos(&self) -> Vector3<T> {
        self.pos
    }
    #[inline]
    pub fn dir(&self) -> Vector3<T> {
        self.dir
    }
}