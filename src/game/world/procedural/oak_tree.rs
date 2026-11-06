use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion, structure::{ProceduralStructure, Structure}}, game::blocks::blocks::Blocktype};

pub struct OakTree {}

impl Structure for OakTree {

    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion) {

        let apple_tree = random_range(0..35) == 0;

        let lx = random_range(0..CHUNK_WIDTH);
        let lz = random_range(0..CHUNK_WIDTH);

        let top = chunk.get_top_block(lx, lz);
        let top_block = top.0;
        let top = top.1;
        if top_block == Blocktype::Leaves && top_block == Blocktype::OakLog {return;} // &&


        let w = 1;
        let h = random_range(5..10);

        for x in lx-w..lx+w {
            for z in lz-w..lz+w {
                for y in top..top+h {
                    chunk.set_block_global(region, x, y as i32, z, Blocktype::OakLog);
                }
            }
        }
        // bush
        for x in lx-w-1..lx+w+1 {
            for z in lz-w-1..lz+w+1 {
                chunk.set_block_global(region, x, top as i32 + 1, z, Blocktype::Leaves);
                for y in top-1..top+1 {
                }
            }
        }

        for i in 1..random_range(3..5) {
            chunk.set_block_global(region, lx-1+random_range(0..=2), top as i32, lz-1+random_range(0..=2), Blocktype::OakLog);
        }

        fill_sphere(lx, (top + h) as i32, lz, 4, vec![Blocktype::Leaves, if apple_tree && random_range(0..20) == 0 {Blocktype::Leaves} else {Blocktype::Leaves}], chunk, region);
    }

}

pub fn fill_sphere(lx: i32, ly: i32, lz: i32, radius: u32, fill_blocks: Vec<Blocktype>, chunk: &mut Chunk, region: &mut ChunkRegion) {

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