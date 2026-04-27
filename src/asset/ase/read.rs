//! Aseprite file parsing.
//!
//! This module is not complete. Many Aseprite features are not supported.
//!
//! Uses the [Aseprite file specifications] as reference.
//!
//! [Aseprite file specifications]:
//!     https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md

use std::ffi::{c_int, c_void};
use std::io::{self, Read, Seek};
use std::ptr;
use sdl3_sys::pixels::*;
use sdl3_sys::render::*;
use crate::asset::FromBytes;
use crate::math::{Color, Rect, Vec2};
use crate::sdl::err::{non_null_or_sdl_panic, sdl_assert};
use crate::window;
use super::*;

// Aseprite file value types. For reference, see [References] in the file
// specifications.
//
// [References]:
//     https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md#references

type AseByte   = u8;
type AseWord   = u16;
type AseDWord  = u32;
type AseQWord  = u64;
type AseUuid   = u128;
type AseShort  = i16;
type AseLong   = i32;
type AseLong64 = i64;
type AseFloat  = f32;
type AseDouble = f64;

/// Defines a read function for a numeric primitive.
macro_rules! read_fn {
	( $name:ident -> $ty:ident ) => {
		fn $name(&mut self) -> io::Result<$ty> {
			let mut bytes = [0; size_of::<$ty>()];
			self.read_exact(&mut bytes)?;
			Ok($ty::from_le_bytes(bytes)) // Aseprite files always store data in little-endian format
		}
	};
}

/// A [`Read`] extension trait for reading Aseprite-file-specific types.
trait AseReadExt: Read {

	read_fn!(read_byte   -> AseByte);
	read_fn!(read_word   -> AseWord);
	read_fn!(read_dword  -> AseDWord);
	read_fn!(read_qword  -> AseQWord);
	read_fn!(read_uuid   -> AseUuid);
	read_fn!(read_short  -> AseShort);
	read_fn!(read_long   -> AseLong);
	read_fn!(read_long64 -> AseLong64);
	read_fn!(read_float  -> AseFloat);
	read_fn!(read_double -> AseDouble);

	/// Reads `n` bytes.
	fn read_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
		let mut buf = vec![0; n];
		self.read_exact(&mut buf)?;
		Ok(buf)
	}

	/// Reads a string value.
	fn read_string(&mut self) -> io::Result<String> {
		let len = self.read_word().unwrap();
		let chars = self.read_bytes(len as usize)?;
		String::from_utf8(chars)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("Couldn't read Aseprite string as valid UTF-8: {err}")))
	}

}

impl<T: Read> AseReadExt for T {}

// Aseprite file flag and enum values. For reference, see [Header] and [Chunk
// types] in the file specifications.
//
// [Header]:
//     https://github.com/aseprite/aseprite/blob/main/docs/CODING_STYLE.md#header
// [Chunk types]:
//     https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md#chunk-types

const HEADER_FLAG_LAYERS_HAVE_OPACITY:                       AseDWord = 1;
const HEADER_FLAG_LAYERS_GROUPS_HAVE_OPACITY_AND_BLEND_MODE: AseDWord = 2;
const HEADER_FLAG_LAYERS_HAVE_UUID:                          AseDWord = 4;

const CHUNK_TYPE_PALETTE_OLD_1:  AseWord = 0x0004;
const CHUNK_TYPE_PALETTE_OLD_2:  AseWord = 0x0011;
const CHUNK_TYPE_LAYER:          AseWord = 0x2004;
const CHUNK_TYPE_CEL:            AseWord = 0x2005;
const CHUNK_TYPE_CEL_EXTRA:      AseWord = 0x2006;
const CHUNK_TYPE_COLOR_PROFILE:  AseWord = 0x2007;
const CHUNK_TYPE_EXTERNAL_FILES: AseWord = 0x2008;
const CHUNK_TYPE_MASK:           AseWord = 0x2016;
const CHUNK_TYPE_PATH:           AseWord = 0x2017;
const CHUNK_TYPE_TAGS:           AseWord = 0x2018;
const CHUNK_TYPE_PALETTE:        AseWord = 0x2019;
const CHUNK_TYPE_USER_DATA:      AseWord = 0x2020;
const CHUNK_TYPE_SLICE:          AseWord = 0x2022;
const CHUNK_TYPE_TILESET:        AseWord = 0x2023;

