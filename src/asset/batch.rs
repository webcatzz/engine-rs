//! Batch loading.

// TODO: Parallelized, async loading + progress reporting

use std::path::Path;
use super::ASSET_CACHE;
use super::{Cached, Cache, Load};

/// Loads multiple assets.
pub fn batch_load<'a, T, P>(iter: impl Iterator<Item = (P, T::Params<'a>)>) -> Vec<T>
where T: Load, P: AsRef<Path>
{
	iter.map(|(path, params)| T::load(path, params).unwrap()).collect()
}

/// Loads multiple assets. Caches results.
pub fn batch_load_cached<'a, T, P>(iter: impl Iterator<Item = (P, T::Params<'a>)>) -> Vec<Cached<T>>
where T: Load + Send + Sync + 'static, P: AsRef<Path>
{
	let cache = &mut *ASSET_CACHE.lock().unwrap();
	iter.map(|(path, params)| Cached::handle(Cache::get_or_load(path.as_ref().to_owned(), params, cache).unwrap())).collect()
}