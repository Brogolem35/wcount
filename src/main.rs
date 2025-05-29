mod args;
mod count;
mod exclusions;
mod regexes;
mod stream;
mod warning;
use std::fmt::Write;
use std::io::BufWriter;
use std::{io, process::ExitCode};

use anyhow::{anyhow, Context, Result};
use args::WordRegex;
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

	let counts = get_counts(files, args.pattern, args.case_sensitive, args.werror)?;

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

	output_csv(counts, words_to_print, display_total, &args.total_label)?;

	Ok(())
}

fn get_counts(
	files: &Vec<String>,
	pattern: WordRegex,
	case_sensitive: bool,
	werror: bool,
) -> Result<Vec<StreamWordCount>> {
	let counts: Vec<_> = files
		.iter()
		.filter_map(|f| Stream::from_str(f))
		.filter_map(|s| StreamWordCount::from_stream(s, pattern.to_regex(), case_sensitive))
		.collect();

	if werror && warning_printed() {
		return Err(anyhow!("--werror: Processes stopped early due to warnings"));
	}

	Ok(counts)
}

fn output_csv(
	counts: Vec<StreamWordCount>,
	words_to_print: Vec<(Ustr, usize)>,
	display_total: bool,
	total_label: &str,
) -> Result<()> {
	// Using two buffers.
	// `out_buf` is used to avoid unnecessary allocations caused by the `to_string`
	// method of the numeric types. The line is formatted here before pushing to writer.
	let mut out_buf = String::new();
	// Buffered writer to stdout. Much faster than printing to stdout directly.
	let mut writer = BufWriter::new(io::stdout().lock());

	write!(&mut out_buf, "word,")?;

	if display_total {
		write!(&mut out_buf, "{},", total_label)?;
	}

	for label in counts.iter().map(|r| r.label()) {
		write!(&mut out_buf, "{},", label)?;
	}
	// Last ',' is redundant.
	out_buf.pop().context("No ',' at the end")?;
	out_buf.push('\n');

	io::Write::write(&mut writer, out_buf.as_bytes())?;
	for (word, count) in words_to_print {
		out_buf.clear();

		write!(&mut out_buf, "{},", word.as_str())?;

		if display_total {
			write!(&mut out_buf, "{},", count)?;
		}

		// Can't just use `write_record`, as the closures didn't play well with the buffer.
		for c in counts.iter() {
			write!(&mut out_buf, "{},", c.count(&word))?;
		}
		out_buf.pop().context("No ',' at the end")?;
		out_buf.push('\n');

		io::Write::write(&mut writer, out_buf.as_bytes())?;
	}

	Ok(())
}
