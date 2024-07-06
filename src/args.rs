use clap::{Parser, ValueEnum};
use regex::Regex;

use crate::regexes::{
	ALL_REGEX, ALPHANUMERIC_REGEX, ALPHA_REGEX, NOAPOSTROPHE_REGEX, NODASH_REGEX, NUMERIC_REGEX,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TotalColumn {
	Enabled,
	Disabled,
	Force,
}

impl TotalColumn {
	pub fn should_display(&self, count: usize) -> bool {
		match self {
			TotalColumn::Enabled => {
				if count > 1 {
					true
				} else {
					false
				}
			}
			TotalColumn::Disabled => false,
			TotalColumn::Force => true,
		}
	}
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum WordRegex {
	All,
	OnlyAlphanumeric,
	OnlyAlphabetic,
	OnlyNumeric,
	NoDash,
	NoApostrophe,
}

impl WordRegex {
	pub fn to_regex(&self) -> &'static Regex {
		match self {
			WordRegex::All => &ALL_REGEX,
			WordRegex::OnlyAlphanumeric => &ALPHANUMERIC_REGEX,
			WordRegex::OnlyAlphabetic => &ALPHA_REGEX,
			WordRegex::OnlyNumeric => &NUMERIC_REGEX,
			WordRegex::NoDash => &NODASH_REGEX,
			WordRegex::NoApostrophe => &NOAPOSTROPHE_REGEX,
		}
	}
}

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
	/// Files that the words will be counted from
	pub files: Vec<String>,

	/// Custom label for the `total_count` column
	#[arg(short, long, default_value_t = {"total_count".to_string()}, value_name = "COLUMN_LABEL")]
	pub total_label: String,

	/// Control the `total_count` column output
	#[arg(long, value_enum, default_value_t = {TotalColumn::Enabled})]
	pub total_column: TotalColumn,

	/// Number of rows of words and their counts to be displayed, unlimited for 0
	#[arg(long, default_value_t = 50, value_name = "ROW_COUNT")]
	pub row_count: usize,

	/// Case sensitivity, results will be displayed in lower case if false
	#[arg(short, long, default_value_t = false)]
	pub case_sensitive: bool,

	/// Pattern to match for words
	#[arg(short, long, value_enum ,default_value_t = {WordRegex::All})]
	pub pattern: WordRegex,

	/// Show the results in ascending order, instead of descending
	#[arg(short, long, default_value_t = false)]
	pub reverse: bool,

	/// Close the process at any warning
	#[arg(short = 'W', long)]
	pub werror: bool,
}

#[cfg(test)]
mod tests {
	use std::vec;

	use super::*;
	use clap::{CommandFactory, FromArgMatches};

	#[test]
	fn default() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn no_files() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert!(cli.files.is_empty());
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn custom_total_label() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--total-label",
			"custom_label",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "custom_label");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn disabled_total() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--total-column=disabled",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Disabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn force_total() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--total-column=force",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Force));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn werror() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--werror",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn row_count1() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--row-count",
			"75",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 75);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn row_count2() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--row-count",
			"250",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 250);
		assert!(!cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn case_sensitive() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--case-sensitive",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(cli.case_sensitive);
		assert!(!cli.reverse);
	}

	#[test]
	fn reverse() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--reverse",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total_label, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
		assert!(!cli.werror);
		assert_eq!(cli.row_count, 50);
		assert!(!cli.case_sensitive);
		assert!(cli.reverse);
	}
}
