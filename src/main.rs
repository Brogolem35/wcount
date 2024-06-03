mod args;
use std::{
	fs::{self, read_to_string, File},
	io::{self, Read},
	path::PathBuf,
	process::exit,
};

use clap::Parser;

#[derive(Debug)]
enum Stream {
	Stdin(io::Stdin),
	File(File, String),
}

impl Stream {
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

	let files = cargs.files;

	if files.is_empty() {
		eprintln!("No files entered");
		exit(1);
	}

	let files: Vec<_> = files
		.iter()
		.filter_map(|f| {
			if f == "-" {
				Some(Stream::Stdin(io::stdin()))
			} else {
				match fs::metadata(f) {
					Ok(meta) => {
						if meta.is_file() {
							if let Ok(file) = File::open(f) {
								Some(Stream::File(
									file,
									f.to_string(),
								))
							} else {
								eprintln!("{}: error accessing", f);
								None
							}
						} else if meta.is_dir() {
							eprintln!("{}: Is a directory", f);
							None
						} else {
							eprintln!("{}: error accessing", f);
							None
						}
					}
					Err(e) => {
						eprintln!("{}: error accessing", f);
						None
					}
				}
			}
		})
		.collect();

	if files.is_empty() {
		exit(1);
	}

	for p in files {
		println!("{:?}", p);
	}
}
