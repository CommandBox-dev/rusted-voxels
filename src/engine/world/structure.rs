use crate::engine::world::{chunk::Chunk, chunk_region::ChunkRegion, generation::terrain_generator::TerrainGenerator};

pub trait Structure {
    /// Tests if structure will be placed in the chunk
    fn test(& self, chunk: Chunk, generator: &TerrainGenerator) {
        todo!();
        // check if biome is correct and random place chance 
        // check chunk for space and correct floor
    }
    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion);
}

pub trait ProceduralStructure : Structure {

}