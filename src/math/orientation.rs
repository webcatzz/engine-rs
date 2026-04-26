use std::ops::Neg;
use super::{Vec2, One, Zero};

/// 2D axes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Axis {
	X,
	Y,
}

impl Axis {

	/// Returns the other axis.
	pub const fn other(self) -> Self {
		match self {
			Self::X => Self::Y,
			Self::Y => Self::X,
		}
	}

	/// Returns an iterator over [`X`](Axis::X) and [`Y`](Axis::Y), in that order.
	pub fn iter() -> impl Iterator<Item = Self> {
		[Self::X, Self::Y].into_iter()
	}

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {

	/// Returns the unit vector representing a direction.
	pub fn unit<T: One + Zero + Neg<Output = T>>(self) -> Vec2<T> {
		match self {
			Self::Up    => Vec2 { x: T::ZERO, y: -T::ONE },
			Self::Down  => Vec2 { x: T::ZERO, y: T::ONE  },
			Self::Left  => Vec2 { x: -T::ONE, y: T::ZERO },
			Self::Right => Vec2 { x: T::ONE,  y: T::ZERO },
		}
	}

}

pub enum RadialDir {
	Clockwise,
	CounterClockwise,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Corner {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
}

pub enum Face {
	Front,
	Back,
}