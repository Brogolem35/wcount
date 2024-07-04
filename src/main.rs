mod args;
mod count;
mod regexes;
mod result;
mod stream;
use std::{io, process::exit};

use clap::Parser;
use count::*;
use regexes::WORD_REGEX;
use result::ResultItem;
use stream::Stream;
use ustr::Ustr;

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

	let counts: Vec<_> = streams
		.into_iter()
		.filter_map(|s| StreamWordCount::from_stream(s, &WORD_REGEX))
		.collect();

	let total = TotalCount::from_counts(counts.iter());
	let mut scounts: Vec<_> = counts.into_iter().map(|s| ResultItem::Stream(s)).collect();

	let mut res = vec![ResultItem::Total(total.clone())];
	res.append(&mut scounts);

	let mut wtr = csv::Writer::from_writer(io::stdout());
	wtr.write_field("word")
		.expect("Could not output the result");
	wtr.write_record(res.iter().map(|r| r.label()))
		.expect("Could not output the result");

	let words_to_print: Vec<Ustr> = if cargs.row_count == 0 {
		total.to_ordered_vec()
			.iter()
			.map(|(s, _)| s.clone())
			.collect()
	} else {
		total.to_ordered_vec()
			.iter()
			.map(|(s, _)| s.clone())
			.take(cargs.row_count)
			.collect()
	};

	for word in words_to_print {
		wtr.write_field(word.as_str())
			.expect("Could not output the result");
		wtr.write_record(res.iter().map(|r| r.count(&word).to_string()))
			.expect("Could not output the result");
	}

	wtr.flush().expect("Could not output the result");
}
