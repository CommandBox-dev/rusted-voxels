use std::sync::mpsc;
use std::thread::JoinHandle;
use std::thread::Thread;
use std::thread::spawn;

use crate::engine::world::chunk;
use crate::engine::world::chunk::*;
use crate::engine::world::generation::terrain_generator;

use fastnoise_lite::FastNoiseLite;
use fastnoise_lite::NoiseType::OpenSimplex2;
use fastnoise_lite::NoiseType::Perlin;
use rand::Rng;

pub struct TerrainGenerator {
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

impl TerrainGenerator {
    pub fn new(seed: i32, worldtype: Worldtype) -> Self {
        let mut terra_gen = TerrainGenerator {
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
        terra_gen.noise.set_domain_warp_amp(Some(25.0));

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

    pub fn generate_chunk_terrain(&self, chunk: &mut Chunk, total_block_count: u32) {
        /*for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    if y > 30 {continue;}
                    let local_voxel_index = chunk::voxel_index(x, y, z);
                    chunk.block_data[local_voxel_index] = 4;
                }
            }
        }
        chunk.generation_step = GenerationStep::Terrain;
        return;*/

        let height_offset = 60; // 30

        for x in 0..CHUNK_WIDTH {
            let lx = chunk.cbx + x;
            for z in 0..CHUNK_WIDTH {
                let lz = chunk.cbz + z;

                let mut placed = false;
                let value = self
                    .noise
                    .get_noise_2d((x + chunk.cbx) as f32 * 2.0, (z + chunk.cbz) as f32 * 2.0);
                let biome = self.biome_noise
                    .get_noise_2d((x + chunk.cbx) as f32 * 1.0, (z + chunk.cbz) as f32 * 1.0);

                for y in 0..CHUNK_HEIGHT {
                    let ly = y;

                    let local_voxel_index = chunk::voxel_index(x, y, z);
                    if y == 0 {
                        chunk.block_data[local_voxel_index] = 15;
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
                            chunk.block_data[local_voxel_index] = if biome > 0.1 { 3 } else { 4 };
                        } else {
                            chunk.block_data[local_voxel_index] = 8;
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
                        chunk.block_data[local_voxel_index] = 8;
                        placed = true;
                        if rand::random_range(0..=1500) == 0 {
                            chunk.block_data[local_voxel_index] = 14
                        } // 3
                    } else {
                        if placed {
                            placed = false;
                            chunk.block_data[local_voxel_index] = if biome > 0.5 { 3 } else { 4 };
                        }
                    }
                }
            }
        }
        chunk.generation_step = GenerationStep::Terrain;
    }
}
