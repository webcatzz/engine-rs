use std::ffi::{c_int, c_void};
use std::ptr;
use std::io::{self, Read, Seek, Write};
use sdl3_sys::audio::*;
use crate::asset::{Cached, Audio};
use crate::sdl::err::sdl_assert;
use super::{AudioDevice, AudioStream};

/// Plays audio from source data.
pub struct AudioPlayer {
	/// Output stream.
	stream: AudioStream,
	/// Playback data.
	playback_data: Box<AudioPlaybackData>,
}

impl AudioPlayer {

	/// Returns a new audio player.
	pub fn new(audio: Cached<Audio>, device: &AudioDevice) -> Self {
		unsafe {
			let mut stream = AudioStream::new(Some(audio.spec().clone()), None);
			stream.bind(device);
			let mut playback_data = Box::new(AudioPlaybackData::new(audio));
			SDL_SetAudioStreamGetCallback(stream.sdl_audio_stream(), Some(Self::get_callback), ptr::from_mut(&mut *playback_data) as *mut c_void);
			Self { stream, playback_data }
		}
	}

	/// Sets the currently playing audio.
	pub fn set_audio(&mut self, audio: Cached<Audio>) {
		self.playback_data.audio = audio;
	}

	/// Returns true if the audio player is paused.
	pub fn paused(&self) -> bool {
		self.playback_data.paused
	}

	/// Sets whether the audio player is playing audio.
	pub fn set_paused(&mut self, paused: bool) {
		self.playback_data.paused = paused;
		self.stream.clear();
	}

	/// Returns true if the audio player loops its audio.
	pub fn repeat(&mut self) -> bool {
		self.playback_data.repeat
	}

	/// Sets whether the audio player loops its audio.
	pub fn set_repeat(&mut self, repeat: bool) {
		self.playback_data.repeat = repeat;
	}

	/// Returns the [`SDL_AudioStream`] pointer underlying an [`AudioDevice`].
	pub(crate) fn sdl_audio_stream(&self) -> *mut SDL_AudioStream {
		self.stream.sdl_audio_stream()
	}

	/// [`SDL_AudioStreamCallback`] interface.
	extern "C" fn get_callback(userdata: *mut c_void, stream: *mut SDL_AudioStream, additional_amount: c_int, _total_amount: c_int) {
		unsafe {
			let playback_data = &mut *(userdata as *mut AudioPlaybackData);
			if playback_data.paused { return; }
			if playback_data.pos < playback_data.audio.data().len() - 1 {
				let mut buf = vec![0; additional_amount as usize];
				let len = playback_data.read(&mut buf).unwrap();
				sdl_assert!(SDL_PutAudioStreamData(stream, buf.as_ptr() as *const c_void, len as c_int));
			} else if playback_data.repeat {
				playback_data.pos = 0;
			}
		}
	}

}

/// Playback data for an [`AudioPlayer`].
pub struct AudioPlaybackData {
	/// Source audio.
	audio: Cached<Audio>,
	/// Current position in the source audio.
	pos: usize,
	/// Whether audio should loop.
	repeat: bool,
	/// Whether audio should be played back.
	paused: bool,
}

impl AudioPlaybackData {

	pub fn new(audio: Cached<Audio>) -> Self {
		Self {
			audio,
			pos: 0,
			repeat: false,
			paused: true,
		}
	}

}

impl Read for AudioPlaybackData {

	fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
		let res = buf.write(&self.audio.data()[self.pos..]);
		if let Ok(len) = res { self.pos += len; }
		res
	}

}

impl Seek for AudioPlaybackData {

	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		self.pos = match pos {
			io::SeekFrom::Start(offset) =>
				offset as usize,
			io::SeekFrom::Current(offset) =>
				(self.pos as i64 + offset) as usize,
			io::SeekFrom::End(offset) =>
				(self.audio.data().len() as i64 + offset) as usize,
		};
		Ok(self.pos as u64)
	}

}