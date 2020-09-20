use crate::chunk_type::ChunkType;
use crc::crc32;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io::{BufRead, ErrorKind};
use std::str;

// A chunk is comprised of 3 4-byte segments + the data
// data_len + chunk_type + data + crc
pub struct Chunk {
    chunk: Vec<u8>,
}

impl Chunk {
    // Construct a new Chunk from a given ChunkType and data to be placed in the chunk.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data_len = data.len();

        let mut chunk = Vec::from((data_len as u32).to_be_bytes());
        chunk.extend(chunk_type.bytes());
        chunk.extend(data);

        // crc is calculated from the bytes comprising the chunk_type and the data
        let crc = crc32::checksum_ieee(&chunk[4..]);

        for byte in crc.to_be_bytes().iter() {
            chunk.push(*byte);
        }

        Chunk { chunk }
    }

    // returns the length of the data segment of the chunk, does not include the 12 bytes dedicated
    // to metadata
    pub fn data_len(&self) -> u32 {
        self.chunk.len() as u32 - 12
    }

    pub fn chunk_type(&self) -> ChunkType {
        // safe to unwrap here as a Chunk should not be constructable in a way that this could fail
        ChunkType::try_from(&self.chunk[4..8]).unwrap()
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk[8..(self.chunk.len() - 4)]
    }

    pub fn data_as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(str::from_utf8(self.data())?.to_string())
    }

    pub fn crc(&self) -> u32 {
        // unwrapped here as well as Chunk should not be constructed in a way that this will fail
        u32::from_be_bytes(self.chunk[self.chunk.len() - 4..].try_into().unwrap())
    }

    // takes ownership of self and returns the chunk out as a vec for writing to disk
    pub fn as_bytes(self) -> Vec<u8> {
        self.chunk
    }

    // TODO: check that this function is ok to use, it's probably not particularly robust,
    // especially using the slice in the last call to read_exact
    pub fn read_to_chunk<R: BufRead>(
        r: &mut R,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let mut data_len: [u8; 4] = [0; 4];
        match r.read_exact(&mut data_len) {
            Ok(_) => {}
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => Err(e)?,
        };
        let chunk_len = u32::from_be_bytes(data_len.clone());

        // Really lazily being careful to not allocate too much memory if something goes wrong when
        // reading the chunk, 10MB seems a reasonable check as most pngs are smaller than that...
        if chunk_len > 1024 * 1024 * 10 {
            Err("Chunk size is to large")?;
        }

        let mut rest_of_chunk: Vec<u8> = vec![0; chunk_len as usize + 8];

        r.read_exact(&mut rest_of_chunk)?;

        let mut chunk: Vec<u8> = Vec::from(data_len);
        chunk.extend(rest_of_chunk);

        // Successfully creating a chunk type verifies its validity
        let _chunk_type = match ChunkType::try_from(&chunk[4..8]) {
            Ok(c) => c,
            Err(e) => Err(e)?,
        };

        // Check that calculated crc matches crc that was read
        if crc32::checksum_ieee(&chunk[4..chunk.len() - 4])
            == u32::from_be_bytes((&chunk[chunk.len() - 4..]).try_into()?)
        {
            Ok(Some(Chunk { chunk }))
        } else {
            Err("Calculated crc did not match provided crc that was read")?
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Data length: {} bytes\nChunk type: {}\nCrc: {}",
            self.data_len(),
            self.chunk_type().to_string(),
            self.crc()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::io::BufReader;

    fn testing_chunk() -> Chunk {
        let data_len: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_len
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let mut reader = BufReader::new(&chunk_data[..]);
        Chunk::read_to_chunk(&mut reader).unwrap().unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.data_len(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_len: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_len
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let mut reader = BufReader::new(&chunk_data[..]);
        let chunk = Chunk::read_to_chunk(&mut reader).unwrap().unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.data_len(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_len: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_len
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let mut reader = BufReader::new(&chunk_data[..]);
        let chunk = Chunk::read_to_chunk(&mut reader);

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_len: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_len
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let mut reader = BufReader::new(&chunk_data[..]);
        let chunk = Chunk::read_to_chunk(&mut reader).unwrap().unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
