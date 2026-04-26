//! Generic colors
//!
//! Currently targets [`u8`] and [`f32`] representations.

use std::ffi::c_float;
use std::fmt;
use std::num::ParseIntError;
use std::ops::{Sub, AddAssign};
use sdl3_sys::pixels::{SDL_Color, SDL_FColor};
use crate::math::{Cast, Lerp};

/// An RGBA color.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color<C: ColorComponent> {
	/// Red component.
	pub r: C,
	/// Green component.
	pub g: C,
	/// Blue component.
	pub b: C,
	/// Alpha component.
	pub a: C,
}

impl<C: ColorComponent> Color<C> {

	/// Absolute black.
	pub const BLACK: Self = Self::from_rgb(C::MIN, C::MIN, C::MIN);
	/// Absolute white.
	pub const WHITE: Self = Self::from_rgb(C::MAX, C::MAX, C::MAX);
	/// Absolute red.
	pub const RED:   Self = Self::from_rgb(C::MAX, C::MIN, C::MIN);
	/// Absolute green.
	pub const GREEN: Self = Self::from_rgb(C::MIN, C::MAX, C::MIN);
	/// Absolute blue.
	pub const BLUE:  Self = Self::from_rgb(C::MIN, C::MIN, C::MAX);

	/// Returns a new color with its `r`, `g`, and `b` components set.
	pub const fn from_rgb(r: C, g: C, b: C) -> Self {
		Self { r, g, b, a: C::MAX }
	}

	/// Returns a new color with its `r`, `g`, `b`, and `a` components set.
	pub const fn from_rgba(r: C, g: C, b: C, a: C) -> Self {
		Self { r, g, b, a }
	}

	/// Returns a new grayscale color with some value.
	pub const fn from_value(v: C) -> Self {
		Self { r: v, g: v, b: v, a: C::MAX }
	}

	/// Returns a transparent white color with some alpha.
	pub const fn from_alpha(a: C) -> Self {
		Self { r: C::MAX, g: C::MAX, b: C::MAX, a }
	}

	// /// Converts a hexcode to a color.
	// ///
	// /// Supports 6 and 8 character hexcodes. Automatically strips '#' prefixes.
	// /// Panics on invalid lengths.
	// pub fn from_hex(hex: &str) -> Result<Self, C::FromHexError> {
	// 	let hex = hex.strip_prefix('#').unwrap_or(hex);
	// 	match hex.len() {
	// 		6 => Ok(Self::from_rgb(
	// 			C::from_hex(&hex[0..2])?,
	// 			C::from_hex(&hex[2..4])?,
	// 			C::from_hex(&hex[4..6])?,
	// 		)),
	// 		8 => Ok(Self::from_rgba(
	// 			C::from_hex(&hex[0..2])?,
	// 			C::from_hex(&hex[2..4])?,
	// 			C::from_hex(&hex[4..6])?,
	// 			C::from_hex(&hex[6..8])?,
	// 		)),
	// 		_ => panic!("hexcode should be of valid length"),
	// 	}
	// }

	/// Returns the `r`, `g`, and `b` components of a color.
	pub const fn rgb(&self) -> (C, C, C) {
		(self.r, self.g, self.b)
	}

	/// Returns the `r`, `g`, `b`, and `a` components of a color.
	pub const fn rgba(&self) -> (C, C, C, C) {
		(self.r, self.g, self.b, self.a)
	}

	/// Converts the color to [`u8`] representation.
	pub fn to_u8(&self) -> Color<u8> {
		Color {
			r: self.r.to_u8(),
			g: self.g.to_u8(),
			b: self.b.to_u8(),
			a: self.a.to_u8(),
		}
	}

	/// Converts the color to [`f32`] representation.
	pub fn to_f32(&self) -> Color<f32> {
		Color {
			r: self.r.to_f32(),
			g: self.g.to_f32(),
			b: self.b.to_f32(),
			a: self.a.to_f32(),
		}
	}

	/// Converts the color to [`f64`] representation.
	pub fn to_f64(&self) -> Color<f64> {
		Color {
			r: self.r.to_f64(),
			g: self.g.to_f64(),
			b: self.b.to_f64(),
			a: self.a.to_f64(),
		}
	}

}

impl Color<u8> {

	/// Converts a hexcode of the form 0xRRGGBB to a color.
	pub const fn from_hex_rgb(v: u32) -> Self {
		Color {
			r: ((v >> 16) & 0xff) as u8,
			g: ((v >> 8)  & 0xff) as u8,
			b: (v         & 0xff) as u8,
			a: u8::MAX,
		}
	}

