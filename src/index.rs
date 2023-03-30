use crate::FastVec;
use std::{ops::{Index, IndexMut}, slice::SliceIndex};

impl<T: Unpin, I: SliceIndex<[T]>, const N: usize> Index<I> for FastVec<T, N> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T: Unpin, I: SliceIndex<[T]>, const N: usize> IndexMut<I> for FastVec<T, N> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}