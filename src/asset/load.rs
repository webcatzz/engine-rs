//! Traits for loading and parsing types.

use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::Path;

/// Types that can be loaded from the filesystem.
pub trait Load: Sized {
	// Additional parameters for loading.
	type Params<'a>;
	/// Loads from the filesystem.
	fn load(path: impl AsRef<Path>, params: Self::Params<'_>) -> io::Result<Self>;
}

/// Types that can be parsed from bytes.
///
/// Generates a blanket implementation for [`Load`].
pub trait FromBytes: Sized {
	// Additional parameters for parsing.
	type Params<'a>;
	/// Parses from bytes.
	fn from_bytes(bytes: &mut (impl Read + Seek), params: Self::Params<'_>) -> io::Result<Self>;
}

impl<T: FromBytes> Load for T {

	type Params<'a> = <Self as FromBytes>::Params<'a>;

	fn load(path: impl AsRef<Path>, params: Self::Params<'_>) -> io::Result<Self> {
		File::open(path).and_then(|mut file| Self::from_bytes(&mut file, params))
	}

}