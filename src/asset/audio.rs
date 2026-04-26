use std::io::{self, Read, Seek};
use std::mem::MaybeUninit;
use std::slice;
use sdl3_sys::audio::*;
use sdl3_sys::stdinc::SDL_free;
use crate::audio::AudioSpec;
use crate::sdl::err::sdl_assert;
use super::FromBytes;

/// Audio data.
pub struct Audio {
	/// The raw audio data.
	data: Vec<u8>,
	/// The format of the audio data.
	spec: AudioSpec,
}

impl Audio {

	/// Returns the [`AudioSpec`] associated with the audio.
	pub fn spec(&self) -> &AudioSpec {
		&self.spec
	}

	/// Returns a reference to the raw data of the audio.
	pub fn data(&self) -> &[u8] {
		self.data.as_slice()
	}

}

impl FromBytes for Audio {

	type Params<'a> = ();

	fn from_bytes(bytes: &mut (impl Read + Seek), _: Self::Params<'_>) -> io::Result<Self> {
		let mut buf = Vec::new();
		bytes.read_to_end(&mut buf).unwrap();
		let mut audio_spec = MaybeUninit::uninit();
		let mut audio_buf = MaybeUninit::uninit();
		let mut audio_len = MaybeUninit::uninit();
		unsafe {
			let stream = sdl3_sys::iostream::SDL_IOFromConstMem(buf.as_mut_ptr() as *const _, buf.len());
			sdl_assert!(SDL_LoadWAV_IO(stream, true, audio_spec.as_mut_ptr(), audio_buf.as_mut_ptr(), audio_len.as_mut_ptr()));
			let slice = slice::from_raw_parts_mut(audio_buf.assume_init(), audio_len.assume_init() as usize);
			let audio = Audio {
				spec: audio_spec.assume_init().into(),
				data: slice.to_vec(),
			};
			SDL_free(audio_buf.assume_init() as *mut _);
			Ok(audio)
		}

		// let stream = SdlIoStream::new_read_seek(bytes);
		// let mut audio_spec = MaybeUninit::uninit();
		// let mut audio_buf = MaybeUninit::uninit();
		// let mut audio_len = MaybeUninit::uninit();
		// unsafe {
		// 	sdl_assert!(SDL_LoadWAV_IO(stream.sdl_stream(), false, audio_spec.as_mut_ptr(), audio_buf.as_mut_ptr(), audio_len.as_mut_ptr()));
		// 	let slice = slice::from_raw_parts_mut(audio_buf.assume_init(), audio_len.assume_init() as usize);
		// 	let audio = Audio {
		// 		spec: audio_spec.assume_init().into(),
		// 		data: slice.to_vec(),
		// 	};
		// 	SDL_free(audio_buf.assume_init() as *mut _);
		// 	audio
		// }
	}

}