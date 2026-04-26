use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use sdl3_sys::audio::*;
use std::ptr;
use crate::sdl::err::{sdl_assert, sdl_panic};
use super::AudioSpec;

/// A logical audio device.
pub struct AudioDevice<'a>(AudioDeviceId, PhantomData<&'a ()>);

impl<'a> AudioDevice<'a> {

	/// See [`Platform::open_audio_device()`] for documentation.
	///
	/// [`Platform::open_audio_device()`]: crate::platform::Platform::open_audio_device
	pub(crate) fn new(id: AudioDeviceId, spec: Option<AudioSpec>) -> Self {
		match unsafe { SDL_OpenAudioDevice(id.into(), spec.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()).try_into() } {
			Ok(id) => Self(id, PhantomData),
			Err(..) => sdl_panic!(),
		}
	}

	pub fn spec(&self) -> AudioSpec {
		unsafe {
			let mut spec = MaybeUninit::uninit();
			sdl_assert!(SDL_GetAudioDeviceFormat(self.sdl_audio_device_id(), spec.as_mut_ptr(), ptr::null_mut()));
			spec.assume_init().into()
		}
	}

	/// Sets if an audio device is playing or paused.
	///
	/// Any [`AudioStream`]s bound to a paused device will not progress.
	pub fn set_paused(&mut self, paused: bool) {
		match paused {
			true => unsafe { sdl_assert!(SDL_PauseAudioDevice(self.sdl_audio_device_id())); }
			false => unsafe { sdl_assert!(SDL_ResumeAudioDevice(self.sdl_audio_device_id())); }
		}
	}

	/// Returns true if the audio device is paused.
	///
	/// New devices are paused by default.
	pub fn is_paused(&self) -> bool {
		unsafe { SDL_AudioDevicePaused(self.sdl_audio_device_id()) }
	}

	/// Returns the [`SDL_AudioDeviceID`] underlying an [`AudioDevice`].
	pub(crate) fn sdl_audio_device_id(&self) -> SDL_AudioDeviceID {
		self.0.into()
	}

}

impl<'a> Drop for AudioDevice<'a> {

	fn drop(&mut self) {
		unsafe { SDL_CloseAudioDevice(self.sdl_audio_device_id()); }
	}

}

/// A unique ID for an audio device.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioDeviceId(NonZeroU32);

impl AudioDeviceId {

	/// The ID for the default audio playback device.
	pub const DEFAULT_PLAYBACK: Self = Self(NonZeroU32::new(SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK.0).unwrap());
	/// The ID for the default audio recording device.
	pub const DEFAULT_RECORDING: Self = Self(NonZeroU32::new(SDL_AUDIO_DEVICE_DEFAULT_RECORDING.0).unwrap());

	/// Returns a new audio device ID.
	///
	/// Returns `None` if `id` is `0`. The zero ID is used within SDL to signify
	/// an invalid or null device.
	pub fn new(id: u32) -> Option<Self> {
		NonZeroU32::new(id).map(Self)
	}

}

impl TryFrom<SDL_AudioDeviceID> for AudioDeviceId {

	type Error = &'static str;

	fn try_from(id: SDL_AudioDeviceID) -> Result<Self, Self::Error> {
		Self::new(id.0).ok_or("`0` is not a valid audio device ID")
	}

}

impl Into<SDL_AudioDeviceID> for AudioDeviceId {

	fn into(self) -> SDL_AudioDeviceID {
		SDL_AudioDeviceID(self.0.get())
	}

}