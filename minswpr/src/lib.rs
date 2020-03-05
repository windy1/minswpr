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

/// Contains misc. math functions and structures
#[macro_use]
pub mod math;

/// Contains the basic components necessary to draw to the canvas
#[macro_use]
pub mod draw;

mod app;
pub mod board;
pub mod config;
pub mod fonts;
pub mod input;
pub mod layout;
pub mod stopwatch;

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
