use std::{ops::Add, sync::{Arc, RwLock, mpsc::TryRecvError}};

use glam::*;
use rand::{rand_core::block, random_range, seq::IndexedRandom};

use crate::engine::{blocks::block_model::BlockModel, core::worker::Worker, world::{chunk::CHUNK_WIDTH, chunk_manager::ChunkManager}}
;

pub const RENDER_DISTANCE: u32 = 10; // 32
pub const RENDER_AREA: u32 = RENDER_DISTANCE * RENDER_DISTANCE;

pub struct World {
    pub chunk_manager: ChunkManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunk_manager: ChunkManager::new(),
        }
    }

    pub fn update(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {
        self.chunk_manager.update(block_models, worker);
    }

    pub fn update_dirty_chunks(&mut self, block_models: &Vec<BlockModel>) {
        self.chunk_manager.update_dirty_chunks(block_models);
    }

    pub fn chunk_neighbors_loaded(&self, cx: i32, cz: i32) -> bool {
        todo!();

        /*if self.get_chunk(cx + 1, cz + 1).is_none() {
            return false;
        }
        if self.get_chunk(cx + 1, cz + 0).is_none() {
            return false;
        }
        if self.get_chunk(cx + 1, cz - 1).is_none() {
            return false;
        }

        if self.get_chunk(cx + 0, cz + 1).is_none() {
            return false;
        }
        if self.get_chunk(cx + 0, cz - 1).is_none() {
            return false;
        }

        if self.get_chunk(cx - 1, cz + 1).is_none() {
            return false;
        }
        if self.get_chunk(cx - 1, cz + 0).is_none() {
            return false;
        }
        if self.get_chunk(cx - 1, cz - 1).is_none() {
            return false;
        }
        true*/
    }

    pub fn get_block(&self, gx: i32, gy: i32, gz: i32) -> u32 {
        let cx = gx.div_euclid(CHUNK_WIDTH);
        let cz = gz.div_euclid(CHUNK_WIDTH);

        if let Some(chunk) = self.chunk_manager.get_avl_chunk_ref(cx, cz) {
            let lx = gx.rem_euclid(CHUNK_WIDTH);
            let lz = gz.rem_euclid(CHUNK_WIDTH);
            return chunk.get_block(lx, gy, lz);
        } else {
            return 0; // u32::MAX;
        }
    }

    pub fn set_block(&mut self, gx: i32, gy: i32, gz: i32, block: u32) {
        let cx = gx.div_euclid(CHUNK_WIDTH);
        let cz = gz.div_euclid(CHUNK_WIDTH);

        if let Some(chunk) = self.chunk_manager.get_avl_chunk_mut(cx, cz) {
            let lx = gx.rem_euclid(CHUNK_WIDTH); // rem_euclid
            let lz = gz.rem_euclid(CHUNK_WIDTH);
            let replaced_block = chunk.set_block_and_get(lx, gy, lz, block);

            if replaced_block == 5 {
                self.fill_sphere(gx, gy, gz, 8, vec![0]); // 5
            }

            let mut place_pos = IVec3::new(gx, gy, gz);

            self.mark_chunk_area_dirty(place_pos, place_pos);
        }
    }

    pub fn mark_chunk_area_dirty(&mut self, mut min_gp: IVec3, mut max_gp: IVec3) {
        // updates all chunks that are in or are touching the area
        if min_gp.x > max_gp.x || min_gp.y > max_gp.y || min_gp.z > max_gp.z {
            return;
        }

        // scales the area by 1 block to update touching chunks
        min_gp -= IVec3::ONE;
        max_gp += IVec3::ONE;

        // faster floor division - 4 = 16, 2^4 = 16 2^5 for 32...
        let cx = min_gp.x.div_euclid(CHUNK_WIDTH); //>> 4;
        let cz = min_gp.z.div_euclid(CHUNK_WIDTH); //>> 4;
        let mcx = max_gp.x.div_euclid(CHUNK_WIDTH); //>> 4;
        let mcz = max_gp.z.div_euclid(CHUNK_WIDTH); //>> 4;

        for x in cx..=mcx {
            for z in cz..=mcz {
                let coord;
                if let Some(chunk) = self.chunk_manager.get_avl_chunk_mut(x, z) {
                    coord = chunk.coord.clone();
                    if !chunk.is_dirty() {
                        chunk.set_dirty(true);
                    }
                } else {continue;}
                self.chunk_manager.dirty_chunks.push(coord);
            }
        }
    }

    pub fn fill_sphere(&mut self, gx: i32, gy: i32, gz: i32, radius: u32, fill_blocks: Vec<u32>) {
        if fill_blocks.len() > 1 {
            for sx in gx - radius as i32..gx + radius as i32 {
                for sy in gy - radius as i32..gy + radius as i32 {
                    for sz in gz - radius as i32..gz + radius as i32 {
                        let dx = sx - gx;
                        let dy = sy - gy;
                        let dz = sz - gz;

                        if (dx * dx + dy * dy + dz * dz <= (radius * radius) as i32) {
                            self.set_block(
                                sx,
                                sy,
                                sz,
                                fill_blocks[rand::random_range(0..fill_blocks.len())],
                            );
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

                        if (dx * dx + dy * dy + dz * dz <= (radius * radius) as i32) {
                            let block = self.get_block(sx, sy, sz);
                            self.set_block(sx, sy, sz, fill_blocks[0]);
                        }
                    }
                }
            }
        }
    }

    pub fn generate_maze(&mut self, gx: i32, gy: i32, gz: i32) {
        let w = 30;
        let hw = w / 2;
        let h = 10;

        let scale = 6;

        let mut grid = [false; 30 * 30];
        for x in 0..w {
            for z in 0..w {
                let grid_index = (x * 30 + z % 30) as usize;
                if x == 0 || x == w - 1 || z == 0 || z == w - 1 {
                    grid[grid_index] = true;
                    continue;
                }
                if grid[grid_index] == true {
                    continue;
                }
                if x % 2 == 0 || z % 2 == 0 {
                    grid[grid_index] = false;
                } else {
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

        for x in 0..w * scale {
            for z in 0..w * scale {
                let grid_index = ((x / scale) * 30 + (z / scale) % 30) as usize;
                let block: u32 = if grid[grid_index] { 7 } else { 0 };

                for y in gy..gy + h {
                    self.set_block(gx + x - hw, y, gz + z - hw, block);
                }
                self.set_block(gx + x - hw, gy - 1, gz + z - hw, 6);
            }
        }

        for x in 0..w {
            for z in 0..w {
                let grid_index = (x * 30 + z % 30) as usize;
                if grid[grid_index] == false {
                    if x <= 0 || z <= 0 || x >= w || z >= w {
                        continue;
                    }
                    let a = grid[((x + 1) * 30 + z % 30) as usize];
                    let b = grid[((x - 1) * 30 + z % 30) as usize];
                    let c = grid[(x * 30 + (z + 1) % 30) as usize];
                    let d = grid[(x * 30 + (z - 1) % 30) as usize];
                    let mut walls = 0;
                    if a {
                        walls += 1;
                    }
                    if b {
                        walls += 1;
                    }
                    if c {
                        walls += 1;
                    }
                    if d {
                        walls += 1;
                    }
                    if walls == 3 {
                        self.set_block(
                            (gx - hw) + (x * scale) + 2,
                            gy,
                            (gz - hw) + (z * scale) + 2,
                            5,
                        );
                    }
                    if walls == 1 {
                        self.set_block((gx - hw) + (x * scale), gy - 1, (gz - hw) + (z * scale), 0);
                        self.set_block(
                            (gx - hw) + (x * scale) + 1,
                            gy - 1,
                            (gz - hw) + (z * scale),
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale) + 2,
                            gy - 1,
                            (gz - hw) + (z * scale),
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale),
                            gy - 1,
                            (gz - hw) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale) + 1,
                            gy - 1,
                            (gz - hw) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale) + 2,
                            gy - 1,
                            (gz - hw) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale),
                            gy - 1,
                            (gz - hw) + (z * scale) + 2,
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale) + 1,
                            gy - 1,
                            (gz - hw) + (z * scale) + 2,
                            0,
                        );
                        self.set_block(
                            (gx - hw) + (x * scale) + 2,
                            gy - 1,
                            (gz - hw) + (z * scale) + 2,
                            0,
                        );
                    }
                }
            }
        }
    }
}
