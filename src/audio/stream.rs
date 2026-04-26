use std::ffi::{c_int, c_void};
use std::ptr::NonNull;
use sdl3_sys::audio::*;
use std::ptr;
use crate::sdl::err::{non_null_or_sdl_panic, sdl_assert};
use super::AudioDevice;
use super::AudioSpec;

/// An audio stream.
pub struct AudioStream(NonNull<SDL_AudioStream>);

impl AudioStream {

	/// Returns a new audio stream with the given format.
	pub fn new(src_spec: Option<AudioSpec>, dst_spec: Option<AudioSpec>) -> Self {
		unsafe { Self(non_null_or_sdl_panic(SDL_CreateAudioStream(src_spec.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default(), dst_spec.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()))) }
	}

	/// Binds the audio stream to an audio device.
	pub fn bind(&mut self, device: &AudioDevice) {
		unsafe { sdl_assert!(SDL_BindAudioStream(device.sdl_audio_device_id(), self.sdl_audio_stream())); }
	}

	/// Unbinds the audio stream from its audio device.
	pub fn unbind(&mut self) {
		unsafe { SDL_UnbindAudioStream(self.sdl_audio_stream()); }
	}

	/// Adds data to the audio stream.
	pub fn put(&mut self, data: &[u8]) {
		unsafe { sdl_assert!(SDL_PutAudioStreamData(self.sdl_audio_stream(), data.as_ptr() as *const c_void, data.len() as c_int)); }
	}

	/// Clears any pending data in the audio stream.
	pub fn clear(&mut self) {
		unsafe { sdl_assert!(SDL_ClearAudioStream(self.sdl_audio_stream())); }
	}

	/// Signals that no more data is incoming, and that any data being buffered
	/// should be made available immediately.
	pub fn flush(&mut self) {
		unsafe { sdl_assert!(SDL_FlushAudioStream(self.sdl_audio_stream())); }
	}

	/// Returns the [`SDL_AudioStream`] pointer underlying an [`AudioStream`].
	pub(crate) fn sdl_audio_stream(&self) -> *mut SDL_AudioStream {
		self.0.as_ptr()
	}

}