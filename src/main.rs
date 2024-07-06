mod args;
mod count;
mod regexes;
mod stream;
use std::{io, process::exit};

use clap::Parser;
use count::*;
use regexes::ALL_REGEX;
use stream::Stream;
use ustr::Ustr;

enum Return {
	Ok = 0,
	Warning = 1,
	Error = 2,
}

fn main() {
	let mut result = Return::Ok;

	let cargs = args::Cli::parse(); // CLI arguments

	let files = &cargs.files;

	if files.is_empty() {
		eprintln!("No files entered");
		exit(Return::Error as i32);
	}

	let streams: Vec<_> = files.iter().filter_map(|f| Stream::from_str(f)).collect();

	if streams.is_empty() {
		eprintln!("Args does not contain any valid files to process");
		exit(Return::Error as i32);
	}

	if files.len() != streams.len() {
		result = Return::Warning;

		if cargs.werror {
			exit(Return::Warning as i32);
		}
	}

	let counts: Vec<_> = streams
		.into_iter()
		.filter_map(|s| {
			StreamWordCount::from_stream(
				s,
				cargs.pattern.to_regex(),
				cargs.case_sensitive,
			)
		})
		.collect();

	let total = TotalCount::from_counts(counts.iter());

	let display_total = cargs.total_column.should_display(counts.len());

	let mut wtr = csv::Writer::from_writer(io::stdout());
	wtr.write_field("word")
		.expect("Could not output the result");

	if display_total {
		wtr.write_field(&cargs.total_label)
			.expect("Could not output the result");
	}

	wtr.write_record(counts.iter().map(|r| r.label()))
		.expect("Could not output the result");

	let mut total_counts = total.to_ordered_vec();

	if cargs.reverse {
		total_counts.reverse();
	}

	let words_to_print: Vec<(Ustr, usize)> = if cargs.row_count == 0 {
		total_counts
			.iter()
			.map(|(s, i)| (s.clone(), i.clone()))
			.collect()
	} else {
		total_counts
			.iter()
			.map(|(s, i)| (s.clone(), i.clone()))
			.take(cargs.row_count)
			.collect()
	};

	for (word, count) in words_to_print {
		wtr.write_field(word.as_str())
			.expect("Could not output the result");

		if display_total {
			wtr.write_field(count.to_string())
				.expect("Could not output the result");
		}

		wtr.write_record(counts.iter().map(|r| r.count(&word).to_string()))
			.expect("Could not output the result");
	}

	wtr.flush().expect("Could not output the result");

	exit(result as i32);
}
