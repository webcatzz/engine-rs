//! Main loop.

use std::time::Duration;
use crate::event::Event;
use crate::platform::Platform;

/// Types that can be run in a main loop. See [`Platform::main_loop()`].
pub trait Run<'a>: Sized {

	/// How often the main loop should update.
	const UPDATE_FREQ: Duration = Duration::from_nanos(1_000_000_000 / 60);
	/// How often the main loop should render.
	const RENDER_FREQ: Duration = Duration::from_nanos(1_000_000_000 / 60);

	/// Initializes and runs.
	fn run(platform: &'a Platform);

	/// Updates based on an event.
	#[allow(unused_variables)]
	fn listen(&mut self, event: &Event) {}

	/// Updates.
	fn update(&mut self) {}

	/// Renders.
	#[allow(unused_variables)]
	fn render(&mut self, delta: f32) {}

}