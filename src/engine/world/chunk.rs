use std::alloc::System;

use crate::engine::world::{chunk_region::ChunkRegion, chunk_state::GenerationStep, voxel_coordinates::{BlockCoord, ChunkCoord}, world::World};

pub static CHUNK_WIDTH: i32 = 16;
pub static CHUNK_HEIGHT: i32 = 256;

pub static CHUNK_AREA: i32 = CHUNK_WIDTH * CHUNK_WIDTH;
pub static CHUNK_VOLUME: i32 = CHUNK_AREA * CHUNK_HEIGHT;

// cx, cy, cz = chunk coordinates
// cbx, cby, cbz = chunk coordinates in global block coordinates; chunk (cx: 1, cz: 3) = (cbx: 16, cbz: 48)
// lx, ly, lz = local block coordinates
// gx, gy, gz = global block

// TODO: Chunks should own their matrices / the chunk mesh
// TODO: Chunk mesh should be a seperate genral purpose or custom mesh struct

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub block_coord: BlockCoord, // Chunk position in blocks
    gen_step: GenerationStep,
    dirty: bool,

    pub blocktype_data: Box<[u32; CHUNK_VOLUME as usize]>,
    pub light_data: Box<[u32; CHUNK_VOLUME as usize]>,

    pub uptime: u32,

    // mesh
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: u32,
}

impl Chunk {
    pub fn voxel_index(x: i32, y: i32, z: i32) -> usize {
        (x + z * CHUNK_WIDTH + (y % CHUNK_HEIGHT) * CHUNK_AREA) as usize
    }

    pub fn new(cx: i32, cz: i32) -> Self {

        let coord = ChunkCoord::new(cx, cz);
        let block_coord = BlockCoord::from_chunk_coord(&coord);

        Self {
            coord,
            block_coord,
            gen_step: GenerationStep::Empty,
            dirty: false,

            blocktype_data: Box::new([0; CHUNK_VOLUME as usize]),
            light_data: Box::new([0; CHUNK_VOLUME as usize]),

            uptime: 0,

            vao: 0,
            vbo: 0,
            vertex_count: 0,
        }
    }


    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    #[inline]
    pub fn generation_step(&self) -> GenerationStep {
        self.gen_step
    }

    pub fn set_generation_step(&mut self, step: GenerationStep) {
        self.gen_step = step;
    }


    pub fn get_block(&self, lx: i32, ly: i32, lz: i32) -> u32 {
        if lx < 0
        || ly < 0
        || lz < 0
        || lx >= CHUNK_WIDTH
        || ly >= CHUNK_HEIGHT
        || lz >= CHUNK_WIDTH
        {
            return 0;
        }

        let index = Chunk::voxel_index(lx, ly, lz);

        self.blocktype_data[index]
    }

    pub fn set_block(&mut self, lx: i32, ly: i32, lz: i32, block: u32) {
        if lx < 0
        || ly < 0
        || lz < 0
        || lx >= CHUNK_WIDTH
        || ly >= CHUNK_HEIGHT 
        || lz >= CHUNK_WIDTH
        {
            return;
        }

        let index = Chunk::voxel_index(lx, ly, lz);

        self.blocktype_data[index] = block;
    }
    /// Replaces block and returns the replaced block
    pub fn insert_block(&mut self, lx: i32, ly: i32, lz: i32, block: u32) -> u32 {
        if lx < 0
        || ly < 0
        || lz < 0
        || lx >= CHUNK_WIDTH
        || ly >= CHUNK_HEIGHT
        || lz >= CHUNK_WIDTH
        {
            return u32::MAX;
        }

        let index = Chunk::voxel_index(lx, ly, lz);

        let replaced_block = self.blocktype_data[index];
        self.blocktype_data[index] = block;

        replaced_block
    }

    /// Returns the block of the adjacent chunk if the position is outside chunk bounds
    // TODO: rename top chunk to up chunk
    pub fn get_block_global(&self, region: &ChunkRegion, lx: i32, ly: i32, lz: i32) -> u32 {
        if ly < 0 || ly >= CHUNK_HEIGHT {
            return u32::MAX;
        }

        let is_top = lz >= CHUNK_WIDTH;
        let is_down = lz < 0;
        let is_right = lx >= CHUNK_WIDTH;
        let is_left = lx < 0;
        let is_center_v = !is_top && !is_down;
        let is_center_h = !is_right && !is_left;

        // block coord outside chunk check
        if  is_left && is_top {
            return region.top_left_chunk.get_block(lx+CHUNK_WIDTH, ly, lz-CHUNK_WIDTH);
        }
        if is_center_h && is_top {
            return region.top_chunk.get_block(lx, ly, lz-CHUNK_WIDTH);
        }
        if is_right && is_top {
            return region.top_right_chunk.get_block(lx-CHUNK_WIDTH, ly, lz-CHUNK_WIDTH);
        }
        if is_right && is_center_v {
            return region.right_chunk.get_block(lx-CHUNK_WIDTH, ly, lz);
        }
        if is_right && is_down {
            return region.down_right_chunk.get_block(lx-CHUNK_WIDTH, ly, lz+CHUNK_WIDTH);
        }
        if is_center_h && is_down {
            return region.down_chunk.get_block(lx, ly, lz+CHUNK_WIDTH);
        }
        if is_left && is_down {
            return region.down_left_chunk.get_block(lx+CHUNK_WIDTH, ly, lz+CHUNK_WIDTH);
        }
        if is_left && is_center_v {
            return region.left_chunk.get_block(lx+CHUNK_WIDTH, ly, lz);
        }

        self.get_block(lx, ly, lz)
    }

    pub fn set_block_global(&mut self, region: &mut ChunkRegion, lx: i32, ly: i32, lz: i32, block: u32) {
        if ly < 0 || ly >= CHUNK_HEIGHT {
            return;
        }

        let is_top = lz >= CHUNK_WIDTH;
        let is_down = lz < 0;
        let is_right = lx >= CHUNK_WIDTH;
        let is_left = lx < 0;
        let is_center_v = !is_top && !is_down;
        let is_center_h = !is_right && !is_left;

        // block coord outside chunk check
        if  is_left && is_top {
            region.top_left_chunk.set_block(lx+CHUNK_WIDTH, ly, lz-CHUNK_WIDTH, block);
        }
        if is_center_h && is_top {
            region.top_chunk.set_block(lx, ly, lz-CHUNK_WIDTH, block);
        }
        if is_right && is_top {
            region.top_right_chunk.set_block(lx-CHUNK_WIDTH, ly, lz-CHUNK_WIDTH, block);
        }
        if is_right && is_center_v {
            region.right_chunk.set_block(lx-CHUNK_WIDTH, ly, lz, block);
        }
        if is_right && is_down {
            region.down_right_chunk.set_block(lx-CHUNK_WIDTH, ly, lz+CHUNK_WIDTH, block);
        }
        if is_center_h && is_down {
            region.down_chunk.set_block(lx, ly, lz+CHUNK_WIDTH, block);
        }
        if is_left && is_down {
            region.down_left_chunk.set_block(lx+CHUNK_WIDTH, ly, lz+CHUNK_WIDTH, block);
        }
        if is_left && is_center_v {
            region.left_chunk.set_block(lx+CHUNK_WIDTH, ly, lz, block);
        }

        self.set_block(lx, ly, lz, block)
    }

    /// Returns the top block on that coordinate and the height
    /// or AIR and 0 if there is only air.
    pub fn get_top_block(&self, lx: i32, lz: i32) -> (u32, u32) {

        for ly in (0..CHUNK_HEIGHT).rev() {

            let index = Chunk::voxel_index(lx, ly, lz);
            let block = self.blocktype_data[index];

            if block != 0 {
                return (block, ly as u32);
            }
        }

        (0, 0)
    }

    pub fn get_top_block_global(&self, region: &ChunkRegion, lx: i32, lz: i32) -> (u32, u32) {

        let is_top = lz >= CHUNK_WIDTH;
        let is_down = lz < 0;
        let is_right = lx >= CHUNK_WIDTH;
        let is_left = lx < 0;
        let is_center_v = !is_top && !is_down;
        let is_center_h = !is_right && !is_left;

        if  is_left && is_top {
            return region.top_left_chunk.get_top_block(lx+CHUNK_WIDTH, lz-CHUNK_WIDTH);
        }
        if is_center_h && is_top {
            return region.top_chunk.get_top_block(lx, lz-CHUNK_WIDTH);
        }
        if is_right && is_top {
            return region.top_right_chunk.get_top_block(lx-CHUNK_WIDTH, lz-CHUNK_WIDTH);
        }
        if is_right && is_center_v {
            return region.right_chunk.get_top_block(lx-CHUNK_WIDTH, lz);
        }
        if is_right && is_down {
            return region.down_right_chunk.get_top_block(lx-CHUNK_WIDTH, lz+CHUNK_WIDTH);
        }
        if is_center_h && is_down {
            return region.down_chunk.get_top_block(lx, lz+CHUNK_WIDTH);
        }
        if is_left && is_down {
            return region.down_left_chunk.get_top_block(lx+CHUNK_WIDTH, lz+CHUNK_WIDTH);
        }
        if is_left && is_center_v {
            return region.left_chunk.get_top_block(lx+CHUNK_WIDTH, lz);
        }

        self.get_top_block(lx, lz)
    }


    pub fn get_light(&self, lx: i32, ly: i32, lz: i32) -> u32 {
        if lx < 0
        || ly < 0
        || lz < 0
        || lx >= CHUNK_WIDTH
        || ly >= CHUNK_HEIGHT
        || lz >= CHUNK_WIDTH
        {
            return 0;
        }

        let index = Chunk::voxel_index(lx, ly, lz);

        self.light_data[index]
    }

    pub fn set_light(&mut self, lx: i32, ly: i32, lz: i32, light: u32) {
        if lx < 0
        || ly < 0
        || lz < 0
        || lx >= CHUNK_WIDTH
        || ly >= CHUNK_HEIGHT
        || lz >= CHUNK_WIDTH
        {
            return;
        }

        let index = Chunk::voxel_index(lx, ly, lz);

        self.light_data[index] = light;
    }


    pub fn get_light_global(&self, region: &ChunkRegion, lx: i32, ly: i32, lz: i32) -> u32 {
        if ly < 0 || ly >= CHUNK_HEIGHT {
            return 0;
        }

        let is_top = lz >= CHUNK_WIDTH;
        let is_down = lz < 0;
        let is_right = lx >= CHUNK_WIDTH;
        let is_left = lx < 0;
        let is_center_v = !is_top && !is_down;
        let is_center_h = !is_right && !is_left;

        // block coord outside chunk check
        if  is_left && is_top {
            return region.top_left_chunk.get_light(lx+CHUNK_WIDTH, ly, lz-CHUNK_WIDTH);
        }
        if is_center_h && is_top {
            return region.top_chunk.get_light(lx, ly, lz-CHUNK_WIDTH);
        }
        if is_right && is_top {
            return region.top_right_chunk.get_light(lx-CHUNK_WIDTH, ly, lz-CHUNK_WIDTH);
        }
        if is_right && is_center_v {
            return region.right_chunk.get_light(lx-CHUNK_WIDTH, ly, lz);
        }
        if is_right && is_down {
            return region.down_right_chunk.get_light(lx-CHUNK_WIDTH, ly, lz+CHUNK_WIDTH);
        }
        if is_center_h && is_down {
            return region.down_chunk.get_light(lx, ly, lz+CHUNK_WIDTH);
        }
        if is_left && is_down {
            return region.down_left_chunk.get_light(lx+CHUNK_WIDTH, ly, lz+CHUNK_WIDTH);
        }
        if is_left && is_center_v {
            return region.left_chunk.get_light(lx+CHUNK_WIDTH, ly, lz);
        }

        self.get_light(lx, ly, lz)
    }

    pub fn set_light_global(&mut self, region: &mut ChunkRegion, lx: i32, ly: i32, lz: i32, light: u32) {
        if ly < 0 || ly >= CHUNK_HEIGHT {
            return;
        }

        let is_top = lz >= CHUNK_WIDTH;
        let is_down = lz < 0;
        let is_right = lx >= CHUNK_WIDTH;
        let is_left = lx < 0;
        let is_center_v = !is_top && !is_down;
        let is_center_h = !is_right && !is_left;

        // block coord outside chunk check
        if  is_left && is_top {
            region.top_left_chunk.set_light(lx+CHUNK_WIDTH, ly, lz-CHUNK_WIDTH, light);
        }
        if is_center_h && is_top {
            region.top_chunk.set_light(lx, ly, lz-CHUNK_WIDTH, light);
        }
        if is_right && is_top {
            region.top_right_chunk.set_light(lx-CHUNK_WIDTH, ly, lz-CHUNK_WIDTH, light);
        }
        if is_right && is_center_v {
            region.right_chunk.set_light(lx-CHUNK_WIDTH, ly, lz, light);
        }
        if is_right && is_down {
            region.down_right_chunk.set_light(lx-CHUNK_WIDTH, ly, lz+CHUNK_WIDTH, light);
        }
        if is_center_h && is_down {
            region.down_chunk.set_light(lx, ly, lz+CHUNK_WIDTH, light);
        }
        if is_left && is_down {
            region.down_left_chunk.set_light(lx+CHUNK_WIDTH, ly, lz+CHUNK_WIDTH, light);
        }
        if is_left && is_center_v {
            region.left_chunk.set_light(lx+CHUNK_WIDTH, ly, lz, light);
        }

        self.set_light(lx, ly, lz, light)
    }

}