	/// Converts a hexcode of the form 0xRRGGBBAA to a color.
	pub const fn from_hex_rgba(v: u32) -> Self {
		Color {
			r: ((v >> 24) & 0xff) as u8,
			g: ((v >> 16) & 0xff) as u8,
			b: ((v >> 8)  & 0xff) as u8,
			a: (v         & 0xff) as u8,
		}
	}

	/// Converts a hexcode of the form "RRGGBB" or "RRGGBBAA" to a color.
	pub fn from_hex_str(str: &str) -> Result<Self, ParseIntError> {
		Ok(Color {
			r: u8::from_str_radix(&str[0..2], 16)?,
			g: u8::from_str_radix(&str[2..4], 16)?,
			b: u8::from_str_radix(&str[4..6], 16)?,
			a: match str.len() >= 8 {
				true => u8::from_str_radix(&str[6..8], 16)?,
				false => u8::MAX,
			},
		})
	}

}

impl<C: ColorComponent + fmt::Display> fmt::Debug for Color<C> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Color(#{:02x}{:02x}{:02x}{:02x})", self.r.to_u8(), self.g.to_u8(), self.b.to_u8(), self.a.to_u8())
	}
}

impl<C: ColorComponent + Cast + Clone + AddAssign + Sub<Output = C>> Lerp<f32> for Color<C> {
	fn lerp(mut self, to: &Self, by: f32) -> Self {
		self.r += ((to.r - self.r).as_f32() * by).cast();
		self.g += ((to.g - self.g).as_f32() * by).cast();
		self.b += ((to.b - self.b).as_f32() * by).cast();
		self
	}
}

// Conversions

impl<C: ColorComponent> From<(C, C, C)> for Color<C> {
	fn from(value: (C, C, C)) -> Self {
		Self::from_rgb(value.0, value.1, value.2)
	}
}

impl<C: ColorComponent> From<(C, C, C, C)> for Color<C> {
	fn from(value: (C, C, C, C)) -> Self {
		Self::from_rgba(value.0, value.1, value.2, value.3)
	}
}

impl<C: ColorComponent> From<[C; 3]> for Color<C> {
	fn from(value: [C; 3]) -> Self {
		Self::from_rgb(value[0], value[1], value[2])
	}
}

impl<C: ColorComponent> From<[C; 4]> for Color<C> {
	fn from(value: [C; 4]) -> Self {
		Self::from_rgba(value[0], value[1], value[2], value[3])
	}
}

impl<C: ColorComponent> Into<(C, C, C)> for Color<C> {
	fn into(self) -> (C, C, C) {
		self.rgb()
	}
}

impl<C: ColorComponent> Into<(C, C, C, C)> for Color<C> {
	fn into(self) -> (C, C, C, C) {
		self.rgba()
	}
}

impl<C: ColorComponent> Into<[C; 3]> for Color<C> {
	fn into(self) -> [C; 3] {
		self.rgb().into()
	}
}

impl<C: ColorComponent> Into<[C; 4]> for Color<C> {
	fn into(self) -> [C; 4] {
		self.rgba().into()
	}
}

impl Into<SDL_Color> for Color<u8> {
	fn into(self) -> SDL_Color {
		SDL_Color { r: self.r, g: self.g, b: self.b, a: self.a }
	}
}

impl Into<SDL_FColor> for Color<f32> {
	fn into(self) -> SDL_FColor {
		SDL_FColor { r: self.r as c_float, g: self.g as c_float, b: self.b as c_float, a: self.a as c_float }
	}
}

/// Types that can be components of a color.
pub trait ColorComponent: Copy + Cast + PartialEq {

	/// The minimum value of the component.
	const MIN: Self;
	/// The maximum value of the component.
	const MAX: Self;

	/// Converts the component to [`u8`] representation.
	fn to_u8(self) -> u8;
	/// Converts the component to [`f32`] representation.
	fn to_f32(self) -> f32;
	/// Converts the component to [`f64`] representation.
	fn to_f64(self) -> f64;

}

impl ColorComponent for u8 {

	const MIN: Self = 0;
	const MAX: Self = 255;

	fn to_u8(self) -> u8 {
		self
	}

	fn to_f32(self) -> f32 {
		self as f32 / 255.0
	}

	fn to_f64(self) -> f64 {
		self as f64 / 255.0
	}

}

impl ColorComponent for f32 {

	const MIN: Self = 0.0;
	const MAX: Self = 1.0;

	fn to_u8(self) -> u8 {
		(self * 255.0) as u8
	}

	fn to_f32(self) -> f32 {
		self
	}

	fn to_f64(self) -> f64 {
		self as f64
	}

}

impl ColorComponent for f64 {

	const MIN: Self = 0.0;
	const MAX: Self = 1.0;

	fn to_u8(self) -> u8 {
		(self * 255.0) as u8
	}

	fn to_f32(self) -> f32 {
		self as f32
	}

	fn to_f64(self) -> f64 {
		self
	}

}