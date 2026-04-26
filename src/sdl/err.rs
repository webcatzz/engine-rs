//! SDL error handling.

use std::ffi::CStr;
use std::fmt;
use std::ptr::NonNull;
use sdl3_sys::error::SDL_GetError;

/// Represents SDL's current error message. Note that the error message
/// associated with this struct will be overwritten by subsequent SDL errors.
pub struct SdlError;

impl fmt::Display for SdlError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { writeln!(f, "SDL error: {}", CStr::from_ptr(SDL_GetError()).to_str().expect("SDL error should be valid UTF-8")) }
	}

}

impl fmt::Debug for SdlError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}

}

impl std::error::Error for SdlError {}

/// Panics with the current SDL error message.
#[macro_export]
#[doc(hidden)]
macro_rules! sdl_panic {
	() => {
		panic!("{}", crate::sdl::err::SdlError)
	};
}

/// Asserts a condition and prints the current SDL error message on failure.
#[macro_export]
#[doc(hidden)]
macro_rules! sdl_assert {
	($cond:expr) => {
		assert!($cond, "{}", crate::sdl::err::SdlError)
	};
}

pub use sdl_panic;
pub use sdl_assert;

/// Converts a pointer to a [`NonNull`]. If it is null, panics with the current
/// SDL error message.
pub fn non_null_or_sdl_panic<T>(ptr: *mut T) -> NonNull<T> {
	match NonNull::new(ptr) {
		Some(non_null) => non_null,
		None => sdl_panic!(),
	}
}