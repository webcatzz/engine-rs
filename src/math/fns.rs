//! Common math functions

use std::ops::{Add, Sub, Mul, Div, Range};
use crate::math::Vec2;

/// Based on the [midpoint circle algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm#Jesko's_method).
pub fn bresenham_circle(pos: impl Into<Vec2<i32>>, r: u32, mut f: impl FnMut(Vec2<i32>)) {
	let pos = pos.into();
	let r = r.cast_signed();
	let mut cur = pos + Vec2 { x: r, y: 0 };
	let mut t1 = r / 16;
	while cur.x >= cur.y {
		f(cur);
		cur.y += 1;
		t1 += cur.y;
		let t2 = t1 - cur.x;
		if t2 >= 0 {
			t1 = t2;
			cur.x -= 1;
		}
	}
}

/// Maps a value from one range to another.
pub fn remap<T>(v: T, from: Range<T>, to: Range<T>) -> T
where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
{
	(v - from.start) / (from.end - from.start) * (to.end - to.start) + to.start
}