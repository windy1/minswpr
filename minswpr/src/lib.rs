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
pub mod fonts;
pub mod input;
pub mod layout;

pub use app::context::*;
pub use app::*;

pub type MsResult<R = (), E = String> = Result<R, E>;

pub mod utils {
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    use std::time::Duration;
    use std::time::Instant;

    pub fn borrow_safe<T, F, R>(a: &Rc<RefCell<T>>, f: F) -> R
    where
        F: FnOnce(Ref<T>) -> R,
    {
        f(a.borrow())
    }

    #[derive(new, Default)]
    pub struct Stopwatch {
        #[new(default)]
        instant: Option<Instant>,
        #[new(default)]
        elapsed_final: Duration,
    }

    impl Stopwatch {
        pub fn start(&mut self) {
            *self = Self {
                instant: Some(Instant::now()),
                elapsed_final: Default::default(),
            };
        }

        pub fn stop(&mut self) {
            self.elapsed_final = self.elapsed();
            self.instant = None;
        }

        pub fn reset(&mut self) {
            *self = Self::default();
        }

        pub fn elapsed(&self) -> Duration {
            self.instant
                .map(|i| i.elapsed())
                .unwrap_or_else(|| self.elapsed_final)
        }
    }
}
