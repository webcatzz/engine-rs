use std::io::{self, Read, Seek};
use std::ptr::NonNull;
use sdl3_ttf_sys::ttf::*;
use crate::sdl::err::non_null_or_sdl_panic;
use crate::sdl::io::SdlIoStream;
use super::FromBytes;

/// A font.
pub struct Font(NonNull<TTF_Font>);

impl Font {

	/// Wraps a `TTF_Font` pointer in a [`Font`].
	pub(crate) fn from_sdl_font(sdl_font: NonNull<TTF_Font>) -> Self {
		Self(sdl_font)
	}

	/// Returns the `TTF_Font` pointer underlying a [`Font`].
	pub(crate) fn sdl_font(&self) -> *mut TTF_Font {
		self.0.as_ptr()
	}

}

impl FromBytes for Font {

	type Params<'a> = ();

	fn from_bytes(bytes: &mut (impl Read + Seek), _: Self::Params<'_>) -> io::Result<Self> {
		let stream = SdlIoStream::new_read_seek(bytes);
		let ptr = unsafe { TTF_OpenFontIO(stream.sdl_stream(), false, 16.0) };
		Ok(Self::from_sdl_font(non_null_or_sdl_panic(ptr)))
	}

}

impl Drop for Font {

	fn drop(&mut self) {
		unsafe { TTF_CloseFont(self.sdl_font()); }
	}

}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}