//! Types for representing keyboard input.

use std::fmt;
use sdl3_sys::keycode::*;
use crate::util::bitmask::bitmask;

/// Represents a key on a keyboard.
///
/// Character keys are always represented in "lowercase", with the character
/// they would produce if no modifier keys were held.
///
/// Although some control and whitespace keys have corresponding unicode
/// characters, they're usually represented with their own items.
///
/// Thanks to niche filling, [`Key`] has the same size as [`char`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
	/// A unicode character.
	Char(char),
	// Control keys
	Backspace,
	CapsLock,
	Delete,
	Down,
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	LAlt,
	LCtrl,
	Left,
	LShift,
	LSuper,
	RAlt,
	RCtrl,
	Return,
	Right,
	RShift,
	RSuper,
	Space,
	Tab,
	Up,
	/// An unknown key.
	Unknown,
}

impl Key {

	/// Returns the character associated with a key, if any.
	pub const fn char(&self) -> Option<char> {
		match self {
			Self::Char(c) => Some(*c),
			_ => None,
		}
	}

	/// Returns the number associated with a key, if any.
	pub const fn num(&self) -> Option<u8> {
		match self {
			Self::Char('1') => Some(1),
			Self::Char('2') => Some(2),
			Self::Char('3') => Some(3),
			Self::Char('4') => Some(4),
			Self::Char('5') => Some(5),
			Self::Char('6') => Some(6),
			Self::Char('7') => Some(7),
			Self::Char('8') => Some(8),
			Self::Char('9') => Some(9),
			Self::Char('0') => Some(0),
			_ => None,
		}
	}

}

impl fmt::Display for Key {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Char(c) => write!(f, "[{c}]"),
			Self::Backspace => write!(f, "[Backspace]"),
			Self::CapsLock  => write!(f, "[CapsLock]"),
			Self::Delete    => write!(f, "[Delete]"),
			Self::Down      => write!(f, "[Down]"),
			Self::Escape    => write!(f, "[Escape]"),
			Self::F1        => write!(f, "[F1]"),
			Self::F10       => write!(f, "[F10]"),
			Self::F11       => write!(f, "[F11]"),
			Self::F12       => write!(f, "[F12]"),
			Self::F2        => write!(f, "[F2]"),
			Self::F3        => write!(f, "[F3]"),
			Self::F4        => write!(f, "[F4]"),
			Self::F5        => write!(f, "[F5]"),
			Self::F6        => write!(f, "[F6]"),
			Self::F7        => write!(f, "[F7]"),
			Self::F8        => write!(f, "[F8]"),
			Self::F9        => write!(f, "[F9]"),
			Self::LAlt      => write!(f, "[LAlt]"),
			Self::LCtrl     => write!(f, "[LCtrl]"),
			Self::Left      => write!(f, "[Left]"),
			Self::LShift    => write!(f, "[LShift]"),
			Self::LSuper    => write!(f, "[LSuper]"),
			Self::RAlt      => write!(f, "[RAlt]"),
			Self::RCtrl     => write!(f, "[RCtrl]"),
			Self::Return    => write!(f, "[Return]"),
			Self::Right     => write!(f, "[Right]"),
			Self::RShift    => write!(f, "[RShift]"),
			Self::RSuper    => write!(f, "[RSuper]"),
			Self::Space     => write!(f, "[Space]"),
			Self::Tab       => write!(f, "[Tab]"),
			Self::Up        => write!(f, "[Up]"),
			Self::Unknown   => write!(f, "[Unknown]"),
		}
	}

}

impl From<SDL_Keycode> for Key {

	fn from(keycode: SDL_Keycode) -> Self {
		match keycode {
			SDLK_A |
			SDLK_B |
			SDLK_C |
			SDLK_D |
			SDLK_E |
			SDLK_F |
			SDLK_G |
			SDLK_H |
			SDLK_I |
			SDLK_J |
			SDLK_K |
			SDLK_L |
			SDLK_M |
			SDLK_N |
			SDLK_O |
			SDLK_P |
			SDLK_Q |
			SDLK_R |
			SDLK_S |
			SDLK_T |
			SDLK_U |
			SDLK_V |
			SDLK_W |
			SDLK_X |
			SDLK_Y |
			SDLK_Z |
			SDLK_0 |
			SDLK_1 |
			SDLK_2 |
			SDLK_3 |
			SDLK_4 |
			SDLK_5 |
			SDLK_6 |
			SDLK_7 |
			SDLK_8 |
			SDLK_9 |
			SDLK_AMPERSAND |
			SDLK_APOSTROPHE |
			SDLK_ASTERISK |
			SDLK_AT |
			SDLK_BACKSLASH |
			SDLK_CARET |
			SDLK_COLON |
			SDLK_COMMA |
			SDLK_DBLAPOSTROPHE |
			SDLK_DOLLAR |
			SDLK_EQUALS |
			SDLK_EXCLAIM |
			SDLK_GRAVE |
			SDLK_GREATER |
			SDLK_HASH |
			SDLK_LEFTBRACE |
			SDLK_LEFTBRACKET |
			SDLK_LEFTPAREN |
			SDLK_LESS |
			SDLK_MINUS |
			SDLK_PERCENT |
			SDLK_PERIOD |
			SDLK_PIPE |
			SDLK_PLUS |
			SDLK_PLUSMINUS |
			SDLK_QUESTION |
			SDLK_RIGHTBRACE |
			SDLK_RIGHTBRACKET |
			SDLK_RIGHTPAREN |
			SDLK_SEMICOLON |
			SDLK_SLASH |
			SDLK_TILDE |
			SDLK_UNDERSCORE => Key::Char(char::try_from(keycode.0).unwrap()),
			SDLK_BACKSPACE => Key::Backspace,
			SDLK_CAPSLOCK => Key::CapsLock,
			SDLK_DELETE => Key::Delete,
			SDLK_DOWN => Key::Down,
			SDLK_ESCAPE => Key::Escape,
			SDLK_F1 => Key::F1,
			SDLK_F10 => Key::F10,
			SDLK_F11 => Key::F11,
			SDLK_F12 => Key::F12,
			SDLK_F2 => Key::F2,
			SDLK_F3 => Key::F3,
			SDLK_F4 => Key::F4,
			SDLK_F5 => Key::F5,
			SDLK_F6 => Key::F6,
			SDLK_F7 => Key::F7,
			SDLK_F8 => Key::F8,
			SDLK_F9 => Key::F9,
			SDLK_LALT => Key::LAlt,
			SDLK_LCTRL => Key::LCtrl,
			SDLK_LEFT => Key::Left,
			SDLK_LGUI => Key::LSuper,
			SDLK_LSHIFT => Key::LShift,
			SDLK_RALT => Key::RAlt,
			SDLK_RCTRL => Key::RCtrl,
			SDLK_RETURN => Key::Return,
			SDLK_RGUI => Key::RSuper,
			SDLK_RIGHT => Key::Right,
			SDLK_RSHIFT => Key::RShift,
			SDLK_SPACE => Key::Space,
			SDLK_TAB => Key::Tab,
			SDLK_UP => Key::Up,
			_ => Key::Unknown,
		}
	}

}

