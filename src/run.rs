use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use crate::chunk::{Chunk, ChunkType};
use crate::cli::{parse_args, Cli};
use crate::reader::{PngReader, ReaderError};
use crate::writer::PngWriter;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

    match args {
        Cli::Decode { file, chunk_type } => decode(file, chunk_type).map_err(|e| e.into()),
        Cli::Encode {
            file,
            chunk_type,
            message,
            output,
        } => encode(file, chunk_type, message, output),
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

fn encode(
    file: PathBuf,
    chunk_type: String,
    message: String,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reader = PngReader::new(BufReader::new(File::open(&file)?));

    let mut png = reader.read_png()?;
    let chunk_type: ChunkType = chunk_type.parse()?;
    let chunk = Chunk::new(chunk_type, message.into_bytes());
    png.append_chunk(chunk);

    let f = if let Some(output) = output {
        File::create(output)
    } else {
        File::create(file)
    }?;

    let mut writer = PngWriter::new(BufWriter::new(f));
    writer.write_png(png)?;

    Ok(())
}
