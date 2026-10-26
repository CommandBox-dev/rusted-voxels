use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion, structure::{ProceduralStructure, Structure}}, game::terrain_features::oak_tree};

pub struct OakTree {}

impl Structure for OakTree {

    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion) {
        let lx = random_range(0..CHUNK_WIDTH);
        let lz = random_range(0..CHUNK_WIDTH);

        let top = chunk.get_top_block(lx, lz);
        let top_block = top.0;
        if top_block == 17 {return;}
        let top = top.1;


        let w = 1;
        let h = random_range(5..10);

        for x in lx-w..lx+w {
            for z in lz-w..lz+w {
                for y in top..top+h {
                    chunk.set_block_global(region, x, y as i32, z, 16);
                }
            }
        }

        for i in 1..random_range(3..5) {
            chunk.set_block_global(region, lx-1+random_range(0..=2), top as i32, lz-1+random_range(0..=2), 16);
        }

        oak_tree::fill_sphere(lx, (top + h) as i32, lz, 4, vec![17], chunk, region);
    }

}

pub fn fill_sphere(lx: i32, ly: i32, lz: i32, radius: u32, fill_blocks: Vec<u32>, chunk: &mut Chunk, region: &mut ChunkRegion) {

    if fill_blocks.len() > 1 {
        for sx in lx - radius as i32..lx + radius as i32 {
            for sy in ly - radius as i32..ly + radius as i32 {
                for sz in lz - radius as i32..lz + radius as i32 {
                    let dx = sx - lx;
                    let dy = sy - ly;
                    let dz = sz - lz;

                    if (dx * dx + dy * dy + dz * dz <= (radius * radius) as i32) {
                        chunk.set_block_global(
                            region,
                            sx,
                            sy,
                            sz,
                            fill_blocks[rand::random_range(0..fill_blocks.len())],
                        );
                    }
                }
            }
        }
    } else {
        // unessesary optimization...
        for sx in lx - radius as i32..lx + radius as i32 {
            for sy in ly - radius as i32..ly + radius as i32 {
                for sz in lz - radius as i32..lz + radius as i32 {
                    let dx = sx - lx;
                    let dy = sy - ly;
                    let dz = sz - lz;

                    if (dx * dx + dy * dy + dz * dz <= (radius * radius) as i32) {
                        chunk.set_block_global(
                            region,
                            sx,
                            sy,
                            sz,
                            fill_blocks[0]
                        );
                    }
                }
            }
        }
    }
}