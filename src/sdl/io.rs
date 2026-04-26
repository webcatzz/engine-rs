use std::ffi::c_void;
use std::io::{self, Read, Write, Seek, SeekFrom, ErrorKind};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{self, NonNull};
use std::slice;
use sdl3_sys::iostream::*;
use sdl::err::{non_null_or_sdl_panic, sdl_assert};
use crate::sdl;

/// Wraps an SDL IO stream.
///
/// Does NOT close the stream when dropped.
pub struct SdlIoStream<'a, T> {
	stream: NonNull<SDL_IOStream>,
	phantom: PhantomData<&'a T>,
}

impl<'a, T: Read> SdlIoStream<'a, T> {
	/// Opens a new SDL IO stream that allows reading.
	pub fn new_read(data: &'a mut T) -> Self {
		Self {
			stream: unsafe {
				non_null_or_sdl_panic(SDL_OpenIO(&SDL_IOStreamInterface {
					version: size_of::<SDL_IOStreamInterface>() as u32,
					size:    Some(iface_size_fail),
					seek:    Some(iface_seek_fail),
					read:    Some(iface_read::<T>),
					write:   Some(iface_write_fail_readonly),
					flush:   Some(iface_flush_fail_readonly),
					close:   Some(iface_close_noop),
				}, ptr::from_mut(data) as *mut c_void))
			},
			phantom: PhantomData,
		}
	}
}

impl<'a, T: Read + Seek> SdlIoStream<'a, T> {
	/// Opens a new SDL IO stream that allows reading and seeking.
	pub fn new_read_seek(data: &'a mut T) -> Self {
		Self {
			stream: unsafe {
				non_null_or_sdl_panic(SDL_OpenIO(&SDL_IOStreamInterface {
					version: size_of::<SDL_IOStreamInterface>() as u32,
					size:    Some(iface_size_fail),
					seek:    Some(iface_seek::<T>),
					read:    Some(iface_read::<T>),
					write:   Some(iface_write_fail_readonly),
					flush:   Some(iface_flush_fail_readonly),
					close:   Some(iface_close_noop),
				}, ptr::from_mut(data) as *mut c_void))
			},
			phantom: PhantomData,
		}
	}
}

impl<'a, T> SdlIoStream<'a, T> {
	/// Returns the [`SDL_IOStream`] pointer underlying an [`SdlIoStream`].
	pub fn sdl_stream(&self) -> *mut SDL_IOStream {
		self.stream.as_ptr()
	}
}

impl<'a, T> Drop for SdlIoStream<'a, T> {
	fn drop(&mut self) {
		unsafe { sdl_assert!(SDL_CloseIO(self.sdl_stream())); }
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::size`].
///
/// Always fails and returns -1.
extern "C" fn iface_size_fail(_userdata: *mut c_void) -> i64 {
	-1
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::seek`].
///
/// Seeks to `offset` relative to `whence`. Returns the final offset in the data
/// stream.
extern "C" fn iface_seek<T: Seek>(userdata: *mut c_void, offset: i64, whence: SDL_IOWhence) -> i64 {
	unsafe {
		let v = &mut *(userdata as *mut T);
		v.seek(match whence {
			SDL_IO_SEEK_SET => SeekFrom::Start(offset as u64),
			SDL_IO_SEEK_CUR => SeekFrom::Current(offset),
			SDL_IO_SEEK_END => SeekFrom::End(offset),
			_ => panic!("Unrecognized `SDL_IOWhence` variant"),
		}).unwrap() as i64
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::seek`].
///
/// Always fails and returns -1.
extern "C" fn iface_seek_fail(_userdata: *mut c_void, _offset: i64, _whence: SDL_IOWhence) -> i64 {
	-1
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::read`].
///
/// Reads up to `size` bytes from the data stream to the area pointed at by
/// `ptr`. On an incomplete read, sets `status` to an `SDL_IOStatus` value.
/// Returns the number of bytes read.
extern "C" fn iface_read<T: Read>(userdata: *mut c_void, ptr: *mut c_void, size: usize, status: *mut SDL_IOStatus) -> usize {
	unsafe {
		let v = &mut *(userdata as *mut T);
		match v.read(slice::from_raw_parts_mut(ptr as *mut u8, size)) {
			Ok(len) => {
				*status = if len == 0 { SDL_IO_STATUS_EOF } else { SDL_IO_STATUS_READY };
				len
			}
			Err(err) => {
				match err.kind() {
					io::ErrorKind::Interrupted =>
						*status = SDL_IO_STATUS_READY,
					kind => {
						eprintln!("SDL IO interface error: {kind}");
						*status = SDL_IO_STATUS_ERROR;
					}
				}
				0
			}
		}
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::read`].
///
/// Always fails with status `SDL_IO_STATUS_WRITEONLY` and returns `0`.
extern "C" fn iface_read_fail_writeonly(_userdata: *mut c_void, _ptr: *mut c_void, _size: usize, status: *mut SDL_IOStatus) -> usize {
	unsafe {
		*status = SDL_IO_STATUS_WRITEONLY;
		0
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::write`].
///
/// Writes up to `size` bytes from the area pointed at by `ptr` to the data
/// stream. On an incomplete write, sets `status` to an `SDL_IOStatus` value.
/// Returns the number of bytes written.
extern "C" fn iface_write<T: Write>(userdata: *mut c_void, ptr: *const c_void, size: usize, _status: *mut SDL_IOStatus) -> usize {
	unsafe {
		let v = &mut *(userdata as *mut T);
		v.write(slice::from_raw_parts(&*(ptr as *const u8), size)).unwrap()
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::write`].
///
/// Always fails with status `SDL_IO_STATUS_READONLY` and returns 0.
extern "C" fn iface_write_fail_readonly(_userdata: *mut c_void, _ptr: *const c_void, _size: usize, status: *mut SDL_IOStatus) -> usize {
	unsafe { *status = SDL_IO_STATUS_READONLY; }
	0
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::flush`].
///
/// Ensures data is written out if the stream is buffering. On failure, sets
/// `status` to an `SDL_IOStatus` value. Always returns true.
extern "C" fn iface_flush<T: Write>(userdata: *mut c_void, _status: *mut SDL_IOStatus) -> bool {
	unsafe {
		let v = &mut *(userdata as *mut T);
		v.flush().unwrap();
		true
	}
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::flush`].
///
/// Always fails with status `SDL_IO_STATUS_READONLY` and returns false.
extern "C" fn iface_flush_fail_readonly(_userdata: *mut c_void, status: *mut SDL_IOStatus) -> bool {
	unsafe { *status = SDL_IO_STATUS_READONLY; }
	false
}

/// SDL-compatible callback for [`SDL_IOStreamInterface::close`].
///
/// Does nothing. Always returns true.
extern "C" fn iface_close_noop(_userdata: *mut c_void) -> bool {
	true
}