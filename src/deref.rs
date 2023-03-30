use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::FastVec;

impl<T: Unpin, const N: usize> Deref for FastVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr(), self.len()) }
    }
}

impl<T: Unpin, const N: usize> DerefMut for FastVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr_mut(), self.len()) }
    }
}
