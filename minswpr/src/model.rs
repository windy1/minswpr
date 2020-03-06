use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/// A wrapper around an `Rc<RefCell<T>>` for convienience
pub struct ModelRef<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> ModelRef<T> {
    /// Creates a new `ModelRef` with the specified `inner` value
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// Calls `RefCell::borrow` on `inner`
    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    /// Calls `RefCell::borrow_mut` on `inner`
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

impl<T> AsRef<Rc<RefCell<T>>> for ModelRef<T> {
    fn as_ref(&self) -> &Rc<RefCell<T>> {
        &self.inner
    }
}

impl<T> Clone for ModelRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}
