use std::convert::TryFrom;
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct ChunkType<'a> {
    bytes: &'a [u8],
}
// Bit 5 of each byte (ASCII uppercase / lowercase) encodes the information for the chunk

// Uppercase -> Critical, Lowercase -> Ancillary
impl<'a> ChunkType<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ChunkType { bytes: &bytes }
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    // Uppercase -> Public, Lowercase -> Private
    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    // Uppercase -> Valid, Lowercase(is set) -> Invalid
    pub fn is_reserved_set(&self) -> bool {
        self.bytes[2].is_ascii_lowercase()
    }

    // Uppercase -> Unsafe to copy (if modifications to critical chunks), Lowercase -> Safe
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }

    // Must be 4 bytes in length, all ascii_alphabetic, and reserved bit must NOT be set.
    pub fn is_valid(&self) -> bool {
        self.bytes.len() == 4
            && self.bytes.iter().all(|byte| byte.is_ascii_alphabetic())
            && !self.is_reserved_set()
    }

    pub fn bytes(&self) -> Vec<u8> {
        Vec::from(self.bytes)
    }
}

impl<'a> TryFrom<&'a [u8]> for ChunkType<'a> {
    type Error = &'static str;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType::new(&bytes);

        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            Err("Invalid chunk type")
        }
    }
}

impl<'a> fmt::Display for ChunkType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.bytes).unwrap())
    }
}

impl<'a, 'b> PartialEq<&'b str> for ChunkType<'a> {
    fn eq(&self, other: &&'b str) -> bool {
        self.bytes == other.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn chunk_type_is_critical() {
        let chunk_type = ChunkType::try_from("RuSt".as_bytes()).unwrap();
        assert!(chunk_type.is_critical());
    }

    #[test]
    fn chunk_type_is_not_critical() {
        let chunk_type = ChunkType::try_from("ruSt".as_bytes()).unwrap();
        assert!(!chunk_type.is_critical());
    }

    #[test]
    fn chunk_type_is_public() {
        let chunk_type = ChunkType::try_from("RUSt".as_bytes()).unwrap();
        assert!(chunk_type.is_public());
    }

    #[test]
    fn chunk_type_is_private() {
        let chunk_type = ChunkType::try_from("RuSt".as_bytes()).unwrap();
        assert!(!chunk_type.is_public());
    }

    #[test]
    fn chunk_type_is_safe_to_copy() {
        let chunk_type = ChunkType::try_from("RuSt".as_bytes()).unwrap();
        assert!(chunk_type.is_safe_to_copy());
    }

    #[test]
    fn chunk_type_is_unsafe_to_copy() {
        let chunk_type = ChunkType::try_from("RuST".as_bytes()).unwrap();
        assert!(!chunk_type.is_safe_to_copy());
    }

    #[test]
    fn chunk_type_is_valid() {
        let chunk_type = ChunkType::try_from("RuST".as_bytes()).unwrap();
        assert!(chunk_type.is_valid());
    }

    #[test]
    fn chunk_type_is_invalid() {
        // Invalid due to "s" having reserved bit set
        let chunk_type1 = ChunkType::try_from("Rust".as_bytes());
        assert!(chunk_type1.is_err());

        // Invalid due to numeric character
        let chunk_type2 = ChunkType::try_from("Ru1t".as_bytes());
        assert!(chunk_type2.is_err());
    }

    #[test]
    fn chunk_type_partial_eq_str() {
        let chunk_type = ChunkType::try_from("RuST".as_bytes()).unwrap();
        assert_eq!(chunk_type, "RuST");
    }
}
