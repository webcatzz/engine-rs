//! Engine core.

use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use sdl3_sys::init::*;
use sdl3_sys::events::*;
use sdl3_sys::timer::SDL_GetTicksNS;
use sdl3_ttf_sys::ttf::{TTF_Init, TTF_Quit};
use crate::audio::{AudioDevice, AudioDeviceId, AudioSpec};
use crate::dialog;
use crate::event::Event;
use crate::math::Vec2;
use crate::run::Run;
use crate::sdl::err::sdl_assert;
use crate::window::Window;

/// Exposes system functionality.
pub struct Platform(PhantomData<*const ()>);

impl Platform {

	/// Initializes the platform.
	///
	/// Only one [`Platform`] should be in scope at any one time.
	///
	/// # Thread safety
	///
	/// This function should only be called on the main thread.
	pub fn init() -> Self {
		unsafe {
			// Sets SDL app metadata based on environment
			for (sdl, env) in [
				(SDL_PROP_APP_METADATA_NAME_STRING,    option_env!("CARGO_PKG_NAME")),
				(SDL_PROP_APP_METADATA_VERSION_STRING, option_env!("CARGO_PKG_VERSION")),
				(SDL_PROP_APP_METADATA_CREATOR_STRING, option_env!("CARGO_PKG_AUTHORS")),
				(SDL_PROP_APP_METADATA_URL_STRING,     option_env!("CARGO_PKG_HOMEPAGE")),
			] {
				if let Some(var) = env {
					SDL_SetAppMetadataProperty(sdl, CString::new(var).unwrap().as_ptr());
				}
			}
			// Initializes SDL
			sdl_assert!(SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO) && TTF_Init());
			Self(PhantomData)
		}
	}

	/// Returns an iterator over incoming system events.
	pub fn events(&self) -> impl Iterator<Item = Event> {
		Events(PhantomData)
	}

	/// Opens a new window.
	pub fn open_window<'a>(&'a self, title: &str, size: Vec2<u32>) -> Window<'a> {
		Window::new(title, size)
	}

	/// Opens the audio device with the given ID.
	///
	/// The same audio device may be opened multiple times. Each open will return
	/// a new logical audio device that is independent of the others.
	///
	/// If you don't need to open a specific device, use the default IDs
	/// ([`DEFAULT_PLAYBACK`] or [`DEFAULT_RECORDING`]) to open a reasonable
	/// default. If a default audio device ever changes (e.g. one is plugged in or
	/// unplugged), the new device will be seamlessly swapped in.
	///
	/// You may request a specific format for the audio device with `spec`. The
	/// device may not honor the request. If `spec` is `None`, uses reasonable
	/// defaults.
	///
	/// [`DEFAULT_PLAYBACK`]: AudioDeviceId::DEFAULT_PLAYBACK
	/// [`DEFAULT_RECORDING`]: AudioDeviceId::DEFAULT_RECORDING
	pub fn open_audio_device<'a>(&'a self, id: AudioDeviceId, spec: Option<AudioSpec>) -> AudioDevice<'a> {
		AudioDevice::new(id, spec)
	}

	/// Opens an "open file" dialog and blocks until the user accepts or cancels
	/// the dialog, returning the list of files selected by the user.
	///
	/// # Parameters
	///
	/// - `filters`: A list of filters restricting which files the user should
	///   allowed to select. Not all platforms support this option, and platforms
	///   that do support it may allow the user to ignore filters.
	/// - `allow_multiple`: Whether the user should be allowed to select multiple
	///   entries. Not all platforms support this option.
	/// - `default`: The default folder or file to start the dialog at. Not all
	///   platforms support this option.
	/// - `window`: The window that the dialog should be modal for. Not all
	///   platforms support this option.
	pub fn prompt_open_file<'a>(
		&self,
		filters: impl IntoIterator<Item = &'a dialog::DialogFileFilter<'a>>,
		allow_multiple: bool,
		default: Option<impl AsRef<Path>>,
		window: Option<&Window>,
	) -> Vec<PathBuf> {
		dialog::open_file(filters, allow_multiple, default, window)
	}

	/// Opens an "open folder" dialog and blocks until the user accepts or cancels
	/// the dialog, returning the list of folders selected by the user.
	///
	/// # Parameters
	///
	/// - `allow_multiple`: Whether the user should be allowed to select multiple
	///   entries. Not all platforms support this option.
	/// - `default`: The default folder or file to start the dialog at. Not all
	///   platforms support this option.
	/// - `window`: The window that the dialog should be modal for. Not all
	///   platforms support this option.
	pub fn prompt_open_folder(
		&self,
		allow_multiple: bool,
		default: Option<impl AsRef<Path>>,
		window: Option<&Window>,
	) -> Vec<PathBuf> {
		dialog::open_folder(allow_multiple, default, window)
	}

	/// Opens an "save file" dialog and blocks until the user accepts or cancels
	/// the dialog, returning the path selected by the user, if any.
	///
	/// # Parameters
	///
	/// - `filters`: A list of filters restricting which files the user should
	///   allowed to select. Not all platforms support this option, and platforms
	///   that do support it may allow the user to ignore filters.
	/// - `default`: The default folder or file to start the dialog at. Not all
	///   platforms support this option.
	/// - `window`: The window that the dialog should be modal for. Not all
	///   platforms support this option.
	pub fn prompt_save_file<'a>(
		&self,
		filters: impl IntoIterator<Item = &'a dialog::DialogFileFilter<'a>>,
		default: Option<impl AsRef<Path>>,
		window: Option<&Window>,
	) -> Option<PathBuf> {
		dialog::save_file(filters, default, window)
	}

	/// Runs the main loop, blocking until the program is requested to quit.
	///
	/// The main loop involves three calls:
	/// - [`Run::listen()`] is called whenever an incoming event is detected,
	/// - [`Run::update()`] is called roughly every [`Run::UPDATE_FREQ`],
	/// - [`Run::render()`] is called roughly every [`Run::RENDER_FREQ`].
	///
	/// The loop will skip render calls if necessary.
	///
	/// For design motivation, see [Game Loop] from Game Programming Patterns.
	///
	/// [Game Loop]: https://gameprogrammingpatterns.com/game-loop.html
	pub fn main_loop<'a, T: Run<'a>>(&'a self, mut run: T) {
		let mut last_update_time = Instant::now();
		let mut last_render_time = Instant::now();
		let mut update_lag = Duration::ZERO;
		loop {
			// Handles incoming events
			for event in self.events() {
				run.listen(&event);
				// Quits if a quit event was received
				if matches!(event, Event::Quit) {
					println!("Requested to quit, quitting...");
					return;
				}
			}
			// Updates
			update_lag += last_update_time.elapsed();
			last_update_time = Instant::now();
			while update_lag >= T::UPDATE_FREQ {
				run.update();
				update_lag -= T::UPDATE_FREQ;
			}
			// Renders
			if last_render_time.elapsed() > T::RENDER_FREQ {
				last_render_time = Instant::now();
				run.render(update_lag.div_duration_f32(T::UPDATE_FREQ));
			}
		}
	}

	/// Requests a quit by pushing [`Event::Quit`] into the event queue.
	pub fn request_quit(&self) {
		unsafe {
			sdl_assert!(SDL_PushEvent(&mut SDL_Event { quit: SDL_QuitEvent {
				r#type: SDL_EVENT_QUIT,
				timestamp: SDL_GetTicksNS(),
				reserved: 0,
			} }));
		}
	}

}

impl Drop for Platform {

	fn drop(&mut self) {
		unsafe {
			TTF_Quit();
			SDL_Quit();
		}
	}

}

/// An iterator over incoming system events.
struct Events(PhantomData<*const ()>);

impl Iterator for Events {

	type Item = Event;

	fn next(&mut self) -> Option<Event> {
		unsafe {
			let mut event = MaybeUninit::uninit();
			match SDL_PollEvent(event.as_mut_ptr()) {
				true => event.assume_init().try_into().ok(),
				false => None,
			}
		}
	}

}