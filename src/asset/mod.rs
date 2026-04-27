//! Asset loading and caching.
//!
//! Assets are immutable data that can be loaded from the filesystem or from a
//! stream of bytes. Some useful asset types are defined below. To load an asset
//! from the filesystem, see the [`load()`] shorthand.
//!
//! Repeatedly loading the same asset is inefficient. A [`Cached`] handle will
//! load it once and store the result in a global asset cache, using the asset's
//! path as a unique identifier. Any subsequent attempts to load the asset will
//! first look it up in the cache before performing an expensive load. To load
//! an asset using the cache, see the [`load_cached()`] shorthand.
//!
//! To avoid unnecessary lifetime management and resource duplication, most
//! types that need access to an asset ask for a [`Cached`] handle.

pub mod ase;
mod audio;
mod batch;
mod cache;
mod font;
mod image;
mod load;
mod texture;

pub use ase::Aseprite;
pub use audio::*;
pub use batch::*;
pub use cache::*;
pub use font::*;
pub use image::*;
pub use load::*;
pub use texture::*;

use std::io;
use std::path::Path;

/// Convenient shorthand for loading [`Load`] types without requiring [`Load`]
/// to be in scope.
pub fn load<T: Load>(path: impl AsRef<Path>, params: T::Params<'_>) -> io::Result<T> {
	T::load(path, params)
}

/// Convenient shorthand for loading [`Load`] types without requiring [`Load`]
/// to be in scope. Caches results.
///
/// See [`Cached`].
pub fn load_cached<T>(path: impl AsRef<Path>, params: T::Params<'_>) -> io::Result<Cached<T>>
where T: Load + Send + Sync + 'static
{
	Cached::load(path, params)
}