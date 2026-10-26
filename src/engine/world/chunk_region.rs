use crate::engine::world::{chunk::Chunk, chunk_manager::{ChunkManager}, chunk_state::ChunkState, voxel_coordinates::ChunkCoord};

/// 3x3 region of chunks
pub struct ChunkRegion {
    pub coord: ChunkCoord,

    //pub center_chunk: Chunk,
    pub top_left_chunk: Chunk,
    pub top_chunk: Chunk,
    pub top_right_chunk: Chunk,
    pub right_chunk: Chunk,
    pub down_right_chunk: Chunk,
    pub down_chunk: Chunk,
    pub down_left_chunk: Chunk,
    pub left_chunk: Chunk,
}

impl ChunkRegion {

    /// Copies a 3x3 chunk region with the given coord being the central chunk or
    /// returns `None` if one or more chunks are either not loaded or currently moved into a thread
    pub fn copy_chunk_region(chunk_manager: &ChunkManager, coord: &ChunkCoord) -> Option<ChunkRegion> {

        let mid_h = coord.x;
        let mid_v = coord.z;
        let right = coord.x + 1;
        let left = coord.x - 1;
        let up = coord.z + 1;
        let down = coord.z - 1;

        let chunks_available =
            chunk_manager.complete_chunk_avail(left, up)
            && chunk_manager.complete_chunk_avail(mid_h, up)
            && chunk_manager.complete_chunk_avail(right, up)
            && chunk_manager.complete_chunk_avail(right, mid_v)
            && chunk_manager.complete_chunk_avail(right, down)
            && chunk_manager.complete_chunk_avail(mid_h, down)
            && chunk_manager.complete_chunk_avail(left, down)
            && chunk_manager.complete_chunk_avail(left, mid_v)
        ;

        if !chunks_available {return None;}

        let top_left_chunk = chunk_manager.clone_chunk(left, up);
        let top_chunk = chunk_manager.clone_chunk(mid_h, up);
        let top_right_chunk = chunk_manager.clone_chunk(right, up);
        let right_chunk = chunk_manager.clone_chunk(right, mid_v);
        let down_right_chunk = chunk_manager.clone_chunk(right, down);
        let down_chunk = chunk_manager.clone_chunk(mid_h, down);
        let down_left_chunk = chunk_manager.clone_chunk(left, down);
        let left_chunk = chunk_manager.clone_chunk(left, mid_v);
    
        return Some(ChunkRegion {
            coord: ChunkCoord::new(coord.x, coord.z),

            top_left_chunk: top_left_chunk.unwrap(),
            top_chunk: top_chunk.unwrap(),
            top_right_chunk: top_right_chunk.unwrap(),
            right_chunk: right_chunk.unwrap(),
            down_right_chunk: down_right_chunk.unwrap(),
            down_chunk: down_chunk.unwrap(),
            down_left_chunk: down_left_chunk.unwrap(),
            left_chunk: left_chunk.unwrap(),
        });
    }

    pub fn take_chunk_region(chunk_manager: &mut ChunkManager, coord: &ChunkCoord) -> Option<ChunkRegion> {
        let mid_h = coord.x;
        let mid_v = coord.z;
        let right = coord.x + 1;
        let left = coord.x - 1;
        let up = coord.z + 1;
        let down = coord.z - 1;

        let chunks_available =
            chunk_manager.chunk_avail(mid_h, mid_v)
            && chunk_manager.chunk_avail(left, up)
            && chunk_manager.chunk_avail(mid_h, up)
            && chunk_manager.chunk_avail(right, up)
            && chunk_manager.chunk_avail(right, mid_v)
            && chunk_manager.chunk_avail(right, down)
            && chunk_manager.chunk_avail(mid_h, down)
            && chunk_manager.chunk_avail(left, down)
            && chunk_manager.chunk_avail(left, mid_v)
        ;

        if !chunks_available {return None;}

        let top_left_chunk = chunk_manager.take_avl_chunk(left, up);
        let top_chunk = chunk_manager.take_avl_chunk(mid_h, up);
        let top_right_chunk = chunk_manager.take_avl_chunk(right, up);
        let right_chunk = chunk_manager.take_avl_chunk(right, mid_v);
        let down_right_chunk = chunk_manager.take_avl_chunk(right, down);
        let down_chunk = chunk_manager.take_avl_chunk(mid_h, down);
        let down_left_chunk = chunk_manager.take_avl_chunk(left, down);
        let left_chunk = chunk_manager.take_avl_chunk(left, mid_v);
    
        return Some(ChunkRegion {
            coord: ChunkCoord::new(coord.x, coord.z),

            top_left_chunk: top_left_chunk.unwrap(),
            top_chunk: top_chunk.unwrap(),
            top_right_chunk: top_right_chunk.unwrap(),
            right_chunk: right_chunk.unwrap(),
            down_right_chunk: down_right_chunk.unwrap(),
            down_chunk: down_chunk.unwrap(),
            down_left_chunk: down_left_chunk.unwrap(),
            left_chunk: left_chunk.unwrap(),
        });
    }

    pub fn insert_chunk_region(chunk_manager: &mut ChunkManager, coord: &ChunkCoord, region: ChunkRegion) {
        chunk_manager.insert_chunk(region.top_left_chunk, coord.x-1, coord.z+1);
        chunk_manager.insert_chunk(region.top_chunk, coord.x, coord.z+1);
        chunk_manager.insert_chunk(region.top_right_chunk, coord.x+1, coord.z+1);
        chunk_manager.insert_chunk(region.right_chunk, coord.x+1, coord.z);
        chunk_manager.insert_chunk(region.down_right_chunk, coord.x+1, coord.z-1);
        chunk_manager.insert_chunk(region.down_chunk, coord.x, coord.z-1);
        chunk_manager.insert_chunk(region.down_left_chunk, coord.x-1, coord.z-1);
        chunk_manager.insert_chunk(region.left_chunk, coord.x-1, coord.z);
    }
}