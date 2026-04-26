//! Windows.

use std::ffi::{c_int, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use sdl3_sys::video::*;
use sdl3_sys::render::*;
use sdl3_sys::surface::*;
use sdl3_ttf_sys::textengine::*;
use sdl3_ttf_sys::ttf::*;
use crate::math::Vec2;
use crate::sdl::err::{non_null_or_sdl_panic, sdl_assert};
use super::Frame;

/// An open window.
pub struct Window<'a> {
	sdl_window: NonNull<SDL_Window>,
	sdl_renderer: NonNull<SDL_Renderer>,
	sdl_text_engine: NonNull<TTF_TextEngine>,
	phantom: PhantomData<&'a ()>,
}

impl<'a> Window<'a> {

	/// See [`Platform::open_window()`] for documentation.
	///
	/// This function should only be called once the [`Platform`] has been
	/// initialized.
	///
	/// [`Platform::open_window()`]: super::Platform::open_window
	pub(crate) fn new(title: &str, size: Vec2<u32>) -> Self {
		unsafe {
			let mut window = MaybeUninit::uninit();
			let mut renderer = MaybeUninit::uninit();
			sdl_assert!(SDL_CreateWindowAndRenderer(CString::new(title).unwrap().as_ptr(), size.x as c_int, size.y as c_int, SDL_WindowFlags(0), window.as_mut_ptr(), renderer.as_mut_ptr())
				&& SDL_SetDefaultTextureScaleMode(renderer.assume_init(), SDL_SCALEMODE_PIXELART)
				&& SDL_SetRenderVSync(renderer.assume_init(), 1)
				/* && SDL_StartTextInput(window.assume_init()) */);
			Self {
				sdl_window: non_null_or_sdl_panic(window.assume_init()),
				sdl_renderer: non_null_or_sdl_panic(renderer.assume_init()),
				sdl_text_engine: non_null_or_sdl_panic(TTF_CreateRendererTextEngine(renderer.assume_init())),
				phantom: PhantomData,
			}
		}
	}

	/// Returns the next frame of a window for rendering.
	pub fn frame<'b>(&'b mut self) -> Frame<'b> {
		Frame::from_sdl_renderer_and_text_engine(self.sdl_renderer, self.sdl_text_engine)
	}

	/// Returns true if the window is visible.
	pub fn visible(&self) -> bool {
		unsafe { SDL_GetWindowFlags(self.sdl_window()) & SDL_WINDOW_HIDDEN == 0 }
	}

	/// Makes a window visible.
	pub fn show(&mut self) {
		unsafe { sdl_assert!(SDL_ShowWindow(self.sdl_window())); }
	}

	/// Makes a window invisible.
	pub fn hide(&mut self) {
		unsafe { sdl_assert!(SDL_HideWindow(self.sdl_window())); }
	}

	/// Sets the title of a window.
	pub fn set_title(&mut self, title: &str) {
		unsafe { sdl_assert!(SDL_SetWindowTitle(self.sdl_window(), CString::new(title).unwrap().as_ptr())); }
	}

	/// Returns the size of a window.
	pub fn size(&self) -> Vec2<u32> {
		unsafe {
			let mut w = MaybeUninit::uninit();
			let mut h = MaybeUninit::uninit();
			sdl_assert!(SDL_GetWindowSizeInPixels(self.sdl_window(), w.as_mut_ptr(), h.as_mut_ptr()));
			Vec2 { x: w.assume_init() as u32, y: h.assume_init() as u32 }
		}
	}

	/// Sets the size of a window.
	pub fn set_size(&mut self, size: Vec2<u32>) {
		unsafe { sdl_assert!(SDL_SetWindowSize(self.sdl_window(), size.x as c_int, size.y as c_int)); }
	}

	/// Sets the minimum size of a window.
	pub fn set_min_size(&mut self, min_size: Vec2<u32>) {
		unsafe { sdl_assert!(SDL_SetWindowMinimumSize(self.sdl_window(), min_size.x as c_int, min_size.y as c_int)); }
	}

	/// Sets the maximum size of a window.
	pub fn set_max_size(&mut self, max_size: Vec2<u32>) {
		unsafe { sdl_assert!(SDL_SetWindowMaximumSize(self.sdl_window(), max_size.x as c_int, max_size.y as c_int)); }
	}

	/// Sets whether a window can be resized by the user.
	pub fn set_resizable(&mut self, resizable: bool) {
		unsafe { sdl_assert!(SDL_SetWindowResizable(self.sdl_window(), resizable)); }
	}

	/// Centers a window on the screen.
	pub fn center(&mut self) {
		unsafe { sdl_assert!(SDL_SetWindowPosition(self.sdl_window(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED)); }
	}

	/// Returns the `SDL_Window` pointer underlying a [`Window`].
	pub(crate) fn sdl_window(&self) -> *mut SDL_Window {
		self.sdl_window.as_ptr()
	}

	/// Returns the `SDL_Renderer` pointer underlying a [`Window`].
	pub(crate) fn sdl_renderer(&self) -> *mut SDL_Renderer {
		self.sdl_renderer.as_ptr()
	}

	/// Returns the `TTF_TextEngine` pointer underlying a [`Window`].
	pub(crate) fn sdl_text_engine(&self) -> *mut TTF_TextEngine {
		self.sdl_text_engine.as_ptr()
	}

}

impl<'a> Drop for Window<'a> {

	fn drop(&mut self) {
		unsafe {
			// SDL_StopTextInput(self.sdl_window());
			TTF_DestroyRendererTextEngine(self.sdl_text_engine());
			SDL_DestroyRenderer(self.sdl_renderer());
			SDL_DestroyWindow(self.sdl_window());
		}
	}

}