const CEL_TYPE_RAW_IMAGE_DATA:     AseWord = 0;
const CEL_TYPE_LINKED_CEL:         AseWord = 1;
const CEL_TYPE_COMPRESSED_IMAGE:   AseWord = 2;
const CEL_TYPE_COMPRESSED_TILEMAP: AseWord = 3;

const COLOR_PROFILE_TYPE_NONE: AseWord = 0;
const COLOR_PROFILE_TYPE_SRGB: AseWord = 1;
const COLOR_PROFILE_TYPE_ICC:  AseWord = 2;

const COLOR_PROFILE_FLAG_USE_FIXED_GAMMA: AseWord = 1;

const LAYER_TYPE_NORMAL:  AseWord = 0;
const LAYER_TYPE_GROUP:   AseWord = 1;
const LAYER_TYPE_TILEMAP: AseWord = 2;

const LAYER_FLAG_VISIBLE:            AseWord = 1;
const LAYER_FLAG_EDITABLE:           AseWord = 2;
const LAYER_FLAG_LOCK_MOVEMENT:      AseWord = 4;
const LAYER_FLAG_BACKGROUND:         AseWord = 8;
const LAYER_FLAG_PREFER_LINKED_CELS: AseWord = 16;
const LAYER_FLAG_COLLAPSED:          AseWord = 32;
const LAYER_FLAG_REFERENCE:          AseWord = 64;

const PALETTE_FLAG_HAS_NAME: AseWord = 1;

const SLICE_FLAG_NINE_PATCH: AseDWord = 1;
const SLICE_FLAG_PIVOT:      AseDWord = 2;

impl FromBytes for Aseprite {

	type Params<'a> = &'a window::Frame<'a>;

