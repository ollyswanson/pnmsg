use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::cli::{parse_args, Cli};
use crate::reader::{PngReader, ReaderError};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

    match args {
        Cli::Decode { file, chunk_type } => decode(file, chunk_type).map_err(|e| e.into()),
        _ => todo!(),
    }
}

fn decode(file: PathBuf, chunk_type: String) -> Result<(), ReaderError> {
    let reader = PngReader::new(BufReader::new(File::open(&file)?));
    let png = reader.read_png()?;

    if let Some(chunk) = png.chunk_by_type(&chunk_type) {
        println!("Message: {}", chunk.message());
    }

    Ok(())
}
