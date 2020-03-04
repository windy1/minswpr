#![feature(type_ascription)]

#[macro_use]
extern crate minswpr_derive;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod math;
#[macro_use]
pub mod draw;

mod app;
pub mod board;
pub mod config;
pub mod events;
pub mod fonts;
pub mod input;
pub mod layout;

pub use app::context::*;
pub use app::*;

pub type MsResult<R = (), E = String> = Result<R, E>;

pub mod utils {
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    pub fn borrow_safe<T, F, R>(a: &Rc<RefCell<T>>, f: F) -> R
    where
        F: FnOnce(Ref<T>) -> R,
    {
        f(a.borrow())
    }
}
