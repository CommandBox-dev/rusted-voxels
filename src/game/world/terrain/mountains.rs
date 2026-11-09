use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType::{self, OpenSimplex2}};
use rand::random_range;

use crate::{engine::world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk}, generation::world_generator::TerrainGenerator}, game::blocks::blocks::Blocktype};


pub struct MountainGen {
    pub noise: FastNoiseLite,
    pub noise2: FastNoiseLite,
    pub overhang_noise: FastNoiseLite,
    pub cave_noise: FastNoiseLite,
    pub biome_noise: FastNoiseLite,
}

impl TerrainGenerator for MountainGen {
    type GenOptions = ();

    fn new(layers: Self::GenOptions) -> Self where Self: Sized {

        let mut noise = FastNoiseLite::new();
        let mut noise2 = FastNoiseLite::new();
        let mut overhang_noise = FastNoiseLite::new();
        let mut cave_noise = FastNoiseLite::new();
        let mut biome_noise = FastNoiseLite::new();

        noise.set_frequency(Some(0.015));
        noise.set_fractal_octaves(Some(5));
        noise.set_domain_warp_amp(Some(25.0));
        noise.set_fractal_type(Some(FractalType::Ridged));

        noise2.set_seed(Some(721854));
        noise2.set_frequency(Some(0.015));
        noise2.set_fractal_octaves(Some(5));
        noise2.set_domain_warp_amp(Some(25.0));
        noise2.set_fractal_type(Some(FractalType::FBm));

        biome_noise.set_seed(Some(275));
        biome_noise.set_fractal_octaves(Some(3));
        biome_noise.set_frequency(Some(0.01));

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
            noise2,
            overhang_noise,
            cave_noise,
            biome_noise,
        }
    }
    
    fn generate_chunk_terrain(&self, chunk: &mut Chunk) {
        let height_offset = 10;

        
        for x in 0..CHUNK_WIDTH {
            let gx = chunk.block_coord.x + x;
            for z in 0..CHUNK_WIDTH {
                let gz = chunk.block_coord.z + z;
                
                let value = self.noise.get_noise_2d(gx as f32 * 0.1, gz as f32 * 0.1);
                let value2 = self.noise.get_noise_2d(gx as f32 * 0.1, gz as f32 * 0.1);
                let biome = self.biome_noise.get_noise_2d(gx as f32 * 1.0, gz as f32 * 1.0);
                
                let mut placed = false;

                for y in 0..CHUNK_HEIGHT {

                    let local_voxel_index = Chunk::voxel_index(x, y, z);
                    if y == 0 {
                        chunk.blocktype_data[local_voxel_index] = Blocktype::CompressedStone;
                        continue;
                    }

                    if y <= height_offset {
                        if y == height_offset {
                            chunk.blocktype_data[local_voxel_index] = Blocktype::Sand; //if biome > 0.1 { Blocktype::Sand } else { Blocktype::Grass };
                        } else {
                            chunk.blocktype_data[local_voxel_index] = Blocktype::Stone;
                        }
                        continue;
                    } if y as f32 <= value * 100.0 || y as f32 <= value2 * 10.0 + 15.0 {
                        chunk.blocktype_data[local_voxel_index] = Blocktype::Cobblestone;
                    } else if !placed {
                        placed = true;
                        let path = gx < 10 && gx > 5;

                        if path {
                            chunk.blocktype_data[local_voxel_index] = Blocktype::Cobblestone;
                        } else {
                            chunk.blocktype_data[local_voxel_index] = Blocktype::Sand;
                        }
                    }
                }
            }
        }
    }
}