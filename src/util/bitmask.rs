//! Bitmasks.

/// Defines a bitmask type.
///
/// Implements bitwise operations, [`Copy`], [`Eq`], and [`Hash`] for the type.
///
/// # Syntax
///
/// Defines a bitmask with three flags:
///
/// ```no_run
/// bitmask! {
///   pub Bitmask(u32) {
///     pub ONE   = Self(1 << 0),
///     pub TWO   = Self(1 << 1),
///     pub THREE = Self(1 << 2),
///   }
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! bitmask {
	(
		$(#[$attr:meta])*
		$vis:vis $ty:ident($raw_vis:vis $raw_ty:ty) {
			$( $(#[$flag_attr:meta])* $flag_vis:vis $flag:ident = $flag_val:expr ),* $(,)?
		}
	) => {

		#[derive(Clone, Copy, PartialEq, Eq, Hash)]
		$(#[$attr])*
		$vis struct $ty($raw_vis $raw_ty);

		impl $ty {

			$( $(#[$flag_attr])* $flag_vis const $flag: Self = $flag_val; )*

		}

		impl std::ops::BitAnd for $ty {

			type Output = Self;

			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}

		}

		impl std::ops::BitOr for $ty {

			type Output = Self;

			fn bitor(self, rhs: Self) -> Self::Output {
				Self(self.0 | rhs.0)
			}

		}

		impl std::ops::BitXor for $ty {

			type Output = Self;

			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}

		}

		impl std::ops::BitAndAssign for $ty {

			fn bitand_assign(&mut self, rhs: Self) {
				self.0 &= rhs.0
			}

		}

		impl std::ops::BitOrAssign for $ty {

			fn bitor_assign(&mut self, rhs: Self) {
				self.0 |= rhs.0
			}

		}

		impl std::ops::BitXorAssign for $ty {

			fn bitxor_assign(&mut self, rhs: Self) {
				self.0 ^= rhs.0
			}

		}

	};
}

pub use bitmask;