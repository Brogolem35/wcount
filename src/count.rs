use std::{
	collections::HashMap,
	hash::{BuildHasher, Hash},
};

use regex::Regex;
use ustr::{ustr, Ustr, UstrMap};

use crate::stream::Stream;

pub struct StreamWordCount {
	pub from: Stream,
	pub counts: UstrMap<usize>,
}

impl StreamWordCount {
	pub fn from_stream(mut stream: Stream, pattern: &'static Regex) -> Option<Self> {
		let content = stream.read_to_string()?;

		Some(StreamWordCount {
			from: stream,
			counts: Self::count_words(&content, pattern),
		})
	}

	pub fn to_ordered_vec(&self) -> Vec<(Ustr, usize)> {
		let mut res: Vec<_> = self
			.counts
			.iter()
			.map(|(s, i)| (s.clone(), i.clone()))
			.collect();
		res.sort_by(|(_, a), (_, b)| a.cmp(b));

		res
	}

	pub fn label(&self) -> String {
		self.from.label()
	}

	fn count_words(s: &str, pattern: &'static Regex) -> UstrMap<usize> {
		let tokens = pattern.find_iter(&s).map(|m| m.as_str());
		let counts = tokens.fold(UstrMap::default(), |mut a, c| {
			*a.entry(ustr(c)).or_insert(0) += 1;
			a
		});

		counts
	}

	pub fn count(&self, s: &Ustr) -> usize {
		self.counts.get(s).unwrap_or(&0).clone()
	}
}

#[derive(Clone)]
pub struct TotalCount {
	pub counts: UstrMap<usize>,
}

impl TotalCount {
	pub fn from_counts<'a, I>(swc: I) -> Self
	where
		I: Iterator<Item = &'a StreamWordCount>,
	{
		let mut counts = UstrMap::default();

		for wcounts in swc {
			Self::merge_maps(&mut counts, &wcounts.counts);
		}

		TotalCount { counts }
	}

	pub fn add_count(&mut self, swc: &StreamWordCount) {
		Self::merge_maps(&mut self.counts, &swc.counts);
	}

	pub fn to_ordered_vec(&self) -> Vec<(Ustr, usize)> {
		let mut res: Vec<_> = self
			.counts
			.iter()
			.map(|(s, i)| (s.clone(), i.clone()))
			.collect();
		res.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());

		res
	}

	fn merge_maps<K, H: BuildHasher>(a: &mut HashMap<K, usize, H>, b: &HashMap<K, usize, H>)
	where
		K: Eq + Hash + Clone,
	{
		for (w, c) in b.iter() {
			*a.entry(w.clone()).or_insert(0) += c;
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::regexes::DEFAULT_REGEX;

	use super::*;

	#[test]
	fn word_count1() {
		let res = StreamWordCount::count_words("lorem ipsum dolor", &DEFAULT_REGEX);

		assert_eq!(res[&ustr("lorem")], 1);
		assert_eq!(res[&ustr("ipsum")], 1);
		assert_eq!(res[&ustr("dolor")], 1);
	}

	#[test]
	fn word_count2() {
		let res = StreamWordCount::count_words(
			"lorem dolor ipsum dolor. lorem? dolor dolor",
			&DEFAULT_REGEX,
		);

		assert_eq!(res[&ustr("lorem")], 2);
		assert_eq!(res[&ustr("ipsum")], 1);
		assert_eq!(res[&ustr("dolor")], 4);
	}

	#[test]
	fn merge_maps1() {
		let mut map1 = HashMap::from([("lorem", 3), ("ipsum", 2), ("dolor", 17)]);
		let map2 = HashMap::from([("lorem", 27), ("ipsum", 29), ("dolor", 15)]);

		TotalCount::merge_maps(&mut map1, &map2);

		assert_eq!(
			map1,
			HashMap::from([("lorem", 30), ("ipsum", 31), ("dolor", 32)])
		)
	}

	#[test]
	fn merge_maps2() {
		let mut map1 = HashMap::from([
			(ustr("lorem"), 3),
			(ustr("ipsum"), 2),
			(ustr("dolor"), 17),
		]);
		let map2 = HashMap::from([
			(ustr("lorem"), 27),
			(ustr("ipsum"), 29),
			(ustr("dolor"), 15),
		]);

		TotalCount::merge_maps(&mut map1, &map2);

		assert_eq!(
			map1,
			HashMap::from([
				(ustr("lorem"), 30),
				(ustr("ipsum"), 31),
				(ustr("dolor"), 32)
			])
		)
	}
}
