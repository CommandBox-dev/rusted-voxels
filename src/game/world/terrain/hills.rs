use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType::{self, OpenSimplex2}};
use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk}, generation::world_generator::TerrainGenerator}, game::blocks::blocks::Blocktype};


pub struct HillGen {
    pub noise: FastNoiseLite,
    pub overhang_noise: FastNoiseLite,
    pub cave_noise: FastNoiseLite,
    pub biome_noise: FastNoiseLite,
}

impl TerrainGenerator for HillGen {
    type GenOptions = ();

    fn new(layers: Self::GenOptions) -> Self where Self: Sized {

        let mut noise = FastNoiseLite::new();
        let mut overhang_noise = FastNoiseLite::new();
        let mut cave_noise = FastNoiseLite::new();
        let mut biome_noise = FastNoiseLite::new();

        biome_noise.set_seed(Some(275));
        biome_noise.set_fractal_octaves(Some(3));
        biome_noise.set_frequency(Some(0.01));

        noise.set_frequency(Some(0.015));
        noise.set_domain_warp_amp(Some(25.0));

        overhang_noise.set_seed(Some(9334));
        overhang_noise.set_fractal_octaves(Some(4));
        overhang_noise.set_frequency(Some(0.01));
        overhang_noise.set_domain_warp_amp(Some(25.0));

        cave_noise.set_seed(Some(1572));
        cave_noise.set_noise_type(Some(OpenSimplex2));
        cave_noise.set_fractal_type(Some(fastnoise_lite::FractalType::Ridged));
        cave_noise.set_frequency(Some(0.25));
        cave_noise.set_fractal_octaves(Some(2)); // 4
        cave_noise.set_fractal_lacunarity(Some(1.1));
        cave_noise.set_fractal_gain(Some(1.5));

        Self {
            noise,
            overhang_noise,
            cave_noise,
            biome_noise,
        }
    }
    
    fn generate_chunk_terrain(&self, chunk: &mut Chunk) {
        let height_offset = 30; // 60

        for x in 0..CHUNK_WIDTH {
            let lx = chunk.block_coord.x + x;
            for z in 0..CHUNK_WIDTH {
                let lz = chunk.block_coord.z + z;

                let mut placed = false;
                let value = self
                    .noise
                    .get_noise_2d((x + chunk.block_coord.x) as f32 * 2.0, (z + chunk.block_coord.z) as f32 * 2.0);
                let biome = self.biome_noise
                    .get_noise_2d((x + chunk.block_coord.x) as f32 * 1.0, (z + chunk.block_coord.z) as f32 * 1.0);

                for y in 0..CHUNK_HEIGHT {
                    let ly = y;

                    let local_voxel_index = Chunk::voxel_index(x, y, z);
                    if y == 0 {
                        chunk.blocktype_data[local_voxel_index] = Blocktype::CompressedStone;
                        continue;
                    }

                    let cave = self.cave_noise.get_noise_3d(
                        lx as f32 * 0.1,
                        ly as f32 * 0.1,
                        lz as f32 * 0.1,
                    ) * 1.8;

                    if y <= height_offset && cave <= 1.3 {
                        // 1.0
                        if y == height_offset {
                            chunk.blocktype_data[local_voxel_index] = if biome > 0.1 { Blocktype::Sand } else { Blocktype::Grass };
                        } else {
                            chunk.blocktype_data[local_voxel_index] = Blocktype::Stone;
                        }
                        continue;
                    }

                    let overhang = self.noise.get_noise_3d(
                        lx as f32 * 2.0,
                        (ly - height_offset) as f32 * 1.5,
                        lz as f32 * 2.0,
                    );

                    if ((y - height_offset) as f32) < (value + overhang) * 50.0 && cave <= 1.3 {
                        // 1.0
                        chunk.blocktype_data[local_voxel_index] = Blocktype::Stone;
                        placed = true;
                        /*if rand::random_range(0..=1500) == 0 {
                            chunk.blocktype_data[local_voxel_index] = 14
                        } // 3*/
                    } else {
                        if placed {
                            placed = false;
                            chunk.blocktype_data[local_voxel_index] = if biome > 0.5 { Blocktype::Sand } else { Blocktype::Grass };
                        }
                    }
                }
            }
        }
    }
}