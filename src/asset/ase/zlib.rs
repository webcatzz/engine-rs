//! *zlib* decompression.
//!
//! Aseprite files use *zlib* to compress cel data. This module provides a
//! simple interface to decompress that data.
//!
//! Uses the [`zlib-rs`] crate.
//!
//! [`zlib-rs`]: https://crates.io/crates/zlib-rs

use std::error;
use std::fmt;

/// Decompresses compressed data.
pub fn decompress(input: &[u8]) -> Result<Vec<u8>, ZLibError> {
	// TODO: didn't put much thought into this -- just keeps trying to decompress
	// until `buf` has enough size to decompress fully
	let mut buf = vec![0; input.len()];
	loop {
		let (output, rc) = zlib_rs::decompress_slice(buf.as_mut_slice(), input, zlib_rs::InflateConfig::default());
		match rc {
			zlib_rs::ReturnCode::BufError => buf.resize(buf.len() + input.len(), 0),
			zlib_rs::ReturnCode::Ok => {
				let len = output.len();
				buf.truncate(len);
				return Ok(buf);
			}
			_ => return Err(ZLibError(rc)),
		}
	};
}

/// An error returned by *zlib*.
pub struct ZLibError(zlib_rs::ReturnCode);

impl fmt::Debug for ZLibError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "zlib error: {:?}", self.0)
	}

}

impl fmt::Display for ZLibError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&self, f)
	}

}

impl error::Error for ZLibError {}