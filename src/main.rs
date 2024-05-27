use std::path::PathBuf;

use clap::Parser;

mod args;

fn main() {
	let cargs = args::Cli::parse(); // CLI arguments

	let files = cargs.files;

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

	for p in paths {
		println!("{}", p.to_string_lossy());
	}
}
