use crate::math::{Color, Lerp};

/// A gradient of colors.
pub struct Gradient {
	/// Reference points used to sample the gradient.
	points: Vec<(f32, Color<f32>)>,
}

impl Gradient {

	/// Returns an empty gradient.
	pub fn new() -> Self {
		Self { points: Vec::new() }
	}

	/// Returns the color at the given offset in the gradient.
	///
	/// `offset` is clamped to \[0.0, 1.0]. If the gradient is empty, returns
	/// [`None`].
	pub fn sample(&self, mut offset: f32) -> Option<Color<f32>> {
		if self.points.is_empty() { return None; }
		offset = offset.clamp(0.0, 1.0);
		match self.bsearch(offset) {
			Ok(index) => Some(self.points[index].1),
			Err(index) => {
				let (prev_offset, prev_color) = self.points[index.saturating_sub(1)];
				let (next_offset, next_color) = self.points[index.min(self.points.len() - 1)];
				let weight = (offset - prev_offset) / (next_offset - prev_offset);
				Some(prev_color.lerp(&next_color, weight))
			}
		}
	}

	/// Inserts a reference point into the gradient.
	///
	/// `offset` is clamped to \[0.0, 1.0].
	pub fn insert_point(&mut self, mut offset: f32, color: Color<f32>) {
		offset = offset.clamp(0.0, 1.0);
		match self.bsearch(offset) {
			Ok(index) => self.points[index] = (offset, color),
			Err(index) => self.points.insert(index, (offset, color)),
		}
	}

	/// Removes a reference point from the gradient.
	pub fn remove_point(&mut self, index: usize) {
		self.points.remove(index);
	}

	/// Returns the offset of a reference point.
	pub fn point_offset(&self, index: usize) -> f32 {
		self.points[index].0
	}

	/// Returns the color of a reference point.
	pub fn point_color(&self, index: usize) -> Color<f32> {
		self.points[index].1
	}

	/// Sets the offset of a reference point.
	///
	/// `offset` is clamped to \[0.0, 1.0].
	pub fn set_point_offset(&mut self, index: usize, offset: f32) {
		self.points[index].0 = offset.clamp(0.0, 1.0);
	}

	/// Sets the color of a reference point.
	pub fn set_point_color(&mut self, index: usize, color: Color<f32>) {
		self.points[index].1 = color;
	}

	/// Returns the number of reference points in the gradient.
	pub fn point_count(&self) -> usize {
		self.points.len()
	}

	/// Returns an iterator over the reference points of a gradient.
	pub fn points(&self) -> impl Iterator<Item = (f32, Color<f32>)> {
		self.points.iter().copied()
	}

	/// Performs a binary search for `offset` in the gradient. If there is a
	/// reference point with that offset, returns `Ok` with the index of that
	/// point, otherwise returns `Err` with the index of the next reference point.
	fn bsearch(&self, offset: f32) -> Result<usize, usize> {
		self.points.binary_search_by(|(o, ..)| offset.partial_cmp(o).unwrap())
	}

}

impl FromIterator<(f32, Color<f32>)> for Gradient {
	fn from_iter<T: IntoIterator<Item = (f32, Color<f32>)>>(iter: T) -> Self {
		let mut gradient = Gradient::new();
		for (offset, color) in iter {
			gradient.insert_point(offset, color);
		}
		gradient
	}
}