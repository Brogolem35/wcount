use std::{path::PathBuf, process::exit};

use clap::Parser;

mod args;

fn main() {
	let cargs = args::Cli::parse(); // CLI arguments

	let files = cargs.files;

	if files.is_empty() {
		eprintln!("No files entered");
		exit(1);
	}

	let paths: Vec<PathBuf> = files
		.iter()
		.map(|f| PathBuf::from(f))
		.filter(|f| {
			if !f.exists() {
				eprintln!("{}: No such file or directory", f.to_string_lossy());
				false
			} else if f.is_dir() {
				eprintln!("{}: Is a directory", f.to_string_lossy());
				false
			} else {
				f.is_file()
			}
		})
		.collect();

	if paths.is_empty() {
		exit(1);
	}

	for p in paths {
		println!("{}", p.to_string_lossy());
	}
}
