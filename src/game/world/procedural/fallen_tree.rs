use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion, structure::{ProceduralStructure, Structure}}};

pub struct FallenTree {}

impl Structure for FallenTree {

    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion) {

        let lx = random_range(0..CHUNK_WIDTH);
        let lz = random_range(0..CHUNK_WIDTH);

        let top = chunk.get_top_block(lx, lz);
        let top_block = top.0;
        let top = top.1;
        if top_block == 17 || top_block == 21 {return;}


        let w = 1;
        let h = random_range(5..10);

        for x in lx-w..lx+w {
            for y in top..top + w as u32 + w as u32 {
                for z in lz..lz+h {
                    chunk.set_block_global(region, x, y as i32, z, 16);
                }
            }
        }


    }
}