use std::{
	fs::{self, File},
	io::{self, Read},
};

#[derive(Debug)]
pub enum Stream {
	Stdin(io::Stdin),
	File(File, String),
}

impl Stream {
	pub fn from_str(path: &str) -> Option<Stream> {
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

	pub fn read_to_string(&mut self) -> Option<String> {
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

	pub fn label(&self) -> String {
		match self {
			Self::Stdin(_) => String::from("standard_input"),
			Self::File(_, s) => s.clone(),
		}
	}
}
