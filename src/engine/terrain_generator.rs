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
    biome_noise: FastNoiseLite,
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

        let mut terra_gen = TerrainGenerator {
            seed,
            worldtype: Worldtype::DEFAULT,
            dimension: Dimension::OVERWORLD,

            noise: FastNoiseLite::new(),
            biome_noise: FastNoiseLite::new(),
        };

        terra_gen.biome_noise.set_seed(Some(275));
        terra_gen.biome_noise.set_fractal_octaves(Some(3));

        terra_gen
    }

    pub fn generate_chunk_terrain(&self, chunk: &mut Chunk, total_block_count: u32) {

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {

                let mut placed = false;
                let value = self.noise.get_noise_2d((x + chunk.cbx) as f32 * 2.0, (z + chunk.cbz) as f32 * 2.0);
                let biome = self.biome_noise.get_noise_2d((x + chunk.cbx) as f32 * 0.3, (z + chunk.cbz) as f32 * 0.3);

                for y in 0..CHUNK_HEIGHT {
                    if (y as f32) < (value + 5.0) * 20.0 {
                        let index = chunk::voxel_index(x, y, z);
                        chunk.block_data[index] = 8; //rand::random_range(1..=total_block_count);
                        placed = true;
                        if rand::random_range(0..=1500) == 0 {chunk.block_data[index] = 3}
                    } else {
                        if placed {
                            placed = false;
                            chunk.block_data[voxel_index(x, y, z)] = if biome > 0.5 {3} else {4};
                        }
                    }
                }
            }
        }
    }

}