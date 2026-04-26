use std::io::{self, Read, Seek};
use std::ptr::{self, NonNull};
use sdl3_sys::iostream::*;
use sdl3_sys::surface::*;
use sdl3_image_sys::image::*;
use crate::math::Vec2;
use crate::sdl::err::non_null_or_sdl_panic;
use super::FromBytes;

pub struct Image(NonNull<SDL_Surface>);

impl Image {

	pub fn size(&self) -> Vec2<u32> {
		unsafe {
			let v = ptr::read(self.sdl_surface());
			Vec2 { x: v.w as u32, y: v.h as u32 }
		}
	}

	/// Wraps an `SDL_Surface` pointer in a [`Image`].
	pub(crate) const fn from_sdl_surface(sdl_surface: NonNull<SDL_Surface>) -> Self {
		Self(sdl_surface)
	}

	/// Returns the `SDL_Surface` pointer underlying a [`Image`].
	pub(crate) const fn sdl_surface(&self) -> *mut SDL_Surface {
		self.0.as_ptr()
	}

}

impl FromBytes for Image {

	type Params<'a> = ();

	fn from_bytes(bytes: &mut (impl Read + Seek), _: Self::Params<'_>) -> io::Result<Self> {
		unsafe {
			let mut buf = Vec::new();
			bytes.read_to_end(&mut buf)?;
			let stream = SDL_IOFromConstMem(buf.as_mut_ptr() as *const _, buf.len());
			Ok(Self::from_sdl_surface(non_null_or_sdl_panic(IMG_Load_IO(stream, true))))
		}
	}

}

impl Drop for Image {

	fn drop(&mut self) {
		unsafe { SDL_DestroySurface(self.sdl_surface()); }
	}

}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}