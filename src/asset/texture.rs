use std::ffi::c_float;
use std::io::{self, Read, Seek};
use std::ptr::NonNull;
use sdl3_sys::render::*;
use sdl3_image_sys::image::*;
use crate::math::{Cast, Rect, Transform, Vec2};
use crate::sdl::err::{non_null_or_sdl_panic, sdl_assert};
use crate::sdl::io::SdlIoStream;
use crate::window::Frame;
use super::{FromBytes, Image};

/// A texture used for rendering.
pub struct Texture(NonNull<SDL_Texture>);

impl Texture {

	/// Returns the width of the texture, in pixels.
	pub const fn width(&self) -> u32 {
		unsafe { self.sdl_texture().read().w as u32 }
	}

	/// Returns the height of the texture, in pixels.
	pub const fn height(&self) -> u32 {
		unsafe { self.sdl_texture().read().h as u32 }
	}

	/// Returns the size of the texture, in pixels.
	pub const fn size(&self) -> Vec2<u32> {
		unsafe {
			let val = self.sdl_texture().read();
			Vec2 { x: val.w as u32, y: val.h as u32 }
		}
	}

	const fn full_rect(&self) -> Rect<u32> {
		Rect { pos: Vec2::ZERO, size: self.size() }
	}

	/// Draws the texture to a frame.
	pub fn draw(&self, transform: Transform, frame: &mut Frame) {
		self.draw_offset(Vec2::<f32>::ZERO, transform, frame);
	}

	/// Draws the texture to a frame with some offset.
	pub fn draw_offset<T: Cast>(&self, offset: Vec2<T>, transform: Transform, frame: &mut Frame) {
		self.draw_rect_offset(self.full_rect(), offset, transform, frame);
	}

	/// Shorthand for `texture.draw_offset(texture.size() / 2, transform, frame)`.
	pub fn draw_centered(&self, transform: Transform, frame: &mut Frame) {
		self.draw_offset(self.size() / 2, transform, frame);
	}

	/// Draws part of the texture to a frame.
	pub fn draw_rect<T: Cast, U: Cast + Copy>(&self, rect: Rect<T, U>, transform: Transform, frame: &mut Frame) {
		self.draw_rect_offset(rect, Vec2::<f32>::ZERO, transform, frame);
	}

	/// Draws part of a texture to a frame with some offset.
	pub fn draw_rect_offset<T, U, V>(&self, rect: Rect<T, U>, offset: Vec2<V>, transform: Transform, frame: &mut Frame)
	where T: Cast, U: Cast + Copy, V: Cast
	{
		unsafe {
			let offset = offset.as_f32();
			let rem    = rect.size.as_f32() - offset;
			let origin = transform.transform(-offset);
			let right  = transform.transform(Vec2 { x: rem.x, y: -offset.y });
			let down   = transform.transform(Vec2 { x: -offset.x, y: rem.y });
			sdl_assert!(SDL_RenderTextureAffine(frame.sdl_renderer(), self.sdl_texture(), &rect.into(), &origin.into(), &right.into(), &down.into()));
		}
	}

	pub fn fill_rect(&self, rect: Rect<f32>, scale: f32, frame: &mut Frame) {
		unsafe {
			sdl_assert!(SDL_RenderTextureTiled(frame.sdl_renderer(), self.sdl_texture(), &self.full_rect().into(), scale as c_float, &rect.into()));
		}
	}

	pub fn from_image(image: &Image, frame: &Frame) -> Self {
		unsafe { Self::from_sdl_texture(non_null_or_sdl_panic(SDL_CreateTextureFromSurface(frame.sdl_renderer(), image.sdl_surface()))) }
	}

	/// Wraps an `SDL_Texture` pointer in a [`Texture`].
	pub(crate) const fn from_sdl_texture(sdl_texture: NonNull<SDL_Texture>) -> Self {
		Self(sdl_texture)
	}

	/// Returns the `SDL_Texture` pointer underlying a [`Texture`].
	pub(crate) const fn sdl_texture(&self) -> *mut SDL_Texture {
		self.0.as_ptr()
	}

}

impl FromBytes for Texture {

	type Params<'a> = &'a Frame<'a>;

	fn from_bytes(bytes: &mut (impl Read + Seek), params: Self::Params<'_>) -> io::Result<Self> {
		let mut buf = Vec::new();
		bytes.read_to_end(&mut buf).unwrap();
		let stream = unsafe { sdl3_sys::iostream::SDL_IOFromConstMem(buf.as_mut_ptr() as *const _, buf.len()) };
		let ptr = unsafe { IMG_LoadTexture_IO(params.sdl_renderer(), stream, true) };
		Ok(Texture::from_sdl_texture(non_null_or_sdl_panic(ptr)))
		// let stream = SdlIoStream::new_read_seek(bytes);
		// let ptr = unsafe { IMG_LoadTexture_IO(params.sdl_renderer(), stream.sdl_stream(), false) };
		// Texture::from_sdl_texture(non_null_or_sdl_panic(ptr))
	}

}

impl Drop for Texture {

	fn drop(&mut self) {
		unsafe { SDL_DestroyTexture(self.sdl_texture()); }
	}

}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}