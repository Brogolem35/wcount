use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TotalColumn {
	Enabled,
	Disabled,
	Force,
}

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
	/// Files that the words will be counted from
	pub files: Vec<String>,

	/// Custom label for the `total_count` column
	#[arg(short, long, default_value_t = {"total_count".to_string()}, value_name = "COLUMN_LABEL")]
	pub total: String,

	/// Control the `total_count` column output
	#[arg(long, value_enum, default_value_t = {TotalColumn::Enabled})]
	pub total_column: TotalColumn,
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
		assert_eq!(cli.total, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
	}

	#[test]
	fn no_files() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert!(cli.files.is_empty());
		assert_eq!(cli.total, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
	}

	#[test]
	fn custom_total_label() {
		let cmd = Cli::command();
		let matches = cmd.get_matches_from(vec![
			"wcount", // executable name
			"file1.txt",
			"file2.txt",
			"--total",
			"custom_label",
		]);

		let cli = Cli::from_arg_matches(&matches).unwrap();

		assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
		assert_eq!(cli.total, "custom_label");
		assert!(matches!(cli.total_column, TotalColumn::Enabled));
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
		assert_eq!(cli.total, "total_count");
		assert!(matches!(cli.total_column, TotalColumn::Disabled));
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
		assert!(matches!(cli.total_column, TotalColumn::Force));
	}
}
