mod args;
mod count;
mod exclusions;
mod regexes;
mod stream;
mod warning;
use std::{io, process::exit};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use count::*;
use exclusions::Exclusions;
use stream::Stream;
use ustr::Ustr;
use warning::warning_printed;

enum Return {
	Ok = 0,
	Warning = 1,
	Error = 2,
}

fn main() {
	match run() {
		Ok(_) => match warning_printed() {
			true => exit(Return::Warning as i32),
			false => exit(Return::Ok as i32),
		},
		Err(_) => exit(Return::Error as i32),
	}
}

fn run() -> Result<()> {
	let cargs = args::Cli::parse(); // CLI arguments

	let files = &cargs.files;

	if files.is_empty() {
		return Result::Err(anyhow!("No files entered"));
	}

	let streams: Vec<_> = files.iter().filter_map(|f| Stream::from_str(f)).collect();

	if streams.is_empty() {
		return Result::Err(anyhow!("Args does not contain any valid files to process"));
	}

	if cargs.werror && warning_printed() {
		exit(Return::Warning as i32);
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
		.context("Could not output the result")?;

	if display_total {
		wtr.write_field(&cargs.total_label)
			.context("Could not output the result")?;
	}

	wtr.write_record(counts.iter().map(|r| r.label()))
		.context("Could not output the result")?;

	for (word, count) in words_to_print {
		wtr.write_field(word.as_str())
			.context("Could not output the result")?;

		if display_total {
			wtr.write_field(count.to_string())
				.context("Could not output the result")?;
		}

		wtr.write_record(counts.iter().map(|r| r.count(&word).to_string()))
			.context("Could not output the result")?;
	}

	wtr.flush().context("Could not output the result")?;

	Ok(())
}
