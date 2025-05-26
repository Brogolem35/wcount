mod args;
mod count;
mod exclusions;
mod regexes;
mod stream;
mod warning;
use std::{io, process::exit};

use clap::Parser;
use count::*;
use exclusions::Exclusions;
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
		wprintln!("No files entered");
		exit(Return::Error as i32);
	}

	let streams: Vec<_> = files.iter().filter_map(|f| Stream::from_str(f)).collect();

	if streams.is_empty() {
		wprintln!("Args does not contain any valid files to process");
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

	let display_total = cargs.display_total.should_display(counts.len());

	let mut total_counts = total.to_ordered_vec();

	if cargs.reverse {
		total_counts.reverse();
	}

	let words_to_print: Vec<(Ustr, usize)> = if cargs.row_count == 0 {
		total_counts.iter().map(|(s, i)| (*s, *i)).collect()
	} else {
		total_counts
			.iter()
			.map(|(s, i)| (*s, *i))
			.take(cargs.row_count)
			.collect()
	};

	let words_to_print: Vec<_> = if let Some(s) = cargs.excluded_words {
		let exclude_stream = Stream::from_str(&s);

		let exclusions = if let Some(mut s) = exclude_stream {
			Exclusions::from_stream(&mut s).expect("Can't read --excluded-words file")
		} else {
			exit(Return::Error as i32);
		};

		words_to_print
			.into_iter()
			.filter(|(s, _)| !exclusions.contains(s))
			.collect()
	} else {
		words_to_print
	};

	let mut wtr = csv::Writer::from_writer(io::stdout());
	wtr.write_field("word")
		.expect("Could not output the result");

	if display_total {
		wtr.write_field(&cargs.total_label)
			.expect("Could not output the result");
	}

	wtr.write_record(counts.iter().map(|r| r.label()))
		.expect("Could not output the result");

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
