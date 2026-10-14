use core::marker::PhantomData;

use bumpalo::{Bump, boxed};

pub struct Driver<'a> {
    heap: Bump,
    _phantom: PhantomData<&'a mut Bump>,
}

impl<'a> Driver<'a> {
    pub fn new() -> Self {
        Self {
            heap: Bump::new(),
            _phantom: PhantomData,
        }
    }

    pub fn alloc<T: Sized>(&'a self, item: T) -> boxed::Box<'a, T> {
        boxed::Box::new_in(item, &self.heap)
    }
}

impl<'a> Default for Driver<'a> {
    fn default() -> Self {
        Self::new()
    }
}
