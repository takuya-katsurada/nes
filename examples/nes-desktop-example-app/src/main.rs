use clap::{Parser};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    /// Path of the iNES file to be set in the emulator
    #[clap(short = 'p', long = "path", value_name = "FILE")]
    file: String
}

fn main() {
    let cli = Cli::parse();

    let path = PathBuf::from(cli.file);
    let f = match fs::canonicalize(&path) {
        Ok(file) => file,
        Err(error) => panic!("{:?}", error)
    };
    println!("{:?}", f);
}
