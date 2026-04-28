//! Aseprite files.

use std::time::Duration;
use std::ops::Range;
use crate::math::{Color, Rect, Transform, Vec2};
use super::{Image, Texture};

mod read;
mod zlib;

/// An Aseprite sprite.
pub struct Aseprite {
	size: Vec2<u16>,
	pub pixel_size: Vec2<u8>,
	pub grid_offset: Vec2<i16>,
	pub grid_size: Vec2<u16>,
	pub color_mode: ColorMode,

	pub palette: Vec<PaletteEntry>,
	pub frames:  Vec<Frame>,
	pub layers:  Vec<Layer>,
	pub tags:    Vec<Tag>,
	pub slices:  Vec<Slice>,
}

/// The color mode of an Aseprite file. See the [Color mode] documentation.
///
/// [Color mode]: https://www.aseprite.org/docs/color-mode
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
	/// Each pixel is an RGBA color.
	Rgba,
	/// Each pixel is a grayscale value.
	Grayscale,
	/// Each pixel is an index into a palette.
	Indexed,
}

pub struct Frame {
	pub duration: Duration,
	pub cels: Vec<Cel>,
}

pub struct Cel {
	pub frame_index: usize,
	pub layer_index: usize,
	pub pos: Vec2<i16>,
	pub opacity: u8,
	pub z_index: i16,
	pub content: CelContent,
}

pub enum CelContent {
	Linked { linked_frame_index: usize },
	Texture { texture: Texture },
}

/// A layer in an Aseprite file. See the [Layers] documentation.
///
/// [Layers]: https://www.aseprite.org/docs/layers/
#[derive(Clone)]
pub struct Layer {
	pub name: String,
	pub kind: LayerKind,
	pub visible: bool,
	pub opacity: u8,
	pub uuid: Option<u128>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayerKind {
	Normal,
	Group,
	Tilemap,
}

/// A tag in an Aseprite file. See the [Tags] documentation.
///
/// [Tags]: https://www.aseprite.org/docs/tags/
#[derive(Clone)]
pub struct Tag {
	/// The name of the tag, as displayed in the editor.
	pub name: String,
	/// The color of the tag, as displayed in the editor.
	pub color: Color<u8>,
	/// The range of frames covered by the tag.
	pub frames: Range<u16>,
	/// The animation direction of the tag.
	pub animation_direction: AnimationDirection,
	/// The number of times the tag loops its animation.
	pub repeat_count: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
	/// The animation plays forwards.
	Forward,
	/// The animation plays backwards.
	Reverse,
	/// The animation alternates between playing forwards and backwards.
	PingPong,
	/// The animation alternates between playing backwards and forwards.
	PingPongReverse,
}

/// A slice in an Aseprite file. See the [Slices] documentation.
///
/// [Slices]: https://www.aseprite.org/docs/slices/
#[derive(Clone)]
pub struct Slice {
	/// The name of the slice.
	pub name: String,
	/// The rectangle taken up by the slice.
	pub rect: Rect<i32, u32>,
	/// The center patch of the slice, if any. Relative to the slice origin.
	pub nine_patch: Option<Rect<i32, u32>>,
	/// The coordinates of the pivot point, if any. Relative to the slice origin.
	pub pivot: Option<Vec2<i32>>,
}

/// An palette entry in an Aseprite file.
#[derive(Clone)]
pub struct PaletteEntry {
	/// The name of the entry, if any.
	pub name: Option<String>,
	/// The color of the entry.
	pub color: Color<u8>,
}

pub struct ExternalFile {

}

pub enum ExternalFileType {
	Palette,
	Tileset,
}

impl Aseprite {

	/// Returns the dimensions of the sprite.
	pub fn size(&self) -> Vec2<u16> {
		self.size
	}

	pub fn cel_texture(&self, layer: usize, frame: usize) -> Option<&Texture> {
		let cel = self.frames[frame].cels.iter().find(|cel| cel.layer_index == layer)?;
		match &cel.content {
			CelContent::Texture { texture } => Some(texture),
			CelContent::Linked { linked_frame_index } => self.cel_texture(layer, *linked_frame_index),
		}
	}

	pub fn slice_by_name(&self, name: &str) -> Option<&Slice> {
		self.slices.iter().find(|slice| slice.name == name)
	}

	pub fn layer(&self, name: &str) -> Option<&Layer> {
		self.layers.iter().find(|layer| layer.name == name)
	}

}