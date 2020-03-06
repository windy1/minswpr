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

/// Defines basic components to manage the state of the main board
pub mod board;
/// Handles user configuration parsing and deserialization
pub mod config;
/// Defines components for the control panel above the board
pub mod control;
/// Handles font loading
pub mod fonts;
/// Implements the behavior for input events
pub mod input;
/// Handles the layout of the GUI
pub mod layout;

pub use app::context::*;
pub use app::*;

/// Helper type for Result
pub type MsResult<R = (), E = String> = Result<R, E>;

/// Misc. utilities
pub mod utils {
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    /// Helper function for quickly borrowing a `RefCell`
    pub fn borrow_safe<T, F, R>(a: &Rc<RefCell<T>>, f: F) -> R
    where
        F: FnOnce(Ref<T>) -> R,
    {
        f(a.borrow())
    }
}
