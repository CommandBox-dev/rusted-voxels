use crate::engine::world::chunk::Chunk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GenerationStep {
    Empty,
    Terrain,
    TerrainFeatures,
    Structures,
    DirectLight,
    IndirectLight,
    Creatures,
    Complete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkState {
    /// Moved in thread and replaced by `ChunkState::InWork` **placeholder**
    Available(Chunk),
    /// Thread is owning the chunk and actively working on it.
    InWork(GenerationStep),
    // Thread copies chunk region on read
    //Complete(Chunk),
}

impl ChunkState {
    //pub fn unwrap_if_min_gen_step(self) -> Option<Chunk> {}
}