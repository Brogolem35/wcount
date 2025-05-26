use std::{
	fs::{self, File},
	io::{self, Read},
};

use crate::wprintln;

/// Represents a stream of string, either from a file or from Stdin.
#[derive(Debug)]
pub enum Stream {
	/// Represents standard input.
	Stdin(io::Stdin),
	/// Represents a file.
	///
	/// First element is a `File`, the second is the path to the file.
	File(File, String),
}

impl Stream {
	/// Creates a `Stream` from the given string.
	///
	/// If the string is equal to `-`, then it is Stdin. If not, then it will be considered as a path to a file.
	pub fn from_str(path: &str) -> Option<Stream> {
		if path == "-" {
			return Some(Stream::Stdin(io::stdin()));
		}

		let meta = match fs::metadata(path) {
			Ok(meta) => meta,
			Err(e) => {
				wprintln!("{}: {}", path, e);
				return None;
			}
		};

		if meta.is_file() {
			if let Ok(file) = File::open(path) {
				Some(Stream::File(file, path.to_string()))
			} else {
				wprintln!("{}: Error accessing", path);
				None
			}
		} else if meta.is_dir() {
			wprintln!("{}: Is a directory", path);
			None
		} else {
			wprintln!("{}: Error accessing", path);
			None
		}
	}

	/// Reads the `Stream` and returns its contents as a `String`.
	///
	/// Can't read invalid UTF-8 content.
	pub fn read_to_string(&mut self) -> Option<String> {
		let mut buf = String::new();

		match self {
			Stream::Stdin(si) => {
				if si.read_to_string(&mut buf).is_err() {
					wprintln!("-: invalid UTF-8");
					return None;
				}
			}
			Stream::File(f, n) => {
				if f.read_to_string(&mut buf).is_err() {
					wprintln!("{}: invalid UTF-8", n);
					return None;
				}
			}
		};

		Some(buf)
	}

	/// Returns the label of the `Stream`.
	///
	/// Label is `standard_input` for Stdin, and the path of file for the File.
	pub fn label(&self) -> String {
		match self {
			Self::Stdin(_) => String::from("standard_input"),
			Self::File(_, s) => s.clone(),
		}
	}
}
