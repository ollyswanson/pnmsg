use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

pub fn encode(
    file: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(&file)?;
    let mut reader = BufReader::new(f);

    let mut png = Png::read_to_png(&mut reader)?;
    let new_chunk_type = ChunkType::try_from(chunk_type.as_bytes())?;
    let new_chunk = Chunk::new(new_chunk_type, message.into_bytes());
    png.append_chunk(new_chunk);

    match output_file {
        Some(file) => File::create(file)?.write_all(&png.as_bytes())?,
        None => File::create(file)?.write_all(&png.as_bytes())?,
    }

    Ok(())
}

pub fn decode(file: PathBuf, chunk_type: String) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(&file)?;
    let mut reader = BufReader::new(f);

    let png = Png::read_to_png(&mut reader)?;
    if let Some(message) = png.chunk_by_type(&chunk_type) {
        println!("{}", message.data_as_string()?);
    } else {
        println!("No message found!");
    }

    Ok(())
}
