//! A little game engine.
//!
//! Built on top of [Simple DirectMedia Layer] (SDL), a cross-platform
//! development library. Uses the [`sdl3_sys`] Rust bindings.
//!
//! [Simple DirectMedia Layer]: https://www.libsdl.org/
//! [`sdl3_sys`]: https://crates.io/crates/sdl3-sys

pub mod platform;

pub mod asset;
pub mod audio;
pub mod dialog;
pub mod event;
pub mod math;
pub mod run;
pub mod util;
pub mod window;

mod sdl;

use platform::Platform;

/// Convenient shorthand for [`Platform::init()`].
pub fn init() -> Platform {
	Platform::init()
}