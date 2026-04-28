//! Rendering.

use std::ffi::{c_char, c_float, c_int};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use sdl3_sys::render::*;
use sdl3_sys::surface::*;
use sdl3_ttf_sys::textengine::*;
use sdl3_ttf_sys::ttf::*;
use crate::asset::{Texture, Font};
use crate::math::{self, Color, Rect, Vec2, Transform, Cast};
use crate::sdl::err::{non_null_or_sdl_panic, sdl_assert};

/// A frame that can be rendered to. Returned by [`Window::frame()`].
///
/// [`Window::frame()`]: super::Window::frame
pub struct Frame<'a> {
	sdl_renderer: NonNull<SDL_Renderer>,
	sdl_text_engine: NonNull<TTF_TextEngine>,
	phantom: PhantomData<&'a ()>,
}

impl<'a> Frame<'a> {

	/// Displays the frame in its window.
	pub fn present(self) {
		unsafe { sdl_assert!(SDL_RenderPresent(self.sdl_renderer())); }
	}

	/// Clears the frame with a solid color.
	pub fn clear(&mut self, color: Color<u8>) {
		unsafe { sdl_assert!(
			SDL_SetRenderDrawColor(self.sdl_renderer(), color.r, color.g, color.b, color.a)
			&& SDL_RenderClear(self.sdl_renderer())
		); }
	}

	/// Fills a pixel.
	pub fn draw_point(&mut self, pos: Vec2<f32>, color: Color<u8>) {
		unsafe { sdl_assert!(
			SDL_SetRenderDrawColor(self.sdl_renderer(), color.r, color.g, color.b, color.a)
			&& SDL_RenderPoint(self.sdl_renderer(), pos.x as c_float, pos.y as c_float)
		); }
	}

	/// Draws a line from one point to another.
	pub fn draw_line(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color<u8>) {
		unsafe { sdl_assert!(
			SDL_SetRenderDrawColor(self.sdl_renderer(), color.r, color.g, color.b, color.a)
			&& SDL_RenderLine(self.sdl_renderer(), a.x as c_float, a.y as c_float, b.x as c_float, b.y as c_float)
		); }
	}

	/// Outlines a circle.
	pub fn draw_circle(&mut self, center: Vec2<f32>, radius: f32, color: Color<u8>) {
		math::bresenham_circle(center.as_i32(), radius as u32, |pos| {
			self.draw_point(pos.as_f32(), color);
		});
	}

	/// Outlines a rectangle.
	pub fn draw_rect(&mut self, rect: Rect<f32>, color: Color<u8>) {
		unsafe { sdl_assert!(
			SDL_SetRenderDrawColor(self.sdl_renderer(), color.r, color.g, color.b, color.a)
			&& SDL_RenderRect(self.sdl_renderer(), &rect.into())
		); }
	}

	/// Fills a rectangle.
	pub fn fill_rect(&mut self, rect: Rect<f32>, color: Color<u8>) {
		unsafe { sdl_assert!(
			SDL_SetRenderDrawColor(self.sdl_renderer(), color.r, color.g, color.b, color.a)
			&& SDL_RenderFillRect(self.sdl_renderer(), &rect.into())
		); }
	}

	/// Draws text.
	pub fn draw_text(&mut self, text: &str, font: &Font, color: Color<u8>, wrap_width: u32, transform: Transform) {
		if text.is_empty() { return; }
		unsafe {
			let surface = TTF_RenderText_Blended_Wrapped(font.sdl_font(), text.as_ptr() as *const c_char, text.len(), color.into(), wrap_width as c_int);
			sdl_assert!(!surface.is_null());
			let texture = Texture::from_sdl_texture(non_null_or_sdl_panic(SDL_CreateTextureFromSurface(self.sdl_renderer(), surface)));
			SDL_DestroySurface(surface);
			texture.draw(transform, self);
		}
	}

	/// Returns the output size of the frame, in pixels.
	///
	/// Output size is not affected by the current transform.
	pub fn size(&self) -> Vec2<u32> {
		unsafe {
			let (mut w, mut h) = (MaybeUninit::uninit(), MaybeUninit::uninit());
			sdl_assert!(SDL_GetCurrentRenderOutputSize(self.sdl_renderer(), w.as_mut_ptr(), h.as_mut_ptr()));
			Vec2 { x: w.assume_init() as u32, y: h.assume_init() as u32 }
		}
	}

	/// Wraps `SDL_Renderer` and `TTF_TextEngine` pointers in a [`Frame`].
	pub(crate) fn from_sdl_renderer_and_text_engine(sdl_renderer: NonNull<SDL_Renderer>, sdl_text_engine: NonNull<TTF_TextEngine>) -> Self {
		Self { sdl_renderer, sdl_text_engine, phantom: PhantomData }
	}

	/// Returns the `SDL_Renderer` pointer underlying a [`Frame`].
	pub(crate) fn sdl_renderer(&self) -> *mut SDL_Renderer {
		self.sdl_renderer.as_ptr()
	}

	/// Returns the `TTF_TextEngine` pointer underlying a [`Frame`].
	pub(crate) fn sdl_text_engine(&self) -> *mut TTF_TextEngine {
		self.sdl_text_engine.as_ptr()
	}

}

/// Types that can be drawn to a [`Frame`].
pub trait Draw {
	/// Draws to a [`Frame`].
	fn draw(&self, frame: &mut Frame);
}