use std::cmp;
use std::io::{self, BufRead, BufReader, Read};

use crate::chunk::{Chunk, ChunkType, FormatError};
use crate::png::Png;

pub struct PngReader<R: Read> {
    reader: BufReader<R>,
    eof: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum ReaderError {
    #[error("error reading file")]
    Io(#[from] io::Error),
    #[error("error reading chunk")]
    FormatError(#[from] FormatError),
}

impl<R: Read> PngReader<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Self { reader, eof: false }
    }

    fn next_chunk(&mut self) -> Result<Chunk, ReaderError> {
        let mut length = [0; 4];
        self.reader.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);

        let mut chunk_type = [0; 4];
        self.reader.read_exact(&mut chunk_type)?;
        let chunk_type: ChunkType = TryFrom::try_from(chunk_type)?;

        let mut left_to_read = length as usize;
        let mut data = Vec::new();

        while left_to_read > 0 {
            let buf = self.reader.fill_buf()?;
            if buf.is_empty() {
                return Err(FormatError::UnexpectedEof.into());
            }
            let to_read = cmp::min(left_to_read, buf.len());
            data.extend_from_slice(&buf[..to_read]);
            self.reader.consume(to_read);
            left_to_read -= to_read;
        }

        let mut crc = [0; 4];
        self.reader.read_exact(&mut crc)?;
        let crc = u32::from_be_bytes(crc);

        if self.reader.fill_buf()?.is_empty() {
            self.eof = true;
        }

        Ok(Chunk {
            length,
            type_: chunk_type,
            data,
            crc,
        })
    }

    pub fn read_png(mut self) -> Result<Png, ReaderError> {
        let mut header = [0; 8];
        self.reader.read_exact(&mut header)?;
        if header != Png::HEADER {
            return Err(ReaderError::FormatError(FormatError::InvalidHeader));
        }

        let mut chunks = Vec::new();
        while !self.eof {
            chunks.push(self.next_chunk()?);
        }

        Ok(Png::from_chunks(chunks))
    }
}
