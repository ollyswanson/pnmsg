use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author)]
pub enum Cli {
    Encode {
        file: PathBuf,
        chunk_type: String,
        message: String,
    },
    Decode {
        file: PathBuf,
        chunk_type: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
