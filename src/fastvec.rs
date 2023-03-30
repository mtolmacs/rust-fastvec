use crate::{error::Error, stack::Stack};
use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    ptr::{self, NonNull},
};

// NOTE: `cap` could be used for storage when we use the stack storage, saving a
// pointer-sized memory span by moving it to the heap when the buffer is heap
// allocated. However at that point `cap` needs a pointer dereference (at least)
// to get the value, which is slower.
struct Header<T, const N: usize> {
    cap: usize,
    len: usize,
    ptr: NonNull<T>,
}

pub struct FastVec<T, const N: usize>
where
    T: Unpin,
{
    header: Header<T, N>,
    // Stack address shouldn't change, so no need for Pin
    _stack: Stack<T, N>,
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
            header: Header {
                cap: N,
                len: 0,
                ptr: NonNull::dangling(),
            },
            _stack: Stack::new(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.header.len
    }

    #[inline(always)]
    pub fn cap(&self) -> usize {
        self.header.cap
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn is_heap_allocated(&self) -> bool {
        self.header.cap > N
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
            self.header.len += 1;

            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if !self.is_empty() {
                // This will never underflow, so no need to have checked sub
                self.header.len -= 1;

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
            NonNull::new(alloc(layout) as *mut T).ok_or(Error::AllocError)?
        };
        // NOTE: We take advantage that MaybeUninit<T> has the same layout and
        // size as T and the fact that we allocated an N * T sized heap buffer.
        // NOTE: No calling drop on T, since we still own the items, just movng them.
        ptr::copy_nonoverlapping(&mut self._stack as *mut _ as *const T, ptr.as_ptr(), N);

        // NonNull is just a marker trait, so overwriting it should be equivalent
        // to raw pointer assignment
        self.header.ptr = ptr;
        self.header.cap = cap;

        Ok(())
    }

    unsafe fn grow(&mut self, additional: usize) -> Result<(), Error> {
        debug_assert!(self.header.cap > N, "Not heap allocated");

        let layout = Layout::array::<T>(self.cap()).map_err(|_| Error::LayoutError)?;
        self.header.cap = self
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

        let ptr = realloc(self.header.ptr.as_ptr() as *mut u8, layout, size) as *mut T;
        self.header.ptr = NonNull::new(ptr).ok_or(Error::AllocError)?;

        Ok(())
    }

    #[inline(always)]
    pub(crate) unsafe fn ptr(&self) -> *const T {
        if self.is_heap_allocated() {
            self.header.ptr.as_ptr()
        } else {
            &self._stack as *const _ as *const T
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn ptr_mut(&mut self) -> *mut T {
        if self.is_heap_allocated() {
            self.header.ptr.as_ptr()
        } else {
            &mut self._stack as *mut _ as *mut T
        }
    }

    // NOTE: This might be worth optimizing with another heuristics
    #[inline(always)]
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
                let layout = Layout::array::<T>(self.header.cap).unwrap();
                dealloc(self.ptr_mut() as *mut u8, layout);
            }
        }
    }
}
