use std::io::{self, BufWriter, Write};

use crate::chunk::Chunk;
use crate::png::Png;

pub struct PngWriter<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> PngWriter<W> {
    pub fn new(writer: BufWriter<W>) -> Self {
        Self { writer }
    }

    fn write_chunk(&mut self, chunk: Chunk) -> Result<(), io::Error> {
        self.writer.write_all(&chunk.length.to_be_bytes())?;
        self.writer.write_all(chunk.type_.as_bytes())?;
        self.writer.write_all(&chunk.data)?;
        self.writer.write_all(&chunk.crc.to_be_bytes())
    }

    pub fn write_png(&mut self, png: Png) -> Result<(), io::Error> {
        self.writer.write_all(&Png::HEADER)?;

        for chunk in png.into_chunks() {
            self.write_chunk(chunk)?;
        }

        Ok(())
    }
}
