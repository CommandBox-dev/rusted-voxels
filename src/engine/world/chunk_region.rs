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

        let top_left_chunk = chunk_manager.clone_chunk(coord.x-1, coord.z+1);
        let top_chunk = chunk_manager.clone_chunk(coord.x, coord.z+1);
        let top_right_chunk = chunk_manager.clone_chunk(coord.x+1, coord.z+1);
        let right_chunk = chunk_manager.clone_chunk(coord.x+1, coord.z);
        let down_right_chunk = chunk_manager.clone_chunk(&coord.x+1, coord.z-1);
        let down_chunk = chunk_manager.clone_chunk(coord.x, coord.z-1);
        let down_left_chunk = chunk_manager.clone_chunk(coord.x-1, coord.z-1);
        let left_chunk = chunk_manager.clone_chunk(coord.x-1, coord.z);

        if
        top_left_chunk.is_some() &&
        top_chunk.is_some() &&
        top_right_chunk.is_some() &&
        right_chunk.is_some() &&
        down_right_chunk.is_some() &&
        down_chunk.is_some() &&
        down_left_chunk.is_some() &&
        left_chunk.is_some() 
        {
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

        None
    }

    pub fn take_chunk_region(chunk_manager: &mut ChunkManager, coord: &ChunkCoord) -> Option<ChunkRegion> {
        //let _center_chunk = chunk_manager.take_chunk(coord);
        let _top_left_chunk = chunk_manager.take_chunk(coord.x-1, coord.z+1);
        let _top_chunk = chunk_manager.take_chunk(coord.x, coord.z+1);
        let _top_right_chunk = chunk_manager.take_chunk(coord.x+1, coord.z+1);
        let _right_chunk = chunk_manager.take_chunk(coord.x+1, coord.z);
        let _down_right_chunk = chunk_manager.take_chunk(&coord.x+1, coord.z-1);
        let _down_chunk = chunk_manager.take_chunk(coord.x, coord.z-1);
        let _down_left_chunk = chunk_manager.take_chunk(coord.x-1, coord.z-1);
        let _left_chunk = chunk_manager.take_chunk(coord.x-1, coord.z);

        //let center_chunk: Option<Chunk>;
        let top_left_chunk: Option<Chunk>;
        let top_chunk: Option<Chunk>;
        let top_right_chunk: Option<Chunk>;
        let right_chunk: Option<Chunk>;
        let down_right_chunk: Option<Chunk>;
        let down_chunk: Option<Chunk>;
        let down_left_chunk: Option<Chunk>;
        let left_chunk: Option<Chunk>;

        /*if let Some(test_chunk) = _center_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {center_chunk = Some(chunk)}
                ChunkState::Generating(chunk, _) => {center_chunk = Some(chunk)}
                ChunkState::InWork(_) => {center_chunk = None}
            }
        }*/
        if let Some(test_chunk) = _top_left_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {top_left_chunk = Some(chunk)}
                ChunkState::InWork(_) => {top_left_chunk = None}
            }
        } else {top_left_chunk = None}
        if let Some(test_chunk) = _top_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {top_chunk = Some(chunk)}
                ChunkState::InWork(_) => {top_chunk = None}
            }
        } else {top_chunk = None}
        if let Some(test_chunk) = _top_right_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {top_right_chunk = Some(chunk)}
                ChunkState::InWork(_) => {top_right_chunk = None}
            }
        } else {top_right_chunk = None}
        if let Some(test_chunk) = _right_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {right_chunk = Some(chunk)}
                ChunkState::InWork(_) => {right_chunk = None}
            }
        } else {right_chunk = None}
        if let Some(test_chunk) = _down_right_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {down_right_chunk = Some(chunk)}
                ChunkState::InWork(_) => {down_right_chunk = None}
            }
        } else {down_right_chunk = None}
        if let Some(test_chunk) = _down_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {down_chunk = Some(chunk)}
                ChunkState::InWork(_) => {down_chunk = None}
            }
        } else {down_chunk = None}
        if let Some(test_chunk) = _down_left_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {down_left_chunk = Some(chunk)}
                ChunkState::InWork(_) => {down_left_chunk = None}
            }
        } else {down_left_chunk = None}
        if let Some(test_chunk) = _left_chunk {
            match test_chunk {
                ChunkState::Available(chunk) => {left_chunk = Some(chunk)}
                ChunkState::InWork(_) => {left_chunk = None}
            }
        } else {left_chunk = None}

        /*println!("{}, {}, {}, {}, {}, {}, {}, {}",
            top_left_chunk.is_some(),
            top_chunk.is_some(),
            top_right_chunk.is_some(),
            right_chunk.is_some(),
            down_right_chunk.is_some(),
            down_chunk.is_some(),
            down_left_chunk.is_some(),
            left_chunk.is_some(),
        );*/

        if
        //center_chunk.is_some() &&
        top_left_chunk.is_some() &&
        top_chunk.is_some() &&
        top_right_chunk.is_some() &&
        right_chunk.is_some() &&
        down_right_chunk.is_some() &&
        down_chunk.is_some() &&
        down_left_chunk.is_some() &&
        left_chunk.is_some() 
        {
            println!("Copied Chunk Region!");
            return Some(ChunkRegion {
                coord: ChunkCoord::new(coord.x, coord.z),
                //center_chunk: center_chunk.unwrap(),
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

        if let Some(top_left_chunk) = top_left_chunk {chunk_manager.insert_chunk(top_left_chunk, coord.x-1, coord.z+1);}
        if let Some(top_chunk) = top_chunk {chunk_manager.insert_chunk(top_chunk, coord.x, coord.z+1);}
        if let Some(top_right_chunk) = top_right_chunk {chunk_manager.insert_chunk(top_right_chunk, coord.x+1, coord.z+1);}
        if let Some(right_chunk) = right_chunk {chunk_manager.insert_chunk(right_chunk, coord.x+1, coord.z);}
        if let Some(down_right_chunk) = down_right_chunk {chunk_manager.insert_chunk(down_right_chunk, coord.x+1, coord.z-1);}
        if let Some(down_chunk) = down_chunk {chunk_manager.insert_chunk(down_chunk, coord.x, coord.z-1);}
        if let Some(down_left_chunk) = down_left_chunk {chunk_manager.insert_chunk(down_left_chunk, coord.x-1, coord.z-1);}
        if let Some(left_chunk) = left_chunk {chunk_manager.insert_chunk(left_chunk, coord.x-1, coord.z);}

        None
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