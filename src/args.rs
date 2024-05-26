use clap::{Args, Parser};

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
