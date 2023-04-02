use crate::{error::Error};
use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    mem::{ManuallyDrop, MaybeUninit},
    ptr::{self, NonNull},
};

union Data<T, const N: usize>
where
    T: Unpin,
{
    stack: ManuallyDrop<MaybeUninit<[T; N]>>,
    heap: (*mut T, usize),
}

pub struct FastVec<T, const N: usize>
where
    T: Unpin,
{
    capacity: usize,
    data: Data<T, N>,
}

impl<T, const N: usize> FastVec<T, N>
where
    T: Unpin,
{
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        debug_assert!(N > 0, "Zero length buffer is not supported");
        debug_assert!(N < isize::MAX as usize, "Maximum legnth is isize::MAX");

        Self {
            capacity: 0,
            data: Data { stack: ManuallyDrop::new(MaybeUninit::uninit()) }
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            if self.is_heap_allocated() {
                let (_, len) = self.data.heap;
                len
            } else {
                self.capacity
            }
        }
    }

    pub fn cap(&self) -> usize {
        if self.is_heap_allocated() {
            self.capacity
        } else {
            N
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_heap_allocated(&self) -> bool {
        self.capacity > N
    }

    pub fn push(&mut self, value: T) -> Result<(), Error> {
        unsafe {
            let remaining = self.cap() - self.len();
            if remaining == 0 {
                // We need to bump the capacity, potentially move
                // the buf from stack to heap at this point
                if self.is_heap_allocated() {
                    // NOTE: Bump capacity by about 33%, which is a hardcoded
                    // heuristics, which might be worth revisiting
                    self.grow(self.grow_by())?;
                } else {
                    self.heapify()?;
                }
            }

            ptr::write(self.ptr_mut().add(self.len()), value);

            // `len` is never bigger than `cap`,
            // so no need to check for overflow
            if self.is_heap_allocated() {
                self.data.heap.1 += 1;
            } else {
                self.capacity += 1;
            }

            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if !self.is_empty() {
                // This will never underflow, so no need to have checked sub
                if self.is_heap_allocated() {
                    self.data.heap.1 -= 1;
                } else {
                    self.capacity -= 1;
                }

                let value = ptr::read(self.ptr().add(self.len()));
                Some(value)
            } else {
                None
            }
        }
    }
}

impl<T, const N: usize> FastVec<T, N>
where
    T: Unpin,
{
    unsafe fn heapify(&mut self) -> Result<(), Error> {
        debug_assert!(self.cap() <= N, "Already heap allocated");

        let cap = self.cap() + self.grow_by();
        let ptr = {
            let layout = Layout::array::<T>(cap).map_err(|_| Error::LayoutError)?;
            alloc(layout) as *mut T
        };
        // NOTE: We take advantage that MaybeUninit<T> has the same layout and
        // size as T and the fact that we allocated an N * T sized heap buffer.
        // NOTE: No calling drop on T, since we still own the items, just movng them.
        ptr::copy_nonoverlapping(&mut self.data.stack as *mut _ as *const T, ptr, N);

        // NonNull is just a marker trait, so overwriting it should be equivalent
        // to raw pointer assignment

        self.data.heap = (ptr, self.capacity);
        self.capacity = cap;

        Ok(())
    }

    unsafe fn grow(&mut self, additional: usize) -> Result<(), Error> {
        debug_assert!(self.capacity > N, "Not heap allocated");

        let layout = Layout::array::<T>(self.cap()).map_err(|_| Error::LayoutError)?;
        self.capacity = self
            .cap()
            .checked_add(additional)
            .ok_or(Error::CapacityOverflow)?;
        let size = Layout::array::<T>(self.cap())
            .map_err(|_| Error::LayoutError)?
            .size();

        // We need to guarantee the following:
        // * We don't ever allocate `> isize::MAX` byte-size objects.
        // * We don't overflow `usize::MAX` and actually allocate too little.
        if usize::BITS < 64 && size > isize::MAX as usize {
            return Err(Error::CapacityOverflow);
        }

        let ptr = realloc(self.data.heap.0 as *mut u8, layout, size) as *mut T;
        self.data.heap.0 = ptr;

        Ok(())
    }

    pub(crate) unsafe fn ptr(&self) -> *const T {
        if self.is_heap_allocated() {
            self.data.heap.0 as *const T
        } else {
            self.data.stack.as_ptr() as *const T
        }
    }

    pub(crate) unsafe fn ptr_mut(&mut self) -> *mut T {
        if self.is_heap_allocated() {
            self.data.heap.0 as *mut T
        } else {
            (*self.data.stack).as_mut_ptr() as *mut T
        }
    }

    // NOTE: This might be worth optimizing with another heuristics
    fn grow_by(&self) -> usize {
        self.cap()
    }
}

impl<T, const N: usize> Drop for FastVec<T, N>
where
    T: Unpin,
{
    fn drop(&mut self) {
        unsafe {
            for idx in 0..self.len() {
                let ptr = self.ptr_mut().add(idx);
                ptr::drop_in_place(ptr);
            }
            if self.is_heap_allocated() {
                // NOTE: This is the current layout, this shouldn't fail.
                // Unfortunately we don't have a way to gracefully report
                // errors from drop.
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr_mut() as *mut u8, layout);
            }
        }
    }
}