	fn from_bytes(bytes: &mut (impl Read + Seek), params: Self::Params<'_>) -> io::Result<Self> {
		// Sets up vectors
		let mut frames = Vec::new();
		let mut layers = Vec::new();
		let mut palette = Vec::new();
		let mut slices = Vec::new();
		let mut tags = Vec::new();
		// Reads header
		let mut header = {
			let mut buf = [0; 128];
			bytes.read_exact(&mut buf)?;
			io::Cursor::new(buf)
		};
		let file_size = header.read_dword()?;
		if !matches!(header.read_word(), Ok(0xA5E0)) { return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number in Aseprite file")); }
		let frame_count = header.read_word()? as usize;
		let size = Vec2 { x: header.read_word()?, y: header.read_word()? };
		let color_depth = header.read_word()?;
		let header_flags = header.read_dword()?;
		header.seek_relative((size_of::<AseWord>() + size_of::<AseDWord>() * 2) as i64)?; // Skips deprecated and unused values
		let transparent_color_index = header.read_byte()?;
		header.seek_relative(3)?; // Skips unused values
		let color_count = header.read_word()?;
		let pixel_size = Vec2 { x: header.read_byte()?, y: header.read_byte()? };
		let grid_offset = Vec2 { x: header.read_short()?, y: header.read_short()? };
		let grid_size = Vec2 { x: header.read_word()?, y: header.read_word()? };
		header.seek_relative(84)?; // Skips unused values
		// Detects color mode and texture format
		let (color_mode, sdl_texture_format) = match color_depth {
			32 => (ColorMode::Rgba, SDL_PIXELFORMAT_ABGR8888),
			// 16 => (ColorMode::Grayscale, todo!()),
			8  => (ColorMode::Indexed, SDL_PIXELFORMAT_INDEX8),
			_  => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown color depth in Aseprite file: {color_depth}"))),
		};
		// Reads frames
		for frame_index in 0..frame_count {
			let mut frame_header = {
				let mut buf = [0; 16];
				bytes.read_exact(&mut buf)?;
				io::Cursor::new(buf)
			};
			let frame_byte_count = frame_header.read_dword()?;
			if !matches!(frame_header.read_word(), Ok(0xF1FA)) { return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number in Aseprite frame header")); }
			let frame_chunk_count_old = frame_header.read_word()?;
			let frame_duration_ms = frame_header.read_word()?;
			frame_header.seek_relative(2)?; // Skips unused values
			let frame_chunk_count_new = frame_header.read_dword()?;
			let frame_chunk_count = if frame_chunk_count_new == 0 { frame_chunk_count_old as u32 } else { frame_chunk_count_new };
			// Sets up vectors
			let mut cels = Vec::new();
			// Reads chunks
			for _ in 0..frame_chunk_count {
				// Reads chunk header
				let chunk_size = bytes.read_dword()?;
				let chunk_type = bytes.read_word()?;
				let mut chunk_data = io::Cursor::new(bytes.read_bytes(chunk_size as usize - size_of::<AseDWord>() - size_of::<AseWord>())?);
				// Reads chunk based on type
				match chunk_type {
					// Old palette chunk
					CHUNK_TYPE_PALETTE_OLD_1 => {
						// Reads palette header
						let packet_count = chunk_data.read_word()?;
						// Reads palette packets
						for _ in 0..packet_count {
							// Reads packet header
							let skip_count = chunk_data.read_byte()? as usize;
							let color_count = chunk_data.read_byte()? as usize;
							let color_count = if color_count == 0 { 256 } else { color_count };
							// Reserves space in `palette`
							palette.resize((skip_count + color_count).max(palette.len()), PaletteEntry { name: None, color: Color::BLACK });
							// Reads packet colors
							for i in 0..color_count {
								// Reads packet color
								let color_r = chunk_data.read_byte()?;
								let color_g = chunk_data.read_byte()?;
								let color_b = chunk_data.read_byte()?;
								// Adds packet to `palette`
								palette[skip_count + i] = PaletteEntry {
									name: None,
									color: Color {
										r: color_r,
										g: color_g,
										b: color_b,
										a: u8::MAX,
									},
								}
							}
						}
					}
					// Layer chunks
					CHUNK_TYPE_LAYER => {
						// Reads layer data
						let layer_flags = chunk_data.read_word()?;
						let layer_type = chunk_data.read_word()?;
						let layer_child_level = chunk_data.read_word()?;
						chunk_data.seek_relative(size_of::<AseWord>() as i64 * 2)?; // Skips unused space
						let layer_blend_mode = chunk_data.read_word()?;
						let layer_opacity = chunk_data.read_byte()?;
						chunk_data.seek_relative(3)?; // Skips unused space
						let layer_name = chunk_data.read_string()?;
						let layer_tileset_index = if layer_type != 2 { None } else {
							Some(chunk_data.read_dword()?)
						};
						let layer_uuid = if header_flags & HEADER_FLAG_LAYERS_HAVE_UUID == 0 { None } else {
							Some(chunk_data.read_uuid()?)
						};
						// Adds layer to `layers`
						layers.push(Layer {
							name: layer_name,
							kind: match layer_type {
								LAYER_TYPE_NORMAL => LayerKind::Normal,
								LAYER_TYPE_GROUP => LayerKind::Group,
								LAYER_TYPE_TILEMAP => LayerKind::Tilemap,
								_ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown Aseprite layer type: {layer_type}"))),
							},
							visible: layer_flags & LAYER_FLAG_VISIBLE != 0,
							opacity: layer_opacity,
							uuid: layer_uuid,
						})
					}
					// Cel chunks
					CHUNK_TYPE_CEL => {
						// Reads cel header
						let cel_layer_index = chunk_data.read_word()? as usize;
						let cel_pos_x = chunk_data.read_short()?;
						let cel_pos_y = chunk_data.read_short()?;
						let cel_opacity = chunk_data.read_byte()?;
						let cel_type = chunk_data.read_word()?;
						let cel_z_index = chunk_data.read_short()?;
						chunk_data.seek_relative(5)?; // Skips unused space
						// Reads cell content
						let cel_content = match cel_type {
							// Raw image data cels
							CEL_TYPE_RAW_IMAGE_DATA => {
								// Reads cell data
								let cel_width = chunk_data.read_word()?;
								let cel_height = chunk_data.read_word()?;
								let mut cel_pixels = Vec::new();
								chunk_data.read_to_end(&mut cel_pixels)?;
								// Creates SDL texture
								let cel_texture = Texture::from_sdl_texture(non_null_or_sdl_panic(unsafe { SDL_CreateTexture(params.sdl_renderer(), sdl_texture_format, SDL_TEXTUREACCESS_STATIC, cel_width as c_int, cel_height as c_int) }));
								unsafe { sdl_assert!(SDL_UpdateTexture(cel_texture.sdl_texture(), ptr::null(), cel_pixels.as_ptr() as *const c_void, cel_width as c_int * 4)); }
								// Returns cel content
								CelContent::Texture { texture: cel_texture }
							},
							// Linked cels
							CEL_TYPE_LINKED_CEL => {
								// Reads cell data
								let cel_linked_frame_index = chunk_data.read_word()? as usize;
								// Returns cel content
								CelContent::Linked { linked_frame_index: cel_linked_frame_index }
							}
							// Compressed image cels
							CEL_TYPE_COMPRESSED_IMAGE => {
								// Reads cell data
								let cel_width = chunk_data.read_word()?;
								let cel_height = chunk_data.read_word()?;
								let mut cel_pixels_compressed = Vec::new();
								chunk_data.read_to_end(&mut cel_pixels_compressed)?;
								let cel_pixels = zlib::decompress(cel_pixels_compressed.as_slice()).unwrap();
								debug_assert_eq!(cel_pixels.len(), cel_width as usize * cel_height as usize * color_depth as usize / 8);
								// Creates SDL texture
								let cel_texture = Texture::from_sdl_texture(non_null_or_sdl_panic(unsafe { SDL_CreateTexture(params.sdl_renderer(), sdl_texture_format, SDL_TEXTUREACCESS_STATIC, cel_width as c_int, cel_height as c_int) }));
								unsafe { sdl_assert!(SDL_UpdateTexture(cel_texture.sdl_texture(), ptr::null(), cel_pixels.as_ptr() as *const c_void, cel_width as c_int * 4)); }
								// Returns cel content
								CelContent::Texture { texture: cel_texture }
							},
							// Compressed tilemap cels
							CEL_TYPE_COMPRESSED_TILEMAP => todo!("Tilemaps are not yet supported"),
							// Unknown cels
							_ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown Aseprite cel type: {cel_type}"))),
						};
						// Adds cel to `cels`
						cels.push(Cel {
							frame_index,
							layer_index: cel_layer_index,
							pos: Vec2 { x: cel_pos_x, y: cel_pos_y },
							opacity: cel_opacity,
							z_index: cel_z_index,
							content: cel_content,
						});
					}
					// Color profile chunks
					CHUNK_TYPE_COLOR_PROFILE => {
						let color_profile_type = chunk_data.read_word()?;
						let color_profile_flags = chunk_data.read_word()?;
						chunk_data.seek_relative(4)?; // TODO (fixed data)
						chunk_data.seek_relative(8)?; // Skips unused space
						if color_profile_type == COLOR_PROFILE_TYPE_ICC {
							let icc_profile_data_len = chunk_data.read_dword()?;
							let icc_profile_data = chunk_data.read_bytes(icc_profile_data_len as usize)?;
						}
					}
					// Mask chunks
					CHUNK_TYPE_MASK => {
						// Skips deprecated mask chunk
						chunk_data.seek_relative(size_of::<AseShort>() as i64 * 2)?;
						let mask_width = chunk_data.read_word()?;
						let mask_height = chunk_data.read_word()?;
						chunk_data.seek_relative(8)?;
						let mask_name_len = chunk_data.read_word()?;
						chunk_data.seek_relative(mask_name_len as i64 + mask_height as i64 * ((mask_width as i64 + 7) / 8))?;
					}
					// Tag chunks
					CHUNK_TYPE_TAGS => {
						// Reads tags header
						let tag_count = chunk_data.read_word()?;
						chunk_data.seek_relative(8)?; // Skips unused space
						// Reads tags
						for _ in 0..tag_count {
							// Reads tag
							let tag_from_frame = chunk_data.read_word()?;
							let tag_to_frame = chunk_data.read_word()?;
							let tag_animation_direction = chunk_data.read_byte()?;
							let tag_repeat_count = chunk_data.read_word()?;
							chunk_data.seek_relative(6)?; // Skips unused space
							let tag_color_r = chunk_data.read_byte()?;
							let tag_color_g = chunk_data.read_byte()?;
							let tag_color_b = chunk_data.read_byte()?;
							chunk_data.seek_relative(1)?; // Skips unused space
							let tag_name = chunk_data.read_string()?;
							// Adds tag to `tags`
							tags.push(Tag {
								name: tag_name,
								color: Color { r: tag_color_r, g: tag_color_g, b: tag_color_b, a: 255 },
								frames: tag_from_frame..tag_to_frame - tag_from_frame,
								animation_direction: match tag_animation_direction {
									0 => AnimationDirection::Forward,
									1 => AnimationDirection::Reverse,
									2 => AnimationDirection::PingPong,
									3 => AnimationDirection::PingPongReverse,
									_ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown Aseprite tag animation direction: {tag_animation_direction}"))),
								},
								repeat_count: tag_repeat_count,
							});
						}
					}
					// Palette chunks
					CHUNK_TYPE_PALETTE => {
						// Reads palette header
						let palette_entry_count = chunk_data.read_dword()?;
						let palette_first_index = chunk_data.read_dword()?;
						let palette_last_index = chunk_data.read_dword()?;
						chunk_data.seek_relative(8)?; // Skips unused space
						// Reserves space in `palette`
						palette.resize(palette_entry_count as usize, PaletteEntry { name: None, color: Color::BLACK });
						// Reads palette entries
						for i in palette_first_index as usize..palette_last_index as usize {
							// Reads palette entry
							let palette_entry_flags = chunk_data.read_word()?;
							let palette_entry_color_r = chunk_data.read_byte()?;
							let palette_entry_color_g = chunk_data.read_byte()?;
							let palette_entry_color_b = chunk_data.read_byte()?;
							let palette_entry_color_a = chunk_data.read_byte()?;
							let palette_entry_name = match palette_entry_flags & PALETTE_FLAG_HAS_NAME {
								0 => None,
								_ => Some(chunk_data.read_string()?),
							};
							// Adds palette entry to `palette`
							palette[i] = PaletteEntry {
								name: palette_entry_name,
								color: Color {
									r: palette_entry_color_r,
									g: palette_entry_color_g,
									b: palette_entry_color_b,
									a: palette_entry_color_a,
								},
							};
						}
					}
					// User data chunks
					CHUNK_TYPE_USER_DATA => {} // Discards user data
					// Slice chunks
					CHUNK_TYPE_SLICE => {
						// Reads slice header
						let slice_key_count = chunk_data.read_dword()?;
						let slice_flags = chunk_data.read_dword()?;
						chunk_data.seek_relative(size_of::<AseDWord>() as i64)?; // Skips unused space
						let slice_name = chunk_data.read_string()?;
						// Reserves space in `slices`
						slices.reserve_exact(slice_key_count as usize);
						// Reads slices
						for _ in 0..slice_key_count {
							// Reads slice data
							let slice_frame_number = chunk_data.read_dword()?;
							let slice_pos_x = chunk_data.read_long()?;
							let slice_pos_y = chunk_data.read_long()?;
							let slice_width = chunk_data.read_dword()?;
							let slice_height = chunk_data.read_dword()?;
							let slice_nine_patch = if slice_flags & SLICE_FLAG_NINE_PATCH == 0 { None } else {
								Some(Rect {
									pos: Vec2 { x: chunk_data.read_long()?, y: chunk_data.read_long()? },
									size: Vec2 { x: chunk_data.read_dword()?, y: chunk_data.read_dword()? },
								})
							};
							let slice_pivot = if slice_flags & SLICE_FLAG_PIVOT == 0 { None } else {
								Some(Vec2 {
									x: chunk_data.read_long()?,
									y: chunk_data.read_long()?,
								})
							};
							// Adds slice to `slices`
							slices.push(Slice {
								name: slice_name.clone(),
								rect: Rect {
									pos: Vec2 { x: slice_pos_x, y: slice_pos_y },
									size: Vec2 { x: slice_width, y: slice_height },
								},
								nine_patch: slice_nine_patch,
								pivot: slice_pivot,
							});
						}
					}
					// Unknown chunks
					_ => {}
				}
			}
			// Adds frame to `frames`
			frames.push(Frame { cels });
		}
		// Sets palette for indexed sprites
		if color_mode == ColorMode::Indexed {
			// Creates SDL palette
			let sdl_palette = unsafe { SDL_CreatePalette(palette.len() as c_int) };
			assert!(!sdl_palette.is_null());
			let sdl_colors = Vec::from_iter(palette.iter().map(|entry| entry.color.into()));
			unsafe { sdl_assert!(SDL_SetPaletteColors(sdl_palette, sdl_colors.as_ptr(), 0, sdl_colors.len() as c_int)); }
			// Updates cel palettes
			for frame in frames.iter_mut() {
				for cel in frame.cels.iter_mut() {
					if let CelContent::Texture { texture } = &mut cel.content {
						unsafe { sdl_assert!(SDL_SetTexturePalette(texture.sdl_texture(), sdl_palette)); }
					}
				}
			}
		}
		// Returns file data
		Ok(Aseprite {
			size,
			pixel_size,
			grid_offset,
			grid_size,
			color_mode,
			frames,
			tags,
			palette,
			layers,
			slices,
		})
	}

}