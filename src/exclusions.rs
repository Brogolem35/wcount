use ustr::{ustr, Ustr, UstrSet};

use crate::stream::Stream;

pub struct Exclusions {
	words: UstrSet,
}

impl Exclusions {
	pub fn from_stream(stream: &mut Stream) -> Option<Self> {
		let content = stream.read_to_string()?;
		let mut words = UstrSet::default();

		for s in content.split_ascii_whitespace() {
			words.insert(ustr(s));
		}

		Some(Exclusions { words })
	}

	pub fn contains(&self, s: &Ustr) -> bool {
		self.words.contains(s)
	}
}
