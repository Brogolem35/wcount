mod args;
use std::{
	collections::HashMap,
	fs::{self, read_to_string, File},
	io::{self, Read},
	path::PathBuf,
	process::exit,
};

use args::Cli;
use clap::{builder::Str, Parser};
use once_cell::sync::Lazy;
use regex::Regex;
use ustr::{ustr, Ustr};

#[derive(Debug)]
enum Stream {
	Stdin(io::Stdin),
	File(File, String),
}

impl Stream {
	fn from_str(path: &str) -> Option<Stream> {
		if path == "-" {
			return Some(Stream::Stdin(io::stdin()));
		}

		match fs::metadata(path) {
			Ok(meta) => {
				if meta.is_file() {
					if let Ok(file) = File::open(path) {
						Some(Stream::File(file, path.to_string()))
					} else {
						eprintln!("{}: Error accessing", path);
						None
					}
				} else if meta.is_dir() {
					eprintln!("{}: Is a directory", path);
					None
				} else {
					eprintln!("{}: Error accessing", path);
					None
				}
			}
			Err(e) => {
				eprintln!("{}: {}", path, e);
				None
			}
		}
	}

	#[allow(dead_code)]
	fn read_to_string(&mut self) -> Option<String> {
		let mut buf = String::new();

		match self {
			Stream::Stdin(si) => {
				if si.read_to_string(&mut buf).is_err() {
					eprintln!("{}: invalid UTF-8", "-");
					return None;
				}
			}
			Stream::File(f, n) => {
				if f.read_to_string(&mut buf).is_err() {
					eprintln!("{}: invalid UTF-8", n);
					return None;
				}
			}
		};

		Some(buf)
	}
}

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
	let counts = count_words(content, cargs);

	counts
}

fn count_words(s: String, cargs: &Cli) -> Option<HashMap<Ustr, i32>> {
	static WORD_REGEX: Lazy<Regex> =
		Lazy::new(|| Regex::new(r"[a-zA-Z0-9]([a-zA-Z0-9]|'|-)*").unwrap());

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

	static WORD_REGEX: Lazy<Regex> =
		Lazy::new(|| Regex::new(r"[a-zA-Z0-9]([a-zA-Z0-9]|'|-)*").unwrap());

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
}