bitmask! {
	/// A bitmask of modifier keys.
	pub KeyMod(SDL_Keymod) {
		/// Empty mask.
		pub NONE      = Self(SDL_KMOD_NONE),
		/// Mask for the left and right shift keys.
		pub SHIFT     = Self(SDL_KMOD_SHIFT),
		/// Mask for the left shift key.
		pub LSHIFT    = Self(SDL_KMOD_LSHIFT),
		/// Mask for the right shift key.
		pub RSHIFT    = Self(SDL_KMOD_RSHIFT),
		/// Mask for the left and right ctrl keys.
		pub CTRL      = Self(SDL_KMOD_CTRL),
		/// Mask for the left ctrl key.
		pub LCTRL     = Self(SDL_KMOD_LCTRL),
		/// Mask for the right ctrl key.
		pub RCTRL     = Self(SDL_KMOD_RCTRL),
		/// Mask for the left and right alt keys.
		pub ALT       = Self(SDL_KMOD_ALT),
		/// Mask for the left alt key.
		pub LALT      = Self(SDL_KMOD_LALT),
		/// Mask for the right alt key.
		pub RALT      = Self(SDL_KMOD_RALT),
		/// Mask for the left and right super keys.
		pub SUPER     = Self(SDL_KMOD_GUI),
		/// Mask for the left super key.
		pub LSUPER    = Self(SDL_KMOD_LGUI),
		/// Mask for the right super key.
		pub RSUPER    = Self(SDL_KMOD_RGUI),
		/// Mask for the caps lock key.
		pub CAPS_LOCK = Self(SDL_KMOD_CAPS),
	}
}

impl KeyMod {

	/// Returns the value for the shift keys.
	pub const fn shift(self) -> bool {
		self.0.0 & Self::SHIFT.0.0 != 0
	}

	/// Returns the value for the left shift key.
	pub const fn lshift(self) -> bool {
		self.0.0 & Self::LSHIFT.0.0 != 0
	}

	/// Returns the value for the right shift key.
	pub const fn rshift(self) -> bool {
		self.0.0 & Self::RSHIFT.0.0 != 0
	}

	/// Returns the value for the control keys.
	pub const fn ctrl(self) -> bool {
		self.0.0 & Self::CTRL.0.0 != 0
	}

	/// Returns the value for the left control key.
	pub const fn lctrl(self) -> bool {
		self.0.0 & Self::LCTRL.0.0 != 0
	}

	/// Returns the value for the right control key.
	pub const fn rctrl(self) -> bool {
		self.0.0 & Self::RCTRL.0.0 != 0
	}

	/// Returns the value for the alt keys.
	pub const fn alt(self) -> bool {
		self.0.0 & Self::ALT.0.0 != 0
	}

	/// Returns the value for the left alt key.
	pub const fn lalt(self) -> bool {
		self.0.0 & Self::LALT.0.0 != 0
	}

	/// Returns the value for the right alt key.
	pub const fn ralt(self) -> bool {
		self.0.0 & Self::RALT.0.0 != 0
	}

	/// Returns the value for the super keys.
	///
	/// Named `super_` to avoid conflict with the `super` keyword.
	pub const fn super_(self) -> bool {
		self.0.0 & Self::SUPER.0.0 != 0
	}

	/// Returns the value for the left super key.
	pub const fn lsuper(self) -> bool {
		self.0.0 & Self::LSUPER.0.0 != 0
	}

	/// Returns the value for the right super key.
	pub const fn rsuper(self) -> bool {
		self.0.0 & Self::RSUPER.0.0 != 0
	}

}

impl From<SDL_Keymod> for KeyMod {

	fn from(keymod: SDL_Keymod) -> Self {
		Self(keymod)
	}

}