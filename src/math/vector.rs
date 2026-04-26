//! Generic vector math
//!
//! Defines two, three, and four-dimensional vector types [`Vec2`], [`Vec3`],
//! and [`Vec4`].

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Index, IndexMut};
use sdl3_sys::rect::*;
use super::{Axis, Cast, One, Zero};

/// Defines a generic vector type with arbitrary components.
macro_rules! impl_vector {
	($(#[$attr:meta])* $ty:ident($($c:ident),*);) => {

		$(#[$attr])*
		#[derive(Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
		pub struct $ty<T> {
			$(pub $c: T),*
		}

		impl<T> $ty<T> {

			#[deprecated]
			pub const fn new($($c: T),*) -> Self {
				Self { $( $c, )* }
			}

		}

		impl<T: Copy> $ty<T> {

			/// Returns a vector with all components set to `v`.
			pub const fn diagonal(v: T) -> Self {
				Self { $( $c: v, )* }
			}

		}

		impl<T: Cast> $ty<T> {

			pub fn as_i8(self) -> $ty<i8> {
				$ty { $( $c: self.$c.as_i8(), )* }
			}

			pub fn as_i16(self) -> $ty<i16> {
				$ty { $( $c: self.$c.as_i16(), )* }
			}

			pub fn as_i32(self) -> $ty<i32> {
				$ty { $( $c: self.$c.as_i32(), )* }
			}

			pub fn as_i64(self) -> $ty<i64> {
				$ty { $( $c: self.$c.as_i64(), )* }
			}

			pub fn as_i128(self) -> $ty<i128> {
				$ty { $( $c: self.$c.as_i128(), )* }
			}

			pub fn as_isize(self) -> $ty<isize> {
				$ty { $( $c: self.$c.as_isize(), )* }
			}

			pub fn as_u8(self) -> $ty<u8> {
				$ty { $( $c: self.$c.as_u8(), )* }
			}

			pub fn as_u16(self) -> $ty<u16> {
				$ty { $( $c: self.$c.as_u16(), )* }
			}

			pub fn as_u32(self) -> $ty<u32> {
				$ty { $( $c: self.$c.as_u32(), )* }
			}

			pub fn as_u64(self) -> $ty<u64> {
				$ty { $( $c: self.$c.as_u64(), )* }
			}

			pub fn as_u128(self) -> $ty<u128> {
				$ty { $( $c: self.$c.as_u128(), )* }
			}

			pub fn as_usize(self) -> $ty<usize> {
				$ty { $( $c: self.$c.as_usize(), )* }
			}

			pub fn as_f32(self) -> $ty<f32> {
				$ty { $( $c: self.$c.as_f32(), )* }
			}

			pub fn as_f64(self) -> $ty<f64> {
				$ty { $( $c: self.$c.as_f64(), )* }
			}

			pub fn cast<U: Cast>(self) -> $ty<U> {
				$ty { $( $c: self.$c.cast(), )* }
			}

		}

		impl<T: Zero> $ty<T> {

			/// A vector with all components set to zero.
			pub const ZERO: Self = Self { $( $c: T::ZERO, )* };

		}

		impl<T: One> $ty<T> {

			/// A vector with all components set to one.
			pub const ONE: Self = Self { $( $c: T::ONE, )* };

		}

		impl<T: Add<Output = T>> Add for $ty<T> {

			type Output = Self;

			fn add(mut self, rhs: Self) -> Self::Output {
				$( self.$c = self.$c + rhs.$c; )*
				self
			}

		}

		impl<T: Sub<Output = T>> Sub for $ty<T> {

			type Output = Self;

			fn sub(mut self, rhs: Self) -> Self::Output {
				$( self.$c = self.$c - rhs.$c; )*
				self
			}

		}

		impl<T: Mul<Output = T>> Mul for $ty<T> {

			type Output = Self;

			fn mul(mut self, rhs: Self) -> Self::Output {
				$( self.$c = self.$c * rhs.$c; )*
				self
			}

		}

		impl<T: Mul<Output = T> + Copy> Mul<T> for $ty<T> {

			type Output = Self;

			fn mul(mut self, rhs: T) -> Self::Output {
				$( self.$c = self.$c * rhs; )*
				self
			}

		}

		impl<T: Div<Output = T>> Div for $ty<T> {

			type Output = Self;

			fn div(mut self, rhs: Self) -> Self::Output {
				$( self.$c = self.$c / rhs.$c; )*
				self
			}

		}

		impl<T: Div<Output = T> + Copy> Div<T> for $ty<T> {

			type Output = Self;

			fn div(mut self, rhs: T) -> Self::Output {
				$( self.$c = self.$c / rhs; )*
				self
			}

		}

		impl<T: Rem<Output = T>> Rem for $ty<T> {

			type Output = Self;

			fn rem(mut self, rhs: Self) -> Self::Output {
				$( self.$c = self.$c % rhs.$c; )*
				self
			}

		}

		impl<T: Rem<Output = T> + Copy> Rem<T> for $ty<T> {

			type Output = Self;

			fn rem(mut self, rhs: T) -> Self::Output {
				$( self.$c = self.$c % rhs; )*
				self
			}

		}

		impl<T: Neg<Output = T>> Neg for $ty<T> {

			type Output = Self;

			fn neg(mut self) -> Self::Output {
				$( self.$c = -self.$c; )*
				self
			}

		}

		impl<T: AddAssign> AddAssign for $ty<T> {

			fn add_assign(&mut self, rhs: Self) {
				$( self.$c += rhs.$c; )*
			}

		}

		impl<T: SubAssign> SubAssign for $ty<T> {

			fn sub_assign(&mut self, rhs: Self) {
				$( self.$c -= rhs.$c; )*
			}

		}

		impl<T: MulAssign> MulAssign for $ty<T> {

			fn mul_assign(&mut self, rhs: Self) {
				$( self.$c *= rhs.$c; )*
			}

		}

		impl<T: MulAssign + Copy> MulAssign<T> for $ty<T> {

			fn mul_assign(&mut self, rhs: T) {
				$( self.$c *= rhs; )*
			}

		}

		impl<T: DivAssign> DivAssign for $ty<T> {

			fn div_assign(&mut self, rhs: Self) {
				$( self.$c /= rhs.$c; )*
			}

		}

		impl<T: DivAssign + Copy> DivAssign<T> for $ty<T> {

			fn div_assign(&mut self, rhs: T) {
				$( self.$c /= rhs; )*
			}

		}

		impl<T: RemAssign> RemAssign for $ty<T> {

			fn rem_assign(&mut self, rhs: Self) {
				$( self.$c %= rhs.$c; )*
			}

		}

		impl<T: RemAssign + Copy> RemAssign<T> for $ty<T> {

			fn rem_assign(&mut self, rhs: T) {
				$( self.$c %= rhs; )*
			}

		}

		impl<T: fmt::Display> fmt::Debug for $ty<T> {

			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "(")?;
				write_vector!(f, self($( $c ),*));
				write!(f, ")")
			}

		}

	};
}

