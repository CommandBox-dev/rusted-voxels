use crate::engine::world::chunk::CHUNK_WIDTH;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
        }
    }

    pub fn from_block_coord(block_coord: &BlockCoord) -> Self {
        Self::new(block_coord.x.div_euclid(CHUNK_WIDTH), block_coord.z.div_euclid(CHUNK_WIDTH))
    }
}

impl BlockCoord {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
        }
    }

    pub fn from_chunk_coord(chunk_coord: &ChunkCoord) -> Self {
        Self::new(chunk_coord.x * CHUNK_WIDTH, chunk_coord.z * CHUNK_WIDTH)
    }
}