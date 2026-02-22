use std::{fmt::Debug, ops::{Deref, DerefMut, Div}};

/// This heap structure is meant for when you want
/// to utilize the full power of the Heap, and for
/// this reason, it is a very simple version of
/// the data structure.
#[derive(Debug, Clone)]
pub struct RawHeap<T> {
    heap: Vec<T>
}

impl<T> Deref for RawHeap<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}

impl<T> DerefMut for RawHeap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap
    }
}

impl<T> RawHeap<T> {
    fn to_power_of_two(number: usize) -> usize {
        if number.is_power_of_two() {
            number
        } else {
            number.next_power_of_two()
        }
    }
    pub fn new(count: usize) -> Self {
        let len = Self::to_power_of_two(count);
        let mut heap = Vec::<T>::with_capacity(len);
        unsafe {
            heap.set_len(len);
        }
        Self { heap }
    }
    pub fn from_vec(heap: Vec<T>) -> Self  {
        Self { heap }
    }
    pub fn from_iter(iter: impl ExactSizeIterator<Item = T>) -> Self  {
        let mut iter = Vec::from_iter(iter);
        let count = Self::to_power_of_two(iter.len());
        let reserve = count-iter.len();
        iter.reserve(reserve);
        unsafe {
            std::ptr::write_bytes(iter.as_mut_ptr().add(iter.len()), 0, reserve);
            iter.set_len(count);
        }
        Self { heap: iter }
    }
    fn index(&self, value: &T) -> usize {
        let base = self.heap.as_ptr() as usize;
        let value_idx = value as *const _ as usize;
        (value_idx - base).div(std::mem::size_of::<T>())
    }
    fn bottom_up_internal<F>(heap: &mut RawHeap<T>, depth: u32, mut f: F) 
        where F: FnMut(&T, &T) -> T {
        if depth==0 {
            return;
        }
        // Change lifetime with transmute
        let at_depth: &[T] = unsafe { std::mem::transmute(heap.get_depth_slice(depth).unwrap()) };
        
        for slice in at_depth.chunks(2) {
            let idx = heap.index(&slice[0]);
            let parent_i = Self::parent(idx);
            heap.heap[parent_i] = f(&slice[0], &slice[1]);
        }
        Self::bottom_up_internal(heap, depth - 1, f);
    }
    /// Performs a bottom up construction, where the parents 
    /// the children are constructed from the children.
    pub fn bottom_up<F, I: Iterator<Item = T>>(base: I, f: F) -> Self 
        where F: FnMut(&T, &T) -> T {
        let mut base = base.collect::<Vec<T>>();
        
        let count = Self::to_power_of_two(base.len());
        let reserve = count-base.len();

        base.reserve(reserve);
        unsafe {
            std::ptr::write_bytes(base.as_mut_ptr().add(base.len()), 0, reserve);
            base.set_len(count);
        }

        let mut heap = RawHeap::<T>::new(base.len()*2-1);
        let last = heap.get_last_depth_slice_mut().unwrap();
        unsafe { std::ptr::copy_nonoverlapping(base.as_ptr(), last.as_mut_ptr(), last.len()) };
        let depth = heap.depth()-1;
        Self::bottom_up_internal(&mut heap, depth, f);
        heap
    }
    pub fn depth(&self) -> u32 {
        println!("{}", self.heap.len());
        self.heap.len().ilog2()
    }
    pub fn get_depth_slice(&self, depth: u32) -> Option<&[T]> {
        if depth >= self.depth() {
            return None;
        }
        let base = 2usize.pow(depth);
        let length = (base*2)-1;
        Some(&self.heap[(base-1)..length])
    }
    pub fn get_depth_slice_mut(&mut self, depth: u32) -> Option<&mut [T]> {
        if depth >= self.depth() {
            return None;
        }
        let base = 2usize.pow(depth);
        let length = (base*2)-1;
        Some(&mut self.heap[(base-1)..length])
    }
    pub fn get_last_depth_slice_mut(&mut self) -> Option<&mut [T]> {
        self.get_depth_slice_mut(self.depth()-1)
    }
    /// Returns the left child at the specified index.
    pub fn left_child(index: usize) -> usize {
        2*index + 1
    }
    /// Returns the right child at the specified index.
    pub fn right_child(index: usize) -> usize {
        2*index + 2
    }
    /// Returns the right child at the specified index.
    pub fn is_leaf(&self, index: usize) -> bool {
        println!("{}", self.depth());
        println!("{}", index.ilog2());
        self.depth() == index.ilog2()
    }

    /// Returns the parent of the specified index.
    pub fn parent(index: usize) -> usize {
        ((index -1)/2) as usize
    }
}