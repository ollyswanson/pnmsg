use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::Context;

use crate::chunk::{Chunk, ChunkType};
use crate::cli::{parse_args, Cli};
use crate::reader::PngReader;
use crate::writer::PngWriter;

pub fn run() -> anyhow::Result<()> {
    let args = parse_args();

    match args {
        Cli::Decode { file, chunk_type } => decode(file, chunk_type),
        Cli::Encode {
            file,
            chunk_type,
            message,
            output,
        } => encode(file, chunk_type, message, output),
        Cli::Remove { file, chunk_type } => remove(file, chunk_type),
        Cli::Print { file } => print(file),
    }
}

static OPEN_FAILED: &str = "Failed to open PNG";
static DECODE_FAILED: &str = "Failed to decode PNG";
static WRITE_FAILED: &str = "Failed to write PNG";
static INVALID_CHUNK_TYPE: &str = "Invalid chunk type";

fn decode(file: PathBuf, chunk_type: String) -> anyhow::Result<()> {
    let reader = PngReader::new(BufReader::new(File::open(&file).context(OPEN_FAILED)?));
    let png = reader.read_png().context(DECODE_FAILED)?;

    println!("Messages in file:\n");
    for chunk in png.chunks_by_type(&chunk_type) {
        println!("{}", chunk.message());
    }

    Ok(())
}

fn encode(
    file: PathBuf,
    chunk_type: String,
    message: String,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let reader = PngReader::new(BufReader::new(File::open(&file).context(OPEN_FAILED)?));
    let mut png = reader.read_png().context(DECODE_FAILED)?;

    let chunk_type: ChunkType = chunk_type.parse().context(INVALID_CHUNK_TYPE)?;
    let chunk = Chunk::new(chunk_type, message.into_bytes());
    png.append_chunk(chunk);

    let f = if let Some(output) = output {
        File::create(output)
    } else {
        File::create(file)
    }
    .context(WRITE_FAILED)?;
    let mut writer = PngWriter::new(BufWriter::new(f));
    writer.write_png(png).context(WRITE_FAILED)?;

    println!("Successfully hid the message!");

    Ok(())
}

fn remove(file: PathBuf, chunk_type: String) -> anyhow::Result<()> {
    let reader = PngReader::new(BufReader::new(File::open(&file).context(OPEN_FAILED)?));
    let mut png = reader.read_png().context(DECODE_FAILED)?;

    let removed = png.remove_all(&chunk_type);

    let f = File::create(file).context(WRITE_FAILED)?;
    let mut writer = PngWriter::new(BufWriter::new(f));
    writer.write_png(png).context(WRITE_FAILED)?;

    println!("Removed {} chunks", removed);

    Ok(())
}

fn print(file: PathBuf) -> anyhow::Result<()> {
    let reader = PngReader::new(BufReader::new(File::open(&file).context(OPEN_FAILED)?));
    let png = reader.read_png().context(DECODE_FAILED)?;

    println!("Printing all chunk types in file...\n");
    for chunk_type in png.chunk_types() {
        println!("{}", chunk_type);
    }

    Ok(())
}
