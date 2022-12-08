use crate::chunk::Chunk;

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_chunk<T: AsRef<[u8]>>(&mut self, type_: T) {
        // Search from the back as we are usually looking for a chunk that has been added at the
        // end
        if let Some(pos) = self
            .chunks
            .iter()
            .rev()
            .position(|chunk| chunk.type_.as_bytes() == type_.as_ref())
            .map(|pos| self.chunks.len() - pos - 1)
        {
            self.chunks.remove(pos);
        }
    }

    pub fn chunk_by_type<T: AsRef<[u8]>>(&self, type_: T) -> Option<&Chunk> {
        // Search from the back as we are usually looking for a chunk that has been added at the
        // end.
        self.chunks
            .iter()
            .rev()
            .find(|chunk| chunk.type_.as_bytes() == type_.as_ref())
    }
}
