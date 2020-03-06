use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Model<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Model<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

impl<T> AsRef<Rc<RefCell<T>>> for Model<T> {
    fn as_ref(&self) -> &Rc<RefCell<T>> {
        &self.inner
    }
}

impl<T> Clone for Model<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}
