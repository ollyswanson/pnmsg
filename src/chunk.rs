use std::borrow::Cow;

#[derive(Debug)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns true if the chunk is critical
    pub fn is_critical(&self) -> bool {
        self.0[0] & 32 == 0
    }

    /// Returns true if the chunk is part of the PNG spec
    pub fn is_public(&self) -> bool {
        self.0[1] & 32 == 0
    }

    /// Returns true if the reserved bit is set. If the reserved bit is set then the chunk is
    /// invalid as of the current specification
    pub fn reserved_set(&self) -> bool {
        self.0[2] & 32 != 0
    }

    /// Returns true if the chunk is safe to copy, this is used when the chunk type is unknown.
    pub fn safe_to_copy(&self) -> bool {
        self.0[3] & 32 != 0
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum InvalidChunkType {
    #[error("reserved bit set")]
    Reserved,
    #[error("must be ascii alphabetic")]
    InvalidFormat,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = InvalidChunkType;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.iter().all(u8::is_ascii_alphabetic) {
            return Err(InvalidChunkType::InvalidFormat);
        }

        let chunk_type = ChunkType(value);

        if chunk_type.reserved_set() {
            return Err(InvalidChunkType::Reserved);
        }

        Ok(chunk_type)
    }
}

pub struct Chunk {
    /// Length of the chunk's data field, does not include the length of the other fields
    pub length: u32,
    pub type_: ChunkType,
    pub data: Vec<u8>,
    /// Cyclic redundancy check, calculated on the type and data fields, but not on the length
    pub crc: u32,
}

impl Chunk {
    /// Create a new chunk from a `ChunkType` and the data to be stored in the chunk, this method
    /// will calculate the cyclic redundancy check for the chunk
    pub fn new(type_: ChunkType, data: Vec<u8>) -> Self {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(type_.as_bytes());
        hasher.update(&data);
        let crc = hasher.finalize();

        Self {
            length: data.len() as u32,
            type_,
            data,
            crc,
        }
    }

    pub fn message(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_when_parsing_invalid_chunk() {
        let cases = [
            (b"Rust", InvalidChunkType::Reserved),
            (b"RuS7", InvalidChunkType::InvalidFormat),
        ];

        for (input, expected) in cases {
            assert_eq!(
                expected,
                TryInto::<ChunkType>::try_into(*input).unwrap_err()
            );
        }
    }

    #[test]
    fn new_chunk() {
        let chunk_type: ChunkType = TryFrom::try_from(*b"RuSt").unwrap();
        let data = b"This is where your secret message will be!".to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length, 42);
        assert_eq!(chunk.crc, 2882656334);
    }
}
