mod args;
mod stream;
use std::{collections::HashMap, process::exit};

use args::Cli;
use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;
use stream::Stream;
use ustr::{ustr, Ustr};

static WORD_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"[a-zA-Z0-9]([a-zA-Z0-9]|'|-)*").unwrap());

fn main() {
	let cargs = args::Cli::parse(); // CLI arguments

	let files = &cargs.files;

	if files.is_empty() {
		eprintln!("No files entered");
		exit(1);
	}

	let streams: Vec<_> = files.iter().filter_map(|f| Stream::from_str(f)).collect();

	if streams.is_empty() {
		eprintln!("Args does not contain any valid files to process");
		exit(1);
	}

	let res: Vec<_> = streams
		.into_iter()
		.filter_map(|s| do_stuff(s, &cargs))
		.collect();

	for p in res {
		println!("{:?}", p);
	}
}

fn do_stuff(mut s: Stream, cargs: &Cli) -> Option<HashMap<Ustr, i32>> {
	let content = s.read_to_string()?;
	let counts = count_words(content);

	counts
}

fn count_words(s: String) -> Option<HashMap<Ustr, i32>> {
	let tokens = WORD_REGEX.find_iter(&s).map(|m| m.as_str());
	let counts = tokens.fold(HashMap::new(), |mut a, c| {
		*a.entry(ustr(c)).or_insert(0) += 1;
		a
	});

	Some(counts)
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
		let res = count_words("lorem ipsum dolor".to_owned()).unwrap();

		assert_eq!(res[&ustr("lorem")], 1);
		assert_eq!(res[&ustr("ipsum")], 1);
		assert_eq!(res[&ustr("dolor")], 1);
	}

	#[test]
	fn word_count2() {
		let res = count_words("lorem dolor ipsum dolor. lorem? dolor dolor".to_owned())
			.unwrap();

		assert_eq!(res[&ustr("lorem")], 2);
		assert_eq!(res[&ustr("ipsum")], 1);
		assert_eq!(res[&ustr("dolor")], 4);
	}
}
