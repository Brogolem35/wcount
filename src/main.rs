mod args;
mod count;
mod stream;
mod result;
use std::process::exit;

use clap::Parser;
use count::*;
use stream::Stream;

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
		.filter_map(|s| StreamWordCount::from_stream(s))
		.collect();

	let total = TotalCount::from_counts(res.iter());

	for p in res {
		println!("{:?}: {:?}", p.from, p.counts);
	}

	println!("{:?}", total.counts);
}
