#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod math;

mod app;
pub mod board;
pub mod fonts;
pub mod input;
pub mod render;

pub use app::*;
