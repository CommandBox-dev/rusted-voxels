use std::{ops::Add, sync::mpsc::TryRecvError};

use glam::*;
use rand::{rand_core::block, random_range, seq::IndexedRandom};

use crate::engine::{blocks::block_model::BlockModel, world::chunk::{self, CHUNK_WIDTH, Chunk, GenerationStep}, rendering::chunk_mesher, rendering::chunk_renderer, world::generation::terrain_generator::{self, TerrainGenerator}, core::worker::Worker, world};

pub const RENDER_DISTANCE: u32 = 32; // 32
pub const RENDER_AREA: u32 = RENDER_DISTANCE * RENDER_DISTANCE;

pub struct World {
    pub chunks: Box<[Option<Chunk>; (RENDER_DISTANCE * RENDER_DISTANCE) as usize]>, //Vec<Chunk>,
    dirty_chunks: Vec<u32>,
    loading_chunks: Vec<GeneratingChunk>,
    circvec_start_x: u32,
    circvec_start_z: u32,
    chunks_x_offset: i32,
    chunks_z_offset: i32,
}

pub struct GeneratingChunk {
    pub cx: i32,
    pub cz: i32,
    pub receiver: std::sync::mpsc::Receiver<Chunk>,
}

impl World {
    pub fn new() -> Self {

        let mut chunks: Box<[Option<Chunk>; RENDER_AREA as usize]> =
            Box::new(std::array::from_fn(|_| None));

        let mut new_self = Self {
            chunks,
            dirty_chunks: Vec::new(),
            loading_chunks: Vec::new(),
            circvec_start_x: 0,
            circvec_start_z: 0,
            chunks_x_offset: 0,
            chunks_z_offset: 0
        };

        new_self
    }