/// Implements [`Index`] and [`IndexMut`] for a vector type.
macro_rules! impl_index {
	(
		$ty:ident[$ity:ty] {
			$( $i:pat => $v:ident ),* $(,)?
		}
	) => {

		impl<T> Index<$ity> for $ty<T> {
			type Output = T;
			fn index(&self, index: $ity) -> &Self::Output {
				#[allow(unreachable_patterns)]
				match index {
					$( $i => &self.$v, )*
					i => panic!("Invalid index: {i:?}"),
				}
			}
		}

		impl<T> IndexMut<$ity> for $ty<T> {
			fn index_mut(&mut self, index: $ity) -> &mut Self::Output {
				#[allow(unreachable_patterns)]
				match index {
					$( $i => &mut self.$v, )*
					i => panic!("Invalid index: {i:?}"),
				}
			}
		}

	};
}

/// Writes a vector type to a formatter with the form `(x, y, ..., z)`.
macro_rules! write_vector {
	($f:ident, $self:ident($c: ident, $($cs:ident),+)) => {
		write!($f, "{}, ", $self.$c)?;
		write_vector!($f, $self($($cs),+));
	};
	($f:ident, $self:ident($c:ident)) => {
		write!($f, "{}", $self.$c)?;
	};
}

impl_vector! {
	/// A two-dimensional vector.
	Vec2(x, y);
}

impl_vector! {
	/// A three-dimensional vector.
	Vec3(x, y, z);
}

impl_vector! {
	/// A four-dimensional vector.
	Vec4(x, y, z, w);
}

impl<T> From<(T, T)> for Vec2<T> {

	fn from(value: (T, T)) -> Self {
		Self { x: value.0, y: value.1 }
	}

}

impl<T> From<(T, T, T)> for Vec3<T> {

	fn from(value: (T, T, T)) -> Self {
		Self { x: value.0, y: value.1, z: value.2 }
	}

}

impl<T> From<(T, T, T, T)> for Vec4<T> {

	fn from(value: (T, T, T, T)) -> Self {
		Self { x: value.0, y: value.1, z: value.2, w: value.3 }
	}

}

impl<T> Into<(T, T)> for Vec2<T> {

	fn into(self) -> (T, T) {
		(self.x, self.y)
	}

}

impl<T> Into<(T, T, T)> for Vec3<T> {

	fn into(self) -> (T, T, T) {
		(self.x, self.y, self.z)
	}

}

impl<T> Into<(T, T, T, T)> for Vec4<T> {

	fn into(self) -> (T, T, T, T) {
		(self.x, self.y, self.z, self.w)
	}

}

impl<T: Cast> Into<SDL_Point> for Vec2<T> {

	fn into(self) -> SDL_Point {
		SDL_Point { x: self.x.cast(), y: self.y.cast() }
	}

}

impl<T: Cast> Into<SDL_FPoint> for Vec2<T> {

	fn into(self) -> SDL_FPoint {
		SDL_FPoint { x: self.x.cast(), y: self.y.cast() }
	}

}

impl<T> Vec2<T> {

	pub fn with_x(mut self, x: T) -> Self {
		self.x = x;
		self
	}

	pub fn with_y(mut self, y: T) -> Self {
		self.y = y;
		self
	}

	pub fn with_z(self, z: T) -> Vec3<T> {
		Vec3 { x: self.x, y: self.y, z }
	}

}

impl_index!(Vec2[u32] {
	0 => x,
	1 => y,
});

impl_index!(Vec2[Axis] {
	Axis::X => x,
	Axis::Y => y,
});

// Additional implementations

// use crate::util::input::InputAction;

// impl Vec2<InputAction> {
// 	pub fn dir()
// }