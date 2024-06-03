use std::{
	fs::{self, read_to_string, File},
	io::{self, Read},
	path::PathBuf,
	process::exit,
};

use clap::Parser;

mod args;

fn main() {
	let cargs = args::Cli::parse(); // CLI arguments

	let files = cargs.files;

	if files.is_empty() {
		eprintln!("No files entered");
		exit(1);
	}

	let files: Vec<_> = files.iter().filter_map(|f| {
		if f == "-" {
			let mut buffer = String::new();
			io::stdin().read_to_string(&mut buffer);
			Some(buffer)
		} else {
			match fs::metadata(f) {
				Ok(meta) => {
					if meta.is_file() {
						let mut buffer = String::new();
						File::open(f).unwrap().read_to_string(&mut buffer);
						Some(buffer)
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
	}).collect();

	if files.is_empty() {
		exit(1);
	}

	for p in files {
		println!("{}", p);
	}
}
