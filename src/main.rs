use clap::Parser;

mod args;

fn main() {
    let cargs = args::Cli::parse(); // CLI arguments

    let total = cargs.total;
    let no_total = cargs.no_total;
    let files = cargs.files;

    for f in files {
        println!("{f}");
    }

    println!("{total}");
    println!("{no_total}");
}
