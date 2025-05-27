mod args;
mod count;
mod exclusions;
mod regexes;
mod stream;
mod warning;
use std::fmt::Write;
use std::{io, process::ExitCode};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use count::*;
use exclusions::Exclusions;
use stream::Stream;
use ustr::Ustr;
use warning::warning_printed;

enum Return {
	Ok = 0,
	Error = 1,
	Warning = 2,
}

fn main() -> ExitCode {
	match run() {
		Ok(_) => match warning_printed() {
			true => ExitCode::from(Return::Warning as u8),
			false => ExitCode::from(Return::Ok as u8),
		},
		Err(e) => {
			eprintln!("{:#}", e);
			ExitCode::from(Return::Error as u8)
		}
	}
}

fn run() -> Result<()> {
	let args = args::Cli::try_parse()?; // CLI arguments

	let files = &args.files;

	if files.is_empty() {
		return Err(anyhow!("No files entered"));
	}

	let counts: Vec<_> = files
		.iter()
		.filter_map(|f| Stream::from_str(f))
		.filter_map(|s| {
			StreamWordCount::from_stream(
				s,
				args.pattern.to_regex(),
				args.case_sensitive,
			)
		})
		.collect();

	if args.werror && warning_printed() {
		return Err(anyhow!("--werror: Processes stopped early due to warnings"));
	}

	if counts.is_empty() {
		return Err(anyhow!("Args does not contain any valid files to process"));
	}

	let total = TotalCount::from_counts(counts.iter());

	let display_total = args.display_total.should_display(counts.len());

	let mut total_counts = total.to_ordered_vec();

	if args.reverse {
		total_counts.reverse();
	}

	let words_to_print: Vec<(Ustr, usize)> = if args.row_count == 0 {
		total_counts.iter().map(|(s, i)| (*s, *i)).collect()
	} else {
		total_counts
			.iter()
			.map(|(s, i)| (*s, *i))
			.take(args.row_count)
			.collect()
	};

	let words_to_print: Vec<_> = if let Some(s) = args.excluded_words {
		let mut exclude_stream =
			Stream::from_str(&s).context("Can't read --excluded-words file")?;

		let exclusions = Exclusions::from_stream(&mut exclude_stream)
			.context("Can't read --excluded-words file")?;

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
		wtr.write_field(&args.total_label)
			.context("Could not output the result")?;
	}

	wtr.write_record(counts.iter().map(|r| r.label()))
		.context("Could not output the result")?;

	// TODO: Document and clean up
	let mut record_buf = String::new();
	for (word, count) in words_to_print {
		wtr.write_field(word.as_str())
			.context("Could not output the result")?;

		if display_total {
			wtr.write_field({
				record_buf.clear();
				write!(&mut record_buf, "{}", count)?;

				&record_buf
			})
			.context("Could not output the result")?;
		}

		for c in counts.iter() {
			wtr.write_field({
				record_buf.clear();
				let _ = write!(&mut record_buf, "{}", c.count(&word));

				&record_buf
			})
			.context("Could not output the result")?;
		}
		wtr.write_record(None::<&[u8]>)
			.context("Could not output the result")?;
	}

	wtr.flush().context("Could not output the result")?;

	Ok(())
}
