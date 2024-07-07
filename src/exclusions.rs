use ustr::{ustr, Ustr, UstrSet};

use crate::stream::Stream;

/// Wrapper around the UstrSet that represents a set of words that are excluded from reading.
pub struct Exclusions {
	words: UstrSet,
}

impl Exclusions {
	/// Creates Exclusions from a `Stream`.
	pub fn from_stream(stream: &mut Stream) -> Option<Self> {
		let content = stream.read_to_string()?;
		let mut words = UstrSet::default();

		for s in content.split_ascii_whitespace() {
			words.insert(ustr(s));
		}

		Some(Exclusions { words })
	}

        /// Checks if the word `s` is in the set.
	pub fn contains(&self, s: &Ustr) -> bool {
		self.words.contains(s)
	}
}
