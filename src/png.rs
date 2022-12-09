use std::borrow::Cow;

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

    /// Removes all chunks matching the given type and returns the number of chunks removed
    pub fn remove_all<T: AsRef<[u8]>>(&mut self, type_: T) -> usize {
        let len = self.chunks.len();
        self.chunks
            .retain(|chunk| chunk.type_.as_bytes() != type_.as_ref());
        len - self.chunks.len()
    }

    /// Returns an iterator with all of the chunks matching the given type
    pub fn chunks_by_type<T: AsRef<[u8]>>(&self, type_: T) -> impl Iterator<Item = &Chunk> {
        // Search from the back as we are usually looking for a chunk that has been added at the
        // end.
        self.chunks
            .iter()
            .filter(move |chunk| chunk.type_.as_bytes() == type_.as_ref())
    }

    /// Returns a deduplicated and sorted list of all of the chunk types
    pub fn chunk_types(&self) -> Vec<Cow<'_, str>> {
        let mut chunk_types: Vec<Cow<'_, str>> = self
            .chunks
            .iter()
            .map(|chunk| String::from_utf8_lossy(chunk.type_.as_bytes()))
            .collect();

        chunk_types.sort_unstable();
        chunk_types.dedup();

        chunk_types
    }

    pub fn into_chunks(self) -> Vec<Chunk> {
        self.chunks
    }
}
