//! System events.

use std::ffi::CStr;
use sdl3_sys::events::*;
use crate::math::Vec2;
use super::key::{Key, KeyMod};
use super::mouse::{MouseBtn, MouseBtns};

/// A system event.
#[derive(Clone, PartialEq)]
pub enum Event {
	/// The program is requested to quit.
	Quit,
	/// A key is pressed or released.
	Key {
		key: Key,
		modifiers: KeyMod,
		down: bool,
	},
	/// Text is input.
	Text {
		text: String,
	},
	/// A mouse button is pressed or released.
	MouseButton {
		btn: MouseBtn,
		pos: Vec2<f32>,
		down: bool,
	},
	/// A mouse is moved.
	MouseMotion {
		pos: Vec2<f32>,
		motion: Vec2<f32>,
		btns: MouseBtns,
	},
	/// A window is resized.
	WindowResize {
		size: Vec2<u32>,
	}
}

impl Event {

	pub fn down(&self) -> Option<bool> {
		match self {
			Self::Key { down, .. } => Some(*down),
			Self::MouseButton { down, .. } => Some(*down),
			_ => None,
		}
	}

}

impl TryFrom<SDL_Event> for Event {

	type Error = String;

	fn try_from(event: SDL_Event) -> Result<Self, Self::Error> {
		unsafe {
			match SDL_EventType(event.r#type) {
				SDL_EVENT_QUIT => Ok(Self::Quit),
				SDL_EVENT_KEY_DOWN |
				SDL_EVENT_KEY_UP => Ok(Self::Key {
					key: event.key.key.into(),
					modifiers: event.key.r#mod.into(),
					down: event.key.down,
				}),
				SDL_EVENT_TEXT_INPUT => Ok(Self::Text {
					text: CStr::from_ptr(event.text.text).to_str().unwrap().to_string(),
				}),
				SDL_EVENT_MOUSE_BUTTON_DOWN |
				SDL_EVENT_MOUSE_BUTTON_UP => Ok(Self::MouseButton {
					pos: Vec2 { x: event.button.x as f32, y: event.button.y as f32 },
					btn: MouseBtn::from_sdl_index(event.button.button),
					down: event.button.down,
				}),
				SDL_EVENT_MOUSE_MOTION => Ok(Self::MouseMotion {
					pos: Vec2 { x: event.motion.x as f32, y: event.motion.y as f32 },
					motion: Vec2 { x: event.motion.xrel as f32, y: event.motion.yrel as f32 },
					btns: event.motion.state.into(),
				}),
				SDL_EVENT_WINDOW_RESIZED => Ok(Self::WindowResize {
					size: Vec2 { x: event.window.data1 as u32, y: event.window.data2 as u32 },
				}),
				_ => Err(format!("No `Event` representation for SDL event of type: {}", event.r#type)),
			}
		}
	}

}