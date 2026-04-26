use std::ffi::{c_int, c_uint};
use sdl3_sys::audio::*;

/// Format description for audio data.
#[derive(Clone)]
pub struct AudioSpec {
	/// Audio data format.
	pub format: AudioFormat,
	/// Number of channels (1 is mono, 2 is stereo, etc.).
	pub channel_count: u32,
	/// Sample frames per second.
	pub freq: u32,
}

impl From<SDL_AudioSpec> for AudioSpec {

	fn from(spec: SDL_AudioSpec) -> Self {
		Self {
			format: spec.format.into(),
			channel_count: spec.channels as u32,
			freq: spec.freq as u32,
		}
	}

}

impl Into<SDL_AudioSpec> for AudioSpec {

	fn into(self) -> SDL_AudioSpec {
		SDL_AudioSpec {
			format: self.format.into(),
			channels: self.channel_count as c_int,
			freq: self.freq as c_int,
		}
	}

}

/// Audio data format.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
	/// Unsigned 8-bit samples.
	U8      = SDL_AUDIO_U8.0 as u32,
	/// Signed 8-bit samples.
	S8      = SDL_AUDIO_S8.0 as u32,
	/// Signed 16-bit samples.
	S16     = SDL_AUDIO_S16.0 as u32,
	/// Signed 32-bit samples.
	S32     = SDL_AUDIO_S32.0 as u32,
	/// Signed 32-bit floating point samples.
	F32     = SDL_AUDIO_F32.0 as u32,
	/// Unspecified format.
	Unknown = SDL_AUDIO_UNKNOWN.0 as u32,
}

impl From<SDL_AudioFormat> for AudioFormat {

	fn from(value: SDL_AudioFormat) -> Self {
		match value {
			SDL_AUDIO_U8      => Self::U8,
			SDL_AUDIO_S8      => Self::S8,
			SDL_AUDIO_S16     => Self::S16,
			SDL_AUDIO_S32     => Self::S32,
			SDL_AUDIO_F32     => Self::F32,
			SDL_AUDIO_UNKNOWN => Self::Unknown,
			_ => panic!("Unrecognized `SDL_AudioFormat` variant"),
		}
	}

}

impl Into<SDL_AudioFormat> for AudioFormat {

	fn into(self) -> SDL_AudioFormat {
		SDL_AudioFormat(self as c_uint)
	}

}