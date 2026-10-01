use std::alloc::System;

use crate::engine::{world::{self, World}};

pub static CHUNK_WIDTH: i32 = 16;
pub static CHUNK_HEIGHT: i32 = 256;

pub static CHUNK_AREA: i32 = CHUNK_WIDTH * CHUNK_WIDTH;
pub static CHUNK_VOLUME: i32 = CHUNK_AREA * CHUNK_HEIGHT;

// cx, cy, cz = chunk coordinates
// cbx, cby, cbz = chunk coordinates in global block coordinates; chunk (cx: 1, cz: 3) = (cbx: 16, cbz: 48)
// lx, ly, lz = local block coordinates
// gx, gy, gz = global block coordinates

pub struct Chunk {
    pub cx: i32,
    pub cz: i32,
    pub cbx: i32,
    pub cbz: i32,

    pub block_data: Box<[u32; CHUNK_VOLUME as usize]>,
    //pub blocks_filled: bool,
    //pub fill_block: u32, // the block if every block in this chunk is the same
    //pub palette: Vec<u32>, // palette index -> block
    //pub palette_indices: Vec<u32>, // block -> palette index

    // mesh
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: u32,
}   

pub fn voxel_index(x: i32, y: i32, z: i32) -> usize {
    (x + z * CHUNK_WIDTH + (y % CHUNK_HEIGHT) * CHUNK_AREA) as usize
}

impl Chunk {

    pub fn new(cx: i32, cz: i32) -> Self {

        let cbx = cx * CHUNK_WIDTH;
        let cbz = cz * CHUNK_WIDTH;
        let mut block_data = Box::new([0; CHUNK_VOLUME as usize]);

        Self {
            cx,
            cz,
            cbx,
            cbz,
            block_data,
            vao: 0,
            vbo: 0,
            vertex_count: 0,
        }
    }

    pub fn get_block(&self, lx: i32, ly: i32, lz: i32) -> u32 {
       
        if lx < 0 || lx >= CHUNK_WIDTH ||
           lz < 0 || lz >= CHUNK_WIDTH ||
           ly < 0 || ly >= CHUNK_HEIGHT
        {return 0}

        self.block_data[voxel_index(lx, ly, lz)]
    }

    pub fn get_block_global(&self, world: &World, lx: i32, ly: i32, lz: i32) -> u32 {
        // returns global world blocks if position is outside chunk bounds
        if ly < 0 || ly >= CHUNK_HEIGHT {return u32::MAX;}

        if lx < 0 || lx >= CHUNK_WIDTH || lz < 0 || lz >= CHUNK_WIDTH {
            let gx = self.cbx + lx;
            let gz = self.cbz + lz;
            return world.get_block(gx, ly, gz);
        }

        self.get_block(lx, ly, lz)

    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {

        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return;}

        self.block_data[voxel_index(x, y, z)] = block;
    }

    pub fn set_block_and_get(&mut self, x: i32, y: i32, z: i32, block: u32) -> u32 {
        // sets block and returns the block it replaced

        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return u32::MAX;}

        let index = voxel_index(x, y, z);

        let replaced_block = self.block_data[index];
        self.block_data[index] = block;

        replaced_block
    }

}