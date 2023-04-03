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

impl<T, const N: usize> Data<T, N> where T: Unpin {
    #[inline(always)]
    unsafe fn stack(&self) -> *const T {
        self.stack.as_ptr() as *const T
    }

    #[inline(always)]
    unsafe fn stack_mut(&mut self) -> *mut T {
        self.stack.as_mut_ptr() as *mut T
    }

    #[inline(always)]
    unsafe fn heap_mut(&mut self) -> &mut (*mut T, usize) {
        &mut self.heap
    }
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
            let (_, len, _) = self.ptr();
            len
        }
    }

    pub fn cap(&self) -> usize {
        unsafe {
            let (_, _, cap) = self.ptr();
            cap
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            let (_, len, _) = self.ptr();
            len == 0
        }
    }

    #[inline(always)]
    pub fn is_heap_allocated(&self) -> bool {
        self.capacity > N        
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            let (mut ptr, mut len, cap) = self.ptr_mut();
            if *len == cap {
                // We need to bump the capacity, potentially move
                // the buf from stack to heap at this point
                if cap > N {
                    // NOTE: Bump capacity by about 33%, which is a hardcoded
                    // heuristics, which might be worth revisiting
                    self.grow(self.grow_by());
                } else {
                    self.heapify();
                }
                let &mut (heap_ptr, ref mut heap_len) = self.data.heap_mut();
                ptr = heap_ptr;
                len = heap_len;
            }

            ptr::write(ptr.add(*len), value);

            *len += 1;
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let (ptr, len, _) = self.ptr_mut();
            if *len == 0 {
                return None;
            }

            let last_index = *len - 1;
            *len = last_index;

            let value = ptr::read(ptr.add(last_index));
            Some(value)
        }
    }
}

impl<T, const N: usize> FastVec<T, N>
where
    T: Unpin,
{
    unsafe fn heapify(&mut self) -> Result<(), Error> {
        debug_assert!(self.cap() <= N, "Already heap allocated");

        let (src, len, cap) = self.ptr();

        let cap = cap + self.grow_by();
        let ptr = {
            let layout = Layout::array::<T>(cap).map_err(|_| Error::LayoutError)?;
            alloc(layout) as *mut T
        };

        ptr::copy_nonoverlapping(src, ptr, len);

        self.data.heap = (ptr, self.capacity);
        self.capacity = cap;

        Ok(())
    }

    unsafe fn grow(&mut self, additional: usize) -> Result<(), Error> {
        debug_assert!(self.capacity > N, "Not heap allocated");

        let (ptr, len, mut cap) = self.ptr_mut();

        let layout = Layout::array::<T>(cap).map_err(|_| Error::LayoutError)?;
        cap = cap
            .checked_add(additional)
            .ok_or(Error::CapacityOverflow)?;
        let size = Layout::array::<T>(cap)
            .map_err(|_| Error::LayoutError)?
            .size();

        // We need to guarantee the following:
        // * We don't ever allocate `> isize::MAX` byte-size objects.
        // * We don't overflow `usize::MAX` and actually allocate too little.
        if usize::BITS < 64 && size > isize::MAX as usize {
            return Err(Error::CapacityOverflow);
        }

        let ptr = realloc(ptr as *mut u8, layout, size) as *mut T;
        self.data = Data { heap: (ptr, *len) };
        self.capacity = cap;

        Ok(())
    }

    #[inline(always)]
    pub(crate) unsafe fn ptr(&self) -> (*const T, usize, usize) {
        if self.capacity > N {
            let (ptr, len) = self.data.heap;
            (ptr, len, self.capacity)
        } else {
            (self.data.stack(), self.capacity, N)
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn ptr_mut(&mut self) -> (*mut T, &mut usize, usize) {
        if self.capacity > N {
            let (ptr, ref mut len) = self.data.heap;
            (ptr, len, self.capacity)
        } else {
            (self.data.stack_mut(), &mut self.capacity, N)
        }
    }

    #[inline(always)]
    // NOTE: This might be worth optimizing with another heuristics
    fn grow_by(&self) -> usize {
        self.capacity
    }
}

impl<T, const N: usize> Drop for FastVec<T, N>
where
    T: Unpin,
{
    fn drop(&mut self) {
        unsafe {
            let (ptr, len, cap) = self.ptr_mut();
            for idx in 0..*len {
                ptr::drop_in_place(ptr.add(idx));
            }
            if cap > N {
                // NOTE: This is the current layout, this shouldn't fail.
                // Unfortunately we don't have a way to gracefully report
                // errors from drop.
                let layout = Layout::array::<T>(cap).unwrap();
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}
