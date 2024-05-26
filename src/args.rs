use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    /// Files that the words will be counted from.
    pub files: Vec<String>,

    /// Custom label for the `total_count` column.
    #[arg(short, long, default_value_t = {"total_count".to_string()}, value_name = "COLUMN_LABEL")]
    pub total: String,

    /// Disables the `total_count` column.
    #[arg(long)]
    pub no_total: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, FromArgMatches};

    #[test]
    fn test_default_total() {
        let cmd = Cli::command();
        let matches = cmd.get_matches_from(vec![
            "wcount", // executable name
            "file1.txt",
            "file2.txt",
        ]);

        let cli = Cli::from_arg_matches(&matches).unwrap();

        assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
        assert_eq!(cli.total, "total_count");
        assert!(!cli.no_total);
    }

    #[test]
    fn test_custom_total() {
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
        assert!(!cli.no_total);
    }

    #[test]
    fn test_no_total() {
        let cmd = Cli::command();
        let matches = cmd.get_matches_from(vec![
            "wcount", // executable name
            "file1.txt",
            "file2.txt",
            "--no-total",
        ]);

        let cli = Cli::from_arg_matches(&matches).unwrap();

        assert_eq!(cli.files, vec!["file1.txt", "file2.txt"]);
        assert_eq!(cli.total, "total_count");
        assert!(cli.no_total);
    }
}
