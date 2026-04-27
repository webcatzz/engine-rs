//! Generic rectangles

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use sdl3_sys::rect::SDL_FRect;
use super::{Axis, Cast, Dir, One, Transform, Vec2, Zero};

/// A rectangle.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T, U = T> {
	/// The top-left corner of the rectangle.
	pub pos: Vec2<T>,
	/// The size of the rectangle.
	pub size: Vec2<U>,
}

impl<T: Copy, U: Copy> Rect<T, U> {

	/// Returns a rectangle with some position.
	pub fn with_pos(mut self, pos: Vec2<T>) -> Self {
		self.pos = pos;
		self
	}

	/// Returns a rectangle with some size.
	pub fn with_size(mut self, size: Vec2<U>) -> Self {
		self.size = size;
		self
	}

}

// constants

impl<T: Zero> Rect<T> {

	/// A rectangle starting at the origin with no size.
	pub const ZERO: Self = Self { pos: Vec2::ZERO, size: Vec2::ZERO };

}

impl<T: One + Zero> Rect<T> {

	/// A rectangle starting at the origin with unit size.
	pub const ONE: Self = Self { pos: Vec2::ZERO, size: Vec2::ONE };

}

impl<T: Copy + Add<Output = T>> Rect<T> {

	/// Returns the bottom-right corner of a rect.
	pub fn end(&self) -> Vec2<T> {
		self.pos + self.size
	}

	/// Returns the `x` coordinate of the bottom-right corner of a rectangle.
	pub fn end_x(&self) -> T {
		self.pos.x + self.size.x
	}

	/// Returns the `y` coordinate of the bottom-right corner of a rectangle.
	pub fn end_y(&self) -> T {
		self.pos.y + self.size.y
	}

}

impl<T: Copy + Mul<Output = T>> Rect<T> {

	/// Returns the area of a rectangle.
	pub fn area(&self) -> T {
		self.size.x * self.size.y
	}

}

impl<T: Copy + PartialOrd + Add<Output = T>> Rect<T> {

	/// Returns true if a point is within a given rectangle.
	pub fn contains_point(self, point: Vec2<T>) -> bool {
		point >= self.pos && point <= self.end()
	}

}

impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T>> Rect<T> {

	/// Returns a new rectangle enclosing a given point.
	pub fn expand(mut self, pos: Vec2<T>) -> Self {
		for axis in Axis::iter() {
			if pos[axis] < self.pos[axis] {
				self.pos[axis] = pos[axis];
			} else {
				self.size[axis] = self.size[axis].min(pos[axis] - self.pos[axis]);
			}
		}
		self
	}

}

impl<T: Copy + AddAssign + SubAssign + Add<Output = T>> Rect<T> {

	/// Expands a rect on all sides.
	pub fn grow(mut self, by: T) -> Self {
		let vec = Vec2 { x: by, y: by };
		self.pos  -= vec;
		self.size += vec + vec;
		self
	}

	/// Expands a rect on all sides.
	pub fn grow_sides(mut self, top: T, right: T, bottom: T, left: T) -> Self {
		self.pos.x  -= left;
		self.pos.y  -= top;
		self.size.x += left + right;
		self.size.y += top + bottom;
		self
	}

	/// expands one side of a rect
	pub fn grow_side(mut self, dir: Dir, by: T) -> Self {
		match dir {
			Dir::Left  => self.pos.x  -= by,
			Dir::Right => self.size.x += by,
			Dir::Up    => self.pos.y  -= by,
			Dir::Down  => self.size.y += by,
		}
		self
	}

}

impl Rect<f32> {

	/// Transforms the rectangle.
	pub fn transform(mut self, transform: Transform) -> Self {
		self.pos = transform.transform(self.pos.as_f32());
		self.size = transform.multiply(self.size.as_f32());
		self
	}

}

// conversions

impl<T: Copy + Cast, U: Copy + Cast> Rect<T, U> {

	pub fn cast<V: Copy + Cast, W: Copy + Cast>(self) -> Rect<V, W> {
		Rect {
			pos: self.pos.cast(),
			size: self.size.cast(),
		}
	}

}

impl<T: Cast, U: Cast> Into<SDL_FRect> for Rect<T, U> {

	fn into(self) -> SDL_FRect {
		SDL_FRect {
			x: self.pos.x.cast(),
			y: self.pos.y.cast(),
			w: self.size.x.cast(),
			h: self.size.y.cast(),
		}
	}

}