use std::{collections::HashMap, hash::Hash};

use once_cell::sync::Lazy;
use regex::Regex;
use ustr::{ustr, Ustr};

use crate::stream::Stream;

static WORD_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"[a-zA-Z0-9]([a-zA-Z0-9]|'|-)*").unwrap());

pub struct StreamWordCount {
	pub from: Stream,
	pub counts: HashMap<Ustr, usize>,
}

impl StreamWordCount {
	pub fn from_stream(mut stream: Stream) -> Option<Self> {
		let content = stream.read_to_string()?;

		Some(StreamWordCount {
			from: stream,
			counts: Self::count_words(&content),
		})
	}

	fn count_words(s: &str) -> HashMap<Ustr, usize> {
		let tokens = WORD_REGEX.find_iter(&s).map(|m| m.as_str());
		let counts = tokens.fold(HashMap::new(), |mut a, c| {
			*a.entry(ustr(c)).or_insert(0) += 1;
			a
		});

		counts
	}
}

pub struct TotalCount {
	pub counts: HashMap<Ustr, usize>,
}

impl TotalCount {
	pub fn from_counts<'a, I>(swc: I) -> Self
	where
		I: Iterator<Item = &'a StreamWordCount>,
	{
		let mut counts = HashMap::new();

		for wcounts in swc {
			Self::merge_maps(&mut counts, &wcounts.counts);
		}

		TotalCount { counts }
	}

	pub fn add_count(&mut self, swc: &StreamWordCount) {
		Self::merge_maps(&mut self.counts, &swc.counts);
	}

	fn merge_maps<K>(a: &mut HashMap<K, usize>, b: &HashMap<K, usize>)
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
	use super::*;

	#[test]
	fn regex1() {
		let rres: Vec<_> = WORD_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn regex2() {
		let rres: Vec<_> = WORD_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn regex3() {
		let rres: Vec<_> = WORD_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "3or"]);
	}

	#[test]
	fn regex4() {
		let rres: Vec<_> = WORD_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2d3"]);
	}

	#[test]
	fn word_count1() {
		let res = StreamWordCount::count_words("lorem ipsum dolor");

		assert_eq!(res[&ustr("lorem")], 1);
		assert_eq!(res[&ustr("ipsum")], 1);
		assert_eq!(res[&ustr("dolor")], 1);
	}

	#[test]
	fn word_count2() {
		let res =
			StreamWordCount::count_words("lorem dolor ipsum dolor. lorem? dolor dolor");

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
