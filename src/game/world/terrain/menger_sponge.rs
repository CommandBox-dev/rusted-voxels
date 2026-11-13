use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk}, generation::world_generator::TerrainGenerator}, game::{blocks::blocks::Blocktype, world::terrain::menger_sponge}};

#[derive(Debug, Clone)]
pub struct MengerSponge {}

impl TerrainGenerator for MengerSponge {
    type GenOptions = ();

    fn new(options: Self::GenOptions) -> Self where Self: Sized {
        Self {}
    }
    
    fn generate_chunk_terrain(&self, chunk: &mut Chunk) {
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    let gx = chunk.block_coord.x + x;
                    let gz = chunk.block_coord.z + z;
                    if menger_sponge::get_menger(gx, y, gz) {
                        chunk.set_block(x, y, z, Blocktype::MazeStone);
                    } else {
                        chunk.set_block(x, y, z, Blocktype::Air);
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn get_menger(mut x: i32, mut y: i32, mut z: i32) -> bool {
    for i in 0..6 {
        let xm = x % 3 == 1;
        let ym = y % 3 == 1;
        let zm = z % 3 == 1;

        if (xm as u32 + ym as u32 + zm as u32) >= 2 {
            return false;
        }
        
        x /= 3;
        y /= 3;
        z /= 3;
    }
    true
}