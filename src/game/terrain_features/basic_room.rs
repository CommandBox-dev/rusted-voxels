use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion, structure::{ProceduralStructure, Structure}}, game::terrain_features::oak_tree};

pub struct BasicRoom {}

impl Structure for BasicRoom {

    fn generate(&self, chunk: &mut Chunk, region: &mut ChunkRegion) {

        let wall_block = 13;
        let floor_block = 3;

        let top = chunk.get_top_block(0, 0).1 as i32 + 1;

        let floor = true; //random_range(0..10) != 0;
        let right_wall = random_range(0..4) == 0;
        let left_wall = random_range(0..4) == 0;
        let front_wall = random_range(0..4) == 0;
        let back_wall = random_range(0..4) == 0;

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {

                if floor {chunk.set_block(x, top, z, floor_block);}
                //chunk.set_block(x, top as i32 + 8, z, 5);

                if right_wall && x == CHUNK_WIDTH-1 {
                    for y in top+1..top+7 {
                        chunk.set_block(x, y, z, wall_block);
                    }
                }
                if left_wall && x == 0 {
                    for y in top+1..top+7 {
                        chunk.set_block(x, y, z, wall_block);
                    }
                }
                if front_wall && z == CHUNK_WIDTH-1 {
                    for y in top+1..top+7 {
                        chunk.set_block(x, y, z, wall_block);
                    }
                }
                if back_wall && z == 0 {
                    for y in top+1..top+7 {
                        chunk.set_block(x, y, z, wall_block);
                    }
                }

            }
        }

        // säule
        for y in top+1..top+7 {
            chunk.set_block(0, y, 0, wall_block);
        }

    }

}