use core::marker::PhantomData;

use bumpalo::{Bump, boxed};

/// Compiler driver -- the struct that should manage the compilation phases.
pub struct Driver<'a> {
    heap: Bump,
    _phantom: PhantomData<&'a mut Bump>,
}

impl<'a> Driver<'a> {
    /// Creates a new `Driver`.
    pub fn new() -> Self {
        Self {
            heap: Bump::new(),
            _phantom: PhantomData,
        }
    }

    /// Allocates a new `T`.
    pub fn alloc<T: Sized>(&'a self, item: T) -> boxed::Box<'a, T> {
        boxed::Box::new_in(item, &self.heap)
    }
}

impl<'a> Default for Driver<'a> {
    fn default() -> Self {
        Self::new()
    }
}
