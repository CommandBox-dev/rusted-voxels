use crate::engine::terrain_generator;
use crate::engine::chunk::*;
use crate::engine::chunk;

use fastnoise_lite::FastNoiseLite;
use rand::Rng;

pub struct TerrainGenerator {
    pub seed: i32,
    pub worldtype: Worldtype,
    pub dimension: Dimension,

    noise: FastNoiseLite,
}

pub enum Worldtype {
    DEFAULT,
    FLAT, // FLAT(optional) layer options and etc
}

pub enum Dimension {
    OVERWORLD,
}

pub enum Biome {
    
}

impl TerrainGenerator {

    pub fn new(seed: i32, worldtype: Worldtype) -> Self {
        Self {
            seed,
            worldtype: Worldtype::DEFAULT,
            dimension: Dimension::OVERWORLD,

            noise: FastNoiseLite::new(),
        }
    }

    pub fn generate_chunk_terrain(&self, chunk: &mut Chunk) {

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {

                let mut placed = false;
                let value = self.noise.get_noise_2d((x + chunk.cbx) as f32 * 2.0, (z + chunk.cbz) as f32 * 2.0);

                for y in 0..CHUNK_HEIGHT {
                    if (y as f32) < (value + 5.0) * 20.0 {
                        let index = chunk::voxel_index(x, y, z);
                        chunk.block_data[index] = rand::random_range(1..=2);
                        placed = true;
                        if rand::random_range(0..=1500) == 0 {chunk.block_data[index] = 3}
                    } else {
                        if placed {
                            placed = false;
                            chunk.block_data[voxel_index(x, y, z)] = 4;
                        }
                    }
                }
            }
        }
    }

}