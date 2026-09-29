use std::ops::Add;

use glam::*;

use crate::engine::{block_model::BlockModel, chunk::{self, Chunk}, chunk_renderer, world};

pub const RENDER_DISTANCE: u32 = 15;
pub const RENDER_AREA: u32 = RENDER_DISTANCE * RENDER_DISTANCE;

pub struct World {
    pub chunks: Box<[Option<Chunk>; (RENDER_DISTANCE * RENDER_DISTANCE) as usize]>, //Vec<Chunk>,
    dirty_chunks: Vec<u32>,
    circvec_start_x: u32,
    circvec_start_z: u32,
    chunks_x_offset: i32,
    chunks_z_offset: i32,
}

impl World {
    pub fn new(block_models: &Vec<BlockModel>) -> Self {

        let mut chunks: Box<[Option<Chunk>; RENDER_AREA as usize]> =
            Box::new(std::array::from_fn(|_| None));
        let mut dirty_chunks = Vec::new();

        for x in 0..RENDER_DISTANCE {
            for z in 0..RENDER_DISTANCE {
                let index = World::chunk_index(x as i32, z as i32);
                chunks[index as usize] = Some(Chunk::new(x as i32, z as i32));
                &dirty_chunks.push(index);
                
            }
        }

        let mut new_self = Self {
            chunks,
            dirty_chunks,
            circvec_start_x: 0,
            circvec_start_z: 0,
            chunks_x_offset: 0,
            chunks_z_offset: 0
        };

        /*for x in 0..world::RENDER_DISTANCE {
            for z in 0..world::RENDER_DISTANCE {
                let index = World::chunk_index(x as i32, z as i32);

                if let Some(chunk) = new_self.chunks[index].as_mut() {
                    let mesh = chunk.build_mesh(&mut new_self, block_models);
                    chunk_renderer::upload_chunk_mesh(chunk, mesh);
                }
            }
        }*/
        new_self
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
        self.chunks[index as usize].as_ref()
    }

    pub fn get_chunk_mut_from_index(&mut self, index: u32) -> Option<&mut Chunk> {
        self.chunks[index as usize].as_mut()
    }

    pub fn get_block(&self, gx: i32, gy: i32, gz: i32) -> u32 {
        let cx = gx / 16;
        let cz = gz / 16;
        let lx = gx % 16;
        let lz = gz % 16;

        if let Some(chunk) = self.get_chunk(cx, cz) {
            return chunk.get_block(lx, gy, lz);
        } else {
            return u32::MAX;
        }
    }

    pub fn set_block(&mut self, gx: i32, gy: i32, gz: i32, block: u32) {
        let cx = gx / 16;
        let cz = gz / 16;
        let lx = gx % 16;
        let lz = gz % 16;

        let index = self.get_chunk_index(cx, cz);
        if index == u32::MAX {return;}

        if let Some(chunk) = self.get_chunk_mut_from_index(index) {
            chunk.set_block(lx, gy, lz, block);

            let mut place_pos = IVec3::new(gx, gy, gz);
            
            self.mark_chunk_area_dirty(place_pos, place_pos);
            //self.mark_chunk_dirty(index);
        }
    }

    pub fn mark_chunk_dirty(&mut self, chunk_index: u32) {
        self.dirty_chunks.push(chunk_index);
    }

    pub fn mark_chunk_area_dirty(&mut self, mut min_gp: IVec3, mut max_gp: IVec3) {
        // updates all chunks that are in or are touching the area
        if min_gp.x > max_gp.x || min_gp.y > max_gp.y || min_gp.z > max_gp.z {return;}

        // scale the area by 1 block to update touching chunks
        min_gp -= IVec3::ONE;
        max_gp += IVec3::ONE;

        let cx = min_gp.x / 16; //>> 4; // floor divide by 16, 2^4 = 16 2^5 for 32...
        let cz = min_gp.z / 16; //>> 4;
        let mcx = max_gp.x / 16; //>> 4;
        let mcz = max_gp.z / 16; //>> 4;

        for x in cx..=mcx {
            for z in cz..=mcz {
                self.dirty_chunks.push(World::chunk_index(x, z));
            }
        }
    }

    pub fn update_dirty_chunks(&mut self, block_models: &Vec<BlockModel>) {
        while let Some(index) = self.dirty_chunks.pop() {
            let mesh = if let Some(chunk) = self.get_chunk_from_index(index) {
                chunk.build_mesh(self, block_models)
            } else {
                continue;
            };
            if let Some(chunk) = self.get_chunk_mut_from_index(index) {
                chunk_renderer::upload_chunk_mesh(chunk, mesh);
            }
        }
    }
}