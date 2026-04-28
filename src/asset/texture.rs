use std::io::{self, Read, Seek};
use std::ptr::{self, NonNull};
use sdl3_sys::render::*;
use sdl3_image_sys::image::*;
use crate::math::{Color, Rect, Transform, Vec2};
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
		let SDL_Texture { w, h, .. } = unsafe { self.sdl_texture().read() };
		Vec2 { x: w as u32, y: h as u32 }
	}

	/// Draws the texture to a frame.
	pub fn draw(&self, TextureDrawOptions { rect, offset, transform, modulate }: TextureDrawOptions, frame: &mut Frame) {
		unsafe {
			let rect   = rect.unwrap_or_else(|| Rect { pos: Vec2::ZERO, size: self.size().as_f32() });
			let rem    = rect.size - offset;
			let origin = transform.transform(-offset);
			let right  = transform.transform(Vec2 { x: rem.x, y: -offset.y });
			let down   = transform.transform(Vec2 { x: -offset.x, y: rem.y });
			sdl_assert!(SDL_SetTextureColorMod(self.sdl_texture(), modulate.r, modulate.g, modulate.b)
				&& SDL_SetTextureAlphaMod(self.sdl_texture(), modulate.a)
				&& SDL_RenderTextureAffine(frame.sdl_renderer(), self.sdl_texture(), &rect.into(), &origin.into(), &right.into(), &down.into()));
		}
	}

	/// Creates a texture from an image.
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

impl Clone for Texture {

	fn clone(&self) -> Self {
		unsafe {
			let renderer = SDL_GetRendererFromTexture(self.sdl_texture());
			sdl_assert!(!renderer.is_null());
			let SDL_Texture { format, w, h, .. } = *self.sdl_texture();
			let texture_ptr = SDL_CreateTexture(renderer, format, SDL_TEXTUREACCESS_TARGET, w, h);
			let texture = Texture::from_sdl_texture(non_null_or_sdl_panic(texture_ptr));
			sdl_assert!(SDL_SetRenderTarget(renderer, texture.sdl_texture())
				&& SDL_RenderTexture(renderer, self.sdl_texture(), ptr::null(), ptr::null())
				&& SDL_SetRenderTarget(renderer, ptr::null_mut()));
			texture
		}
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

/// Options for drawing a texture.
#[derive(Clone)]
pub struct TextureDrawOptions {
	/// The portion of the texture to draw. Uses the full texture if `None`.
	pub rect: Option<Rect<f32>>,
	/// The offset applied when drawing the texture.
	pub offset: Vec2<f32>,
	/// The transform applied when drawing the texture.
	pub transform: Transform,
	/// The color modulation applied when drawing the texture.
	pub modulate: Color<u8>,
}

impl Default for TextureDrawOptions {

	fn default() -> Self {
		Self {
			rect: None,
			offset: Vec2::ZERO,
			transform: Transform::ID,
			modulate: Color::WHITE,
		}
	}

}