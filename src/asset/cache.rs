//! Asset caching.

// NOTE: Cached paths are stored twice, once in their `Cache` and once in
// `ASSET_CACHE`. Could a `HashSet` + custom hashing for `Cache` prevent that?

use std::any::Any;
use std::collections::hash_map::{HashMap, Entry};
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::{Arc, Weak, LazyLock, Mutex};
use super::Load;

/// Stores data for cached paths.
pub(super) type AssetCache = HashMap<PathBuf, Weak<dyn Any + Send + Sync>>;

/// The global asset cache.
///
/// New [`Cache`]s register themselves to this cache and remove themselves when
/// they are dropped. Stored [`Weak`] references always reference a valid
/// [`Cache`].
pub(super) static ASSET_CACHE: LazyLock<Mutex<AssetCache>> = LazyLock::new(|| Mutex::new(AssetCache::new()));

/// A handle for a cached asset.
///
/// Cloning cheaply duplicates the handle. When no more handles for the asset
/// exist, it will be dropped.
pub struct Cached<T: Send + Sync>(Arc<Cache<T>>);

impl<T: Send + Sync> Cached<T> {

	/// Returns a [`Cached`] handle for a [`Cache`].
	pub(super) fn handle(cache: Arc<Cache<T>>) -> Self {
		Self(cache)
	}

}

impl<T: Send + Sync> Clone for Cached<T> {

	fn clone(&self) -> Self {
		Self(self.0.clone())
	}

}

impl<T: Load + Send + Sync + 'static> Load for Cached<T> {

	type Params<'a> = T::Params<'a>;

	fn load(path: impl AsRef<Path>, params: Self::Params<'_>) -> io::Result<Self> {
		let cache = &mut *ASSET_CACHE.lock().unwrap();
		Cache::get_or_load(path.as_ref().to_owned(), params, cache).map(Self)
	}

}

impl<T: Send + Sync> AsRef<T> for Cached<T> {

	fn as_ref(&self) -> &T {
		&self.0.asset
	}

}

impl<T: Send + Sync> Deref for Cached<T> {

	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0.asset
	}

}

/// Stores an asset and its identifying path.
///
/// Removes the entry for its path from [`ASSET_CACHE`] when dropped.
pub(super) struct Cache<T> {
	asset: T,
	path: PathBuf,
}

impl<T: Load + Send + Sync + 'static> Cache<T> {

	/// Returns a cache for the asset at the given path.
	///
	/// If the path is cached in `cache`, returns the cached value. Otherwise,
	/// loads the asset from the filesystem and caches it in `cache`.
	///
	/// `cache` should always be a reference to [`ASSET_CACHE`]. Passing in any
	/// other cache may result in unintuitive behavior, since [`Cache`] removes
	/// itself from [`ASSET_CACHE`] when dropped regardless of the value of
	/// `cache`.
	pub fn get_or_load(path: PathBuf, params: T::Params<'_>, cache: &mut AssetCache) -> io::Result<Arc<Self>> {
		// Hacky assert that `cache` references `ASSET_CACHE` without locking
		// WARN: May start to fail if the memory layout of `Mutex` changes (2026/04/23)
		unsafe { debug_assert_eq!(ptr::from_ref(cache).byte_sub(16), ptr::from_ref(&*ASSET_CACHE) as *const _, "`cache` should reference `ASSET_CACHE`"); }
		match cache.entry(path) {
			Entry::Occupied(entry) =>
				Ok(entry.get().upgrade().unwrap().downcast().unwrap()),
			Entry::Vacant(entry) => {
				let asset = T::load(entry.key(), params)?;
				let arc: Arc<dyn Any + Send + Sync> = Arc::new(Self { asset, path: entry.key().to_owned() });
				entry.insert(Arc::downgrade(&arc));
				Ok(arc.downcast().unwrap())
			}
		}
	}

}

impl<T> Drop for Cache<T> {

	fn drop(&mut self) {
		ASSET_CACHE.lock().unwrap().remove(&self.path);
	}

}