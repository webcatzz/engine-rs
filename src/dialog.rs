//! System file dialogs.
//!
//! SDL's dialog functions are asynchronous and callback-based, while these
//! wrapper functions use [`tokio`] to block until they can return a value.
//!
//! [`tokio`]: https://docs.rs/tokio/

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use sdl3_sys::dialog::*;
use tokio::sync::oneshot;
use crate::window::Window;
use crate::sdl::err::sdl_panic;

/// A filter for file dialogs.
pub struct DialogFileFilter<'a> {
	/// A user-readable label for the filter.
	pub name: &'a str,
	/// A semicolon-separated list of file extensions.
	/// Extensions may only contain alphanumeric characters, hyphens, underscores and periods.
	/// Alternatively, `"*"` allows all files.
	pub pattern: &'a str,
}

macro_rules! convert_filter {
	($filter:ident) => {
		SDL_DialogFileFilter {
			name: CString::new($filter.name).unwrap().as_ptr(),
			pattern: CString::new($filter.pattern).unwrap().as_ptr(),
		}
	};
}

/// See [`Platform::prompt_open_file()`] for documentation.
///
/// [`Platform::prompt_open_file()`]: super::Platform::prompt_open_file
pub(crate) fn open_file<'a>(
	filters: impl IntoIterator<Item = &'a DialogFileFilter<'a>>,
	allow_multiple: bool,
	default: Option<impl AsRef<Path>>,
	window: Option<&Window>,
) -> Vec<PathBuf> {
	unsafe {
		let (tx, rx) = oneshot::channel();
		let c_callback = Some(c_dialog_file_callback as unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int));
		let userdata = Box::into_raw(Box::new(tx)) as *mut c_void;
		let window = window.map(Window::sdl_window).unwrap_or_default();
		let filters = filters.into_iter().map(|filter| convert_filter!(filter)).collect::<Vec<_>>();
		let default = default.map(|d| CString::new(d.as_ref().to_str().unwrap()).unwrap());
		let default = default.map(|d| d.as_ptr()).unwrap_or_default();
		SDL_ShowOpenFileDialog(c_callback, userdata, window, filters.as_ptr(), filters.len() as i32, default, allow_multiple);
		rx.blocking_recv().unwrap()
	}
}

/// See [`Platform::prompt_open_folder()`] for documentation.
///
/// [`Platform::prompt_open_folder()`]: super::Platform::prompt_open_folder
pub(crate) fn open_folder(
	allow_multiple: bool,
	default: Option<impl AsRef<Path>>,
	window: Option<&Window>,
) -> Vec<PathBuf> {
	unsafe {
		let (tx, rx) = oneshot::channel();
		let c_callback = Some(c_dialog_file_callback as unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int));
		let userdata = Box::into_raw(Box::new(tx)) as *mut c_void;
		let window = window.map(Window::sdl_window).unwrap_or_default();
		let default = default.map(|d| CString::new(d.as_ref().to_str().unwrap()).unwrap());
		let default = default.map(|d| d.as_ptr()).unwrap_or_default();
		SDL_ShowOpenFolderDialog(c_callback, userdata, window, default, allow_multiple);
		rx.blocking_recv().unwrap()
	}
}

/// See [`Platform::prompt_save_file()`] for documentation.
///
/// [`Platform::prompt_save_file()`]: super::Platform::prompt_save_file
pub(crate) fn save_file<'a>(
	filters: impl IntoIterator<Item = &'a DialogFileFilter<'a>>,
	default: Option<impl AsRef<Path>>,
	window: Option<&Window>,
) -> Option<PathBuf> {
	unsafe {
		let (tx, rx) = oneshot::channel::<Vec<PathBuf>>();
		let c_callback = Some(c_dialog_file_callback as unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int));
		let userdata = Box::into_raw(Box::new(tx)) as *mut c_void;
		let window = window.map(Window::sdl_window).unwrap_or_default();
		let filters = filters.into_iter().map(|filter| convert_filter!(filter)).collect::<Vec<_>>();
		let default = default.map(|d| CString::new(d.as_ref().to_str().unwrap()).unwrap());
		let default = default.map(|d| d.as_ptr()).unwrap_or_default();
		SDL_ShowSaveFileDialog(c_callback, userdata, window, filters.as_ptr(), filters.len() as i32, default);
		rx.blocking_recv().unwrap().into_iter().next()
	}
}

/// The C-compatible dialogue file callback called by SDL. See
/// [`SDL_DialogFileCallback`].
///
/// [`open_file`], [`open_folder`], and [`save_file`] all leak a [`Sender`] and
/// place it into `userdata`. Here the sender is dereferenced, used to send back
/// file paths, and then properly released.
///
/// [`SDL_DialogFileCallback`]: https://wiki.libsdl.org/SDL3/SDL_DialogFileCallback
/// [`Sender`]: oneshot::Sender
extern "C" fn c_dialog_file_callback(userdata: *mut c_void, filelist: *const *const c_char, _filter: c_int) {
	unsafe {
		let tx = Box::from_raw(userdata as *mut oneshot::Sender<Vec<PathBuf>>);
		if filelist.is_null() { // Error, panics
			sdl_panic!()
		}
		else if (*filelist).is_null() { // No paths were selected, sends back an empty list
			tx.send(Vec::new()).unwrap();
		}
		else { // Sends back selected files
			// Stores null-terminated array `filelist` in a vector
			let mut vec = Vec::new();
			for i in 0.. {
				match (*filelist.add(i)).is_null() {
					true => break,
					false => vec.push(PathBuf::from(CStr::from_ptr(*filelist.add(i)).to_str().unwrap())),
				}
			}
			// Sends back files
			tx.send(vec).unwrap();
		}
	}
}