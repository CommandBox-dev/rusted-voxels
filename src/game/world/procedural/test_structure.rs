use rand::random_range;

use crate::engine::world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion, structure::{ProceduralStructure, Structure}};

pub struct TestStruct {}

impl Structure for TestStruct {

    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion) {
        let lx = random_range(0..CHUNK_WIDTH);
        let lz = random_range(0..CHUNK_WIDTH);
        let top = chunk.get_top_block(lx, lz);

        let w = random_range(1..3);
        let h = 5;

        for x in lx-w..lx+w {
            for z in lz-w..lz+w {
                for y in top.1..top.1+h {
                    chunk.set_block_global(region, x, y as i32, z, 8);
                }
            }
        }
    }
    
}