use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::FastVec;

impl<T: Unpin, const N: usize> Deref for FastVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let (ptr, len, _) = self.ptr();
            slice::from_raw_parts(ptr, len)
        }
    }
}

impl<T: Unpin, const N: usize> DerefMut for FastVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let (ptr, len, _) = self.ptr_mut();
            slice::from_raw_parts_mut(ptr, *len)
        }
    }
}
