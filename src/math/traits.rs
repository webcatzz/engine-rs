//! Mathematical traits.

/// Generalizes over numerical types that may be cast with the `as` operator.
///
/// Offers `as_*()` functions that cast to a specific numerical type, as well as
/// a generic [`cast()`](Cast::cast) function that casts to any other
/// implementor of [`Cast`]. All trait functions decompose to inline `as` casts.
///
/// Prefer using the `as_*()` functions wherever possible for readability.
pub trait Cast: Sized {

	/// Casts to [`i8`].
	fn as_i8(self) -> i8;
	/// Casts to [`i16`].
	fn as_i16(self) -> i16;
	/// Casts to [`i32`].
	fn as_i32(self) -> i32;
	/// Casts to [`i64`].
	fn as_i64(self) -> i64;
	/// Casts to [`i128`].
	fn as_i128(self) -> i128;
	/// Casts to [`isize`].
	fn as_isize(self) -> isize;

	/// Casts to [`u8`].
	fn as_u8(self) -> u8;
	/// Casts to [`u16`].
	fn as_u16(self) -> u16;
	/// Casts to [`u32`].
	fn as_u32(self) -> u32;
	/// Casts to [`u64`].
	fn as_u64(self) -> u64;
	/// Casts to [`u128`].
	fn as_u128(self) -> u128;
	/// Casts to [`usize`].
	fn as_usize(self) -> usize;

	/// Casts to [`f32`].
	fn as_f32(self) -> f32;
	/// Casts to [`f64`].
	fn as_f64(self) -> f64;

	/// Casts from another type.
	fn cast_from<P: Cast>(value: P) -> Self;

	/// Casts to another type.
	#[inline(always)]
	fn cast<P: Cast>(self) -> P {
		P::cast_from(self)
	}

}

/// Implements [`Cast`] for a type based on a casting function.
macro_rules! impl_cast {
	($ty:ty, $as:ident) => {
		impl Cast for $ty {

			#[inline(always)]
			fn as_i8(self) -> i8 {
				self as i8
			}

			#[inline(always)]
			fn as_i16(self) -> i16 {
				self as i16
			}

			#[inline(always)]
			fn as_i32(self) -> i32 {
				self as i32
			}

			#[inline(always)]
			fn as_i64(self) -> i64 {
				self as i64
			}

			#[inline(always)]
			fn as_i128(self) -> i128 {
				self as i128
			}

			#[inline(always)]
			fn as_isize(self) -> isize {
				self as isize
			}

			#[inline(always)]
			fn as_u8(self) -> u8 {
				self as u8
			}

			#[inline(always)]
			fn as_u16(self) -> u16 {
				self as u16
			}

			#[inline(always)]
			fn as_u32(self) -> u32 {
				self as u32
			}

			#[inline(always)]
			fn as_u64(self) -> u64 {
				self as u64
			}

			#[inline(always)]
			fn as_u128(self) -> u128 {
				self as u128
			}

			#[inline(always)]
			fn as_usize(self) -> usize {
				self as usize
			}

			#[inline(always)]
			fn as_f32(self) -> f32 {
				self as f32
			}

			#[inline(always)]
			fn as_f64(self) -> f64 {
				self as f64
			}

			#[inline(always)]
			fn cast_from<P: Cast>(value: P) -> Self {
				value.$as()
			}

		}
	};
}

impl_cast!(i8,    as_i8);
impl_cast!(i16,   as_i16);
impl_cast!(i32,   as_i32);
impl_cast!(i64,   as_i64);
impl_cast!(i128,  as_i128);
impl_cast!(isize, as_isize);
impl_cast!(u8,    as_u8);
impl_cast!(u16,   as_u16);
impl_cast!(u32,   as_u32);
impl_cast!(u64,   as_u64);
impl_cast!(u128,  as_u128);
impl_cast!(usize, as_usize);
impl_cast!(f32,   as_f32);
impl_cast!(f64,   as_f64);

/// Types that can be linearly interpolated.
pub trait Lerp<T> {
	fn lerp(self, to: &Self, by: T) -> Self;
}

impl Lerp<f32> for f32 {
	fn lerp(self, to: &Self, by: f32) -> Self {
		self + (to - self) * by
	}
}

impl Lerp<f64> for f64 {
	fn lerp(self, to: &Self, by: f64) -> Self {
		self + (to - self) * by
	}
}

/// Types that have a zero value.
pub trait Zero {
	const ZERO: Self;
}

macro_rules! impl_zero {
	($ty:ty, $zero:expr) => {
		impl Zero for $ty {
			const ZERO: Self = $zero;
		}
	};
}

impl_zero!(i8,    0);
impl_zero!(i16,   0);
impl_zero!(i32,   0);
impl_zero!(i64,   0);
impl_zero!(i128,  0);
impl_zero!(isize, 0);
impl_zero!(u8,    0);
impl_zero!(u16,   0);
impl_zero!(u32,   0);
impl_zero!(u64,   0);
impl_zero!(u128,  0);
impl_zero!(usize, 0);
impl_zero!(f32,   0.0);
impl_zero!(f64,   0.0);

/// Types that have a one value.
pub trait One {
	const ONE: Self;
}

macro_rules! impl_one {
	($ty:ty, $one:expr) => {
		impl One for $ty {
			const ONE: Self = $one;
		}
	};
}

impl_one!(i8,    1);
impl_one!(i16,   1);
impl_one!(i32,   1);
impl_one!(i64,   1);
impl_one!(i128,  1);
impl_one!(isize, 1);
impl_one!(u8,    1);
impl_one!(u16,   1);
impl_one!(u32,   1);
impl_one!(u64,   1);
impl_one!(u128,  1);
impl_one!(usize, 1);
impl_one!(f32,   1.0);
impl_one!(f64,   1.0);