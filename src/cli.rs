use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author)]
pub enum Cli {
    Encode {
        file: PathBuf,
        chunk_type: String,
        message: String,
        output: Option<PathBuf>,
    },
    Decode {
        file: PathBuf,
        chunk_type: String,
    },
    Remove {
        file: PathBuf,
        chunk_type: String,
    },
    Print {
        file: PathBuf,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