    pub fn load(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {
        
        let block_count = block_models.len() as u32;

        for x in 0..RENDER_DISTANCE {
            for z in 0..RENDER_DISTANCE {
                let index = World::chunk_index(x as i32, z as i32);

                let receiver = worker.execute( move || {
                    let chunk_generator = TerrainGenerator::new(12345, terrain_generator::Worldtype::DEFAULT);
                    let mut chunk = Chunk::new(x as i32, z as i32);

                    chunk_generator.generate_chunk_terrain(&mut chunk, block_count);

                    chunk
                });

                self.loading_chunks.push(GeneratingChunk {cx: x as i32, cz: z as i32, receiver});
            }
        }
    }

    pub fn update(&mut self) {

        //TODO: THE GENERATING CHUNKS VECTOR NEVER GETS CLEARED, FIX
        for i in 0..self.loading_chunks.len() {
            if i >= self.loading_chunks.len() {break};
            
            match self.loading_chunks[i].receiver.try_recv() {
                Ok(chunk) => {
                    let index = self.get_chunk_index(chunk.cx, chunk.cz);
                    self.chunks[index as usize] = Some(chunk);
                    //self.loading_chunks.remove(i);
                }
                Err(TryRecvError::Empty) => {/* chunk still generating */}
                Err(TryRecvError::Disconnected) => {/* inactive / panicked */}
            }
        }
        
        for i in 0..self.chunks.len() {
            if let Some(chunk) = &self.chunks[i] {
                if chunk.generation_step == GenerationStep::Terrain {
                    if !self.chunk_neighbors_loaded(chunk.cx, chunk.cz) {continue;}
                } else {continue;}
            } else {continue;}

            let chunk: &mut Chunk = &mut *self.chunks[i].as_mut().unwrap();
            chunk.generation_step = GenerationStep::Complete;
            let cx = chunk.cx;
            let cz = chunk.cz;
            let gx = cx * CHUNK_WIDTH;
            let gz = cz * CHUNK_WIDTH; 
            //if random_range(0..=40) == 0 {self.generate_maze(gx, 30, gz);}
            self.dirty_chunks.push(i as u32);
        }
    }

    pub fn chunk_neighbors_loaded(&self, cx: i32, cz: i32) -> bool {
        if self.get_chunk(cx + 1, cz + 1).is_none() {return false;}
        if self.get_chunk(cx + 1, cz + 0).is_none() {return false;}
        if self.get_chunk(cx + 1, cz - 1).is_none() {return false;}

        if self.get_chunk(cx + 0, cz + 1).is_none() {return false;}
        if self.get_chunk(cx + 0, cz - 1).is_none() {return false;}

        if self.get_chunk(cx - 1, cz + 1).is_none() {return false;}
        if self.get_chunk(cx - 1, cz + 0).is_none() {return false;}
        if self.get_chunk(cx - 1, cz - 1).is_none() {return false;}
        true
    }

    pub fn chunk_index(cx: i32, cz: i32) -> u32 {
        ((cx * RENDER_DISTANCE as i32) + (cz % RENDER_DISTANCE as i32)) as u32
    }

    pub fn get_chunk_index(&self, cx: i32, cz: i32) -> u32 {
        if cx < 0 || cx >= RENDER_DISTANCE as i32 ||
           cz < 0 || cz >= RENDER_DISTANCE as i32 {return u32::MAX;}

        World::chunk_index(cx, cz)
    }

    pub fn get_chunk(&self, x: i32, z: i32) -> Option<&Chunk> {
        if x < 0 || x >= RENDER_DISTANCE as i32 ||
           z < 0 || z >= RENDER_DISTANCE as i32 {return None;}

        self.chunks[World::chunk_index(x, z) as usize].as_ref()
    }

    pub fn get_chunk_mut(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
        if x < 0 || x >= RENDER_DISTANCE as i32 ||
           z < 0 || z >= RENDER_DISTANCE as i32 {return None;}

        self.chunks[World::chunk_index(x, z) as usize].as_mut()
    }

    pub fn get_chunk_from_index(&self, index: u32) -> Option<&Chunk> {
        if index >= self.chunks.len() as u32 {return None;}
        self.chunks[index as usize].as_ref()
    }

    pub fn get_chunk_mut_from_index(&mut self, index: u32) -> Option<&mut Chunk> {
        self.chunks[index as usize].as_mut()
    }

    pub fn get_block(&self, gx: i32, gy: i32, gz: i32) -> u32 {
        let cx = gx / 16;
        let cz = gz / 16;
        
        if let Some(chunk) = self.get_chunk(cx, cz) {
            let lx = gx % 16;
            let lz = gz % 16;
            return chunk.get_block(lx, gy, lz);
        } else {
            return 0; // u32::MAX;
        }
    }

    pub fn set_block(&mut self, gx: i32, gy: i32, gz: i32, block: u32) {
        let cx = gx / 16;
        let cz = gz / 16;
        
        let index = self.get_chunk_index(cx, cz);
        if index == u32::MAX {return;}
        
        if let Some(chunk) = self.get_chunk_mut_from_index(index) {
            let lx = gx % 16;
            let lz = gz % 16;
            let replaced_block = chunk.set_block_and_get(lx, gy, lz, block);

            if replaced_block == 5 {
                self.fill_circle(gx, gy, gz, 8, vec![0]); // 5
            }

            let mut place_pos = IVec3::new(gx, gy, gz);
            
            self.mark_chunk_area_dirty(place_pos, place_pos);
        }
    }

    pub fn mark_chunk_dirty(&mut self, chunk_index: u32) {
        self.dirty_chunks.push(chunk_index);
    }

    pub fn mark_chunk_area_dirty(&mut self, mut min_gp: IVec3, mut max_gp: IVec3) {
        // updates all chunks that are in or are touching the area
        if min_gp.x > max_gp.x || min_gp.y > max_gp.y || min_gp.z > max_gp.z {return;}

        // scales the area by 1 block to update touching chunks
        min_gp -= IVec3::ONE;
        max_gp += IVec3::ONE;

        // faster floor division - 4 = 16, 2^4 = 16 2^5 for 32...
        let cx = min_gp.x / 16; //>> 4;
        let cz = min_gp.z / 16; //>> 4;
        let mcx = max_gp.x / 16; //>> 4;
        let mcz = max_gp.z / 16; //>> 4;

        for x in cx..=mcx {
            for z in cz..=mcz {
                let index = World::chunk_index(x, z);
                if !self.is_chunk_dirty(index) {
                    if let Some(chunk) = &mut self.chunks[index as usize] {chunk.dirty = true;}
                    self.dirty_chunks.push(index);
                }
            }
        }
    }

    pub fn update_dirty_chunks(&mut self, block_models: &Vec<BlockModel>) {
        while let Some(index) = self.dirty_chunks.pop() {
            let mesh = if let Some(chunk) = self.get_chunk_mut_from_index(index) {
                chunk.dirty = false;
                chunk_mesher::build_mesh(self.get_chunk_from_index(index).unwrap(), self, block_models)
            } else {
                continue;
            };
            if let Some(chunk) = self.get_chunk_mut_from_index(index) {
                chunk_renderer::upload_chunk_mesh(chunk, mesh);
            }
        }
    }

    pub fn fill_circle(&mut self, gx: i32, gy: i32, gz: i32, radius: u32, fill_blocks: Vec<u32>) {
        
        if fill_blocks.len() > 1 {
            for sx in gx - radius as i32..gx + radius as i32 {
                for sy in gy - radius as i32..gy + radius as i32 {
                    for sz in gz - radius as i32..gz + radius as i32 {

                        let dx = sx - gx;
                        let dy = sy - gy;
                        let dz = sz - gz;

                        if (dx*dx + dy*dy + dz*dz <= (radius*radius) as i32 ) {
                            self.set_block(sx, sy, sz, fill_blocks[rand::random_range(0..fill_blocks.len())]);
                        }

                    }
                }
            }
        } else {
            // unessesary optimization...
            for sx in gx - radius as i32..gx + radius as i32 {
                for sy in gy - radius as i32..gy + radius as i32 {
                    for sz in gz - radius as i32..gz + radius as i32 {

                        let dx = sx - gx;
                        let dy = sy - gy;
                        let dz = sz - gz;

                        if (dx*dx + dy*dy + dz*dz <= (radius*radius) as i32 ) {
                            let block = self.get_block(sx, sy, sz);
                            self.set_block(sx, sy, sz, fill_blocks[0]);
                        }

                    }
                }
            }
        }
    }

    pub fn is_chunk_dirty(&self, index: u32) -> bool {
        if let Some(chunk) = &self.chunks[index as usize] {return chunk.dirty;}
        true
    }

    pub fn get_random_chunk(&self) -> Option<&Chunk> {
        self.chunks[rand::random_range(0..self.chunks.len())].as_ref()
    }

    pub fn get_random_chunk_mut(&mut self) -> Option<&mut Chunk> {
        self.chunks[rand::random_range(0..self.chunks.len())].as_mut()
    }

    pub fn generate_maze(&mut self, gx: i32, gy: i32, gz: i32) {

        let w = 30;
        let hw = w/2;
        let h = 10;

        let scale = 6;

        let mut grid = [false; 30 * 30];
        for x in 0..w {
            for z in 0..w {
                let grid_index = (x * 30 + z % 30) as usize;
                if x == 0 || x == w-1 || z == 0 || z == w-1 {grid[grid_index] = true; continue;}
                if grid[grid_index] == true {continue;}
                if x%2==0||z%2==0 {grid[grid_index] = false;} else {
                    grid[grid_index] = true;
                    if random_range(0..=2) == 0 {
                        let next_grid_index = ((x + 1) * 30 + z % 30) as usize;
                        grid[next_grid_index] = true;
                    } else {
                        let next_grid_index = (x * 30 + (z + 1) % 30) as usize;
                        grid[next_grid_index] = true;
                    }
                };
            }
        }

        for x in 0..w*scale {
            for z in 0..w*scale {
                let grid_index = ((x/scale) * 30 + (z/scale) % 30) as usize;
                let block: u32 = if grid[grid_index] {7} else {0};

                for y in gy..gy+h {self.set_block(gx+x-hw, y, gz+z-hw, block);}
                self.set_block(gx+x-hw, gy-1, gz+z-hw, 6);
            }
        }

        for x in 0..w {
            for z in 0..w {
                let grid_index = (x * 30 + z % 30) as usize;
                if grid[grid_index] == false {
                    if x <= 0 || z <= 0 || x >= w || z >= w {continue;}
                    let a = grid[((x + 1) * 30 + z % 30) as usize];
                    let b = grid[((x - 1) * 30 + z % 30) as usize];
                    let c = grid[(x * 30 + (z + 1) % 30) as usize];
                    let d = grid[(x * 30 + (z - 1) % 30) as usize];
                    let mut walls = 0;
                    if a {walls += 1;}
                    if b {walls += 1;}
                    if c {walls += 1;}
                    if d {walls += 1;}
                    if walls == 3 {self.set_block((gx-hw)+(x*scale) + 2, gy, (gz-hw)+(z*scale) + 2, 5);}
                    if walls == 1 {
                        self.set_block((gx-hw)+(x*scale), gy-1, (gz-hw)+(z*scale), 0);
                        self.set_block((gx-hw)+(x*scale)+1, gy-1, (gz-hw)+(z*scale), 0);
                        self.set_block((gx-hw)+(x*scale)+2, gy-1, (gz-hw)+(z*scale), 0);
                        self.set_block((gx-hw)+(x*scale), gy-1, (gz-hw)+(z*scale)+1, 0);
                        self.set_block((gx-hw)+(x*scale)+1, gy-1, (gz-hw)+(z*scale)+1, 0);
                        self.set_block((gx-hw)+(x*scale)+2, gy-1, (gz-hw)+(z*scale)+1, 0);
                        self.set_block((gx-hw)+(x*scale), gy-1, (gz-hw)+(z*scale)+2, 0);
                        self.set_block((gx-hw)+(x*scale)+1, gy-1, (gz-hw)+(z*scale)+2, 0);
                        self.set_block((gx-hw)+(x*scale)+2, gy-1, (gz-hw)+(z*scale)+2, 0);
                    }
                }
            }
        }
    }
}