use std::ops::{Index, IndexMut};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct TowDArray<T> {
    pub w: usize,
    pub h: usize,
    data: Box<[T]>,
}

impl<T: Default + Clone> TowDArray<T> {
    pub fn new(w: usize, h: usize) -> Self {
        TowDArray {
            w: w,
            h: h,
            data: vec![T::default(); w*h].into_boxed_slice(),
        }
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.w * y + x]
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[self.w * y + x] = value;
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.w * y + x]
    }
    pub fn point_to_position(&self, x: usize, y: usize) -> usize {
        self.w * y + x
    }
    pub fn position_to_point(&self, i: usize) -> (usize, usize) {
        (i % self.w, i / self.w)
    }
}

// Traits impl
// impl<T: Default + Clone> Index<usize> for TowDArray<T> {
//     type Output = T;
//     fn index(&self, x :usize) -> &T {
//         &self.data[x]
//     }
// }
impl<T: Default + Clone> Index<(usize, usize)> for TowDArray<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        self.get(x, y)
    }
}

impl<T: Default + Clone> IndexMut<(usize, usize)> for TowDArray<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        self.get_mut(x, y)
    }
}

impl<T> Deref for TowDArray<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.data
    }
}
impl<T> DerefMut for TowDArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
