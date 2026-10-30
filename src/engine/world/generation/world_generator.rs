use std::sync::mpsc;
use std::thread::JoinHandle;
use std::thread::Thread;
use std::thread::spawn;

use crate::engine::world::chunk;
use crate::engine::world::chunk::*;
use crate::engine::world::chunk_region::ChunkRegion;
use crate::engine::world::chunk_state::GenerationStep;
use crate::engine::world::generation::world_generator;
use crate::engine::world::structure::Structure;

use fastnoise_lite::FastNoiseLite;
use fastnoise_lite::FractalType;
use fastnoise_lite::NoiseType;
use fastnoise_lite::NoiseType::OpenSimplex2;
use fastnoise_lite::NoiseType::Perlin;
use rand::Rng;
use rand::random_range;

pub struct WorldGenerator {
    pub seed: i32,
    pub worldtype: Worldtype,
    pub dimension: Dimension,

    noise: FastNoiseLite,
    overhang_noise: FastNoiseLite,
    cave_noise: FastNoiseLite,
    biome_noise: FastNoiseLite,
}

pub enum Worldtype {
    DEFAULT,
    FLAT, // FLAT(optional) layer options and etc
}

pub enum Dimension {
    OVERWORLD,
}

pub enum Biome {}

impl WorldGenerator {
    pub fn new(seed: i32, worldtype: Worldtype) -> Self {
        let mut terra_gen = WorldGenerator {
            seed,
            worldtype: Worldtype::DEFAULT,
            dimension: Dimension::OVERWORLD,

            noise: FastNoiseLite::new(),
            overhang_noise: FastNoiseLite::new(),
            cave_noise: FastNoiseLite::new(),
            biome_noise: FastNoiseLite::new(),
        };

        terra_gen.biome_noise.set_seed(Some(275));
        terra_gen.biome_noise.set_fractal_octaves(Some(3));
        terra_gen.biome_noise.set_frequency(Some(0.01));

        terra_gen.noise.set_frequency(Some(0.015));
        //terra_gen.noise.set_domain_warp_amp(Some(25.0));

        terra_gen.noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        terra_gen.noise.set_fractal_type(Some(FractalType::FBm));
        terra_gen.noise.set_frequency(Some(0.005));
        //terra_gen.noise.set_fractal_lacunarity(Some(3.0));
        terra_gen.noise.set_fractal_octaves(Some(6));
        //terra_gen.noise.set_fractal_gain(Some(0.5));
        
        terra_gen.overhang_noise.set_seed(Some(9334));
        terra_gen.overhang_noise.set_fractal_octaves(Some(4));
        terra_gen.overhang_noise.set_frequency(Some(0.01));
        terra_gen.overhang_noise.set_domain_warp_amp(Some(25.0));

        terra_gen.cave_noise.set_seed(Some(1572));
        terra_gen.cave_noise.set_noise_type(Some(OpenSimplex2));
        terra_gen
            .cave_noise
            .set_fractal_type(Some(fastnoise_lite::FractalType::Ridged));
        terra_gen.cave_noise.set_frequency(Some(0.25));
        terra_gen.cave_noise.set_fractal_octaves(Some(2)); // 4
        terra_gen.cave_noise.set_fractal_lacunarity(Some(1.1));
        terra_gen.cave_noise.set_fractal_gain(Some(1.5));

        terra_gen
    }

    pub fn generate_terrain(&self, chunk: &mut Chunk, total_block_count: u32) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    if y > 30 {continue;}
                    let local_voxel_index = Chunk::voxel_index(x, y, z);
                    chunk.blocktype_data[local_voxel_index] = if random_range(0..40) == 0 { 19 } else { 4 };
                }
            }
        }
        chunk.set_generation_step(GenerationStep::Terrain);
        return;

        let height_offset = 60; // 30

        /*for x in 0..CHUNK_WIDTH {
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
                        chunk.blocktype_data[local_voxel_index] = 15;
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
                            chunk.blocktype_data[local_voxel_index] = if biome > 0.1 { 3 } else { 4 };
                        } else {
                            chunk.blocktype_data[local_voxel_index] = 8;
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
                        chunk.blocktype_data[local_voxel_index] = 8;
                        placed = true;
                        if rand::random_range(0..=1500) == 0 {
                            chunk.blocktype_data[local_voxel_index] = 14
                        } // 3
                    } else {
                        if placed {
                            placed = false;
                            chunk.blocktype_data[local_voxel_index] = if biome > 0.5 { 3 } else { 4 };
                        }
                    }
                }
            }
        }*/


        for x in 0..CHUNK_WIDTH {
            let gx = chunk.block_coord.x + x;
            for z in 0..CHUNK_WIDTH {
                let gz = chunk.block_coord.z + z;

                let value = self.noise.get_noise_2d(
                    gx as f32 * 1.0,
                    gz as f32 * 1.0,
                ) * 50.0 + 50.0;
                
                let mut placed = false;

                for y in 0..CHUNK_HEIGHT {

                    let index = Chunk::voxel_index(x, y, z);

                    if y == 0 {
                        chunk.blocktype_data[index] = 15;
                        continue;
                    }

                    if value > y as f32 {
                        chunk.blocktype_data[index] = 8;
                        placed = true;
                    } else {
                        if placed {
                            chunk.blocktype_data[index] = 18; // 4
                        }
                        placed = false;
                    }
                }
            }
        }
        chunk.set_generation_step(GenerationStep::Terrain);
    }

    pub fn generate_structure<S: Structure>(structure: &S, chunk: &mut Chunk, region: &mut ChunkRegion) {
        structure.generate(chunk, region);
    }
}
