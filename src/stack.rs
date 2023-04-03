use std::mem::{MaybeUninit, ManuallyDrop};

/// The only thing this does is reserve space on the stack (a.k.a. stack "allocation").
pub struct Stack<T, const N: usize>(ManuallyDrop<MaybeUninit<[T; N]>>);

impl<T, const N: usize> Stack<T, N> {
    #[allow(clippy::uninit_assumed_init)]
    #[must_use]
    pub fn new() -> Self {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        // See: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        Self(ManuallyDrop::new(MaybeUninit::uninit()))
    }
}