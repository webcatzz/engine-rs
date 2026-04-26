//! Types for representing mouse input.

use std::ffi::c_int;
use sdl3_sys::mouse::*;
use crate::util::bitmask::bitmask;

/// Represents a mouse button.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseBtn(u8);

impl MouseBtn {

	/// Represents the left mouse button.
	pub const LEFT:   Self = Self(0);
	/// Represents the right mouse button.
	pub const RIGHT:  Self = Self(1);
	/// Represents the middle mouse button.
	pub const MIDDLE: Self = Self(2);

	/// Returns a value representing the nth additional mouse button, starting
	/// from 0, after the left, right, and middle buttons.
	pub const fn additional(n: u8) -> Self {
		Self(n + 3)
	}

	/// Returns the [`MouseBtn`] value corresponding to an SDL mouse button index.
	pub const fn from_sdl_index(index: u8) -> Self {
		match index as c_int {
			SDL_BUTTON_LEFT => Self::LEFT,
			SDL_BUTTON_RIGHT => Self::RIGHT,
			SDL_BUTTON_MIDDLE => Self::MIDDLE,
			SDL_BUTTON_X1 => Self::additional(0),
			SDL_BUTTON_X2 => Self::additional(1),
			_ => panic!(),
		}
	}

}

bitmask! {
	/// A bitmask of mouse buttons.
	pub MouseBtns(u32) {
		/// An empty mask.
		pub NONE   = Self(0),
		/// Mask for the left mouse button.
		pub LEFT   = Self::mask(MouseBtn::LEFT),
		/// Mask for the right mouse button.
		pub RIGHT  = Self::mask(MouseBtn::RIGHT),
		/// Mask for the middle mouse button.
		pub MIDDLE = Self::mask(MouseBtn::MIDDLE),
	}
}

impl MouseBtns {

	/// Returns a mask for a button.
	pub const fn mask(btn: MouseBtn) -> Self {
		Self(1 << btn.0)
	}

	/// Returns the value of a button.
	pub const fn btn(self, btn: MouseBtn) -> bool {
		self.0 & Self::mask(btn).0 != 0
	}

	/// Returns the left mouse button value.
	pub const fn left(self) -> bool {
		self.0 & Self::LEFT.0 != 0
	}

	/// Returns the right mouse button value.
	pub const fn right(self) -> bool {
		self.0 & Self::RIGHT.0 != 0
	}

	/// Returns the middle mouse button value.
	pub const fn middle(self) -> bool {
		self.0 & Self::MIDDLE.0 != 0
	}

	/// Returns an additional mouse button value.
	///
	/// See [`MouseBtn::additional()`] for details.
	pub const fn additional(self, n: u8) -> bool {
		self.0 & (1 << MouseBtn::additional(n).0) != 0
	}

}

impl From<SDL_MouseButtonFlags> for MouseBtns {

	fn from(flags: SDL_MouseButtonFlags) -> Self {
		if flags.0 == 0 { MouseBtns::NONE }
		else {
			let mut btns = MouseBtns::NONE;
			if flags.0 & SDL_BUTTON_LMASK.0  != 0 { btns.0 |= MouseBtns::LEFT.0; }
			if flags.0 & SDL_BUTTON_RMASK.0  != 0 { btns.0 |= MouseBtns::RIGHT.0; }
			if flags.0 & SDL_BUTTON_MMASK.0  != 0 { btns.0 |= MouseBtns::MIDDLE.0; }
			if flags.0 & SDL_BUTTON_X1MASK.0 != 0 { btns.0 |= MouseBtns::mask(MouseBtn::additional(0)).0; }
			if flags.0 & SDL_BUTTON_X2MASK.0 != 0 { btns.0 |= MouseBtns::mask(MouseBtn::additional(1)).0; }
			btns
		}
	}

}