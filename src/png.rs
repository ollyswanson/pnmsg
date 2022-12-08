use crate::chunk::Chunk;

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
}
