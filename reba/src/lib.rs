#![feature(type_ascription)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;

mod init;

/// Contains misc. math functions and structures
#[macro_use]
pub mod math;
pub mod fonts;
pub mod context;
pub mod app;
pub mod draw;

pub use init::*;

pub type RebaResult<R = (), E = String> = Result<R, E>;
