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

    pub fn gen_terrain<T: TerrainGenerator>(&self, chunk: &mut Chunk, terrain_generator: &T) {
        terrain_generator.generate_chunk_terrain(chunk);
        
        chunk.set_generation_step(GenerationStep::Terrain);
    }

    pub fn generate_structure<S: Structure>(structure: &S, chunk: &mut Chunk, region: &mut ChunkRegion) {
        structure.generate(chunk, region);
    }
}

pub trait TerrainGenerator {
    type GenOptions;

    fn new(options: Self::GenOptions) -> Self where Self: Sized; // Box<dyn TerrainGenerator>

    fn generate_chunk_terrain(&self, chunk: &mut Chunk);
}