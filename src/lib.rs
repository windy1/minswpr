#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;

#[macro_use]
pub mod math;

mod app;
pub mod board;
pub mod events;
pub mod fonts;
pub mod input;
pub mod layout;
pub mod render;

pub use app::*;
