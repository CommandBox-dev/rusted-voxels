use std::{ops::Add, sync::{Arc, RwLock, mpsc::TryRecvError}};

use glam::*;
use rand::{rand_core::block, random_range, seq::IndexedRandom};

use crate::engine::{blocks::block_model::BlockModel, core::worker::Worker, world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH}, chunk_manager::ChunkManager}}
;

pub const RENDER_DISTANCE: u32 = 16;
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

    pub fn update_dirty_chunks(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {
        self.chunk_manager.update_dirty_chunks(block_models, worker);
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
            let lx = gx.rem_euclid(CHUNK_WIDTH);
            let lz = gz.rem_euclid(CHUNK_WIDTH);
            let replaced_block = chunk.insert_block(lx, gy, lz, block);

            if replaced_block == 5 {
                //self.fill_sphere(gx, gy, gz, 8, vec![0]); // 5
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
                        self.chunk_manager.dirty_chunks.push(coord);
                    }
                } else {continue;}
            }
        }
    }

    pub fn get_top_block(&self, gx: i32, gz: i32) -> (u32, u32) {

        let cx = gx.div_euclid(CHUNK_WIDTH);
        let cz = gz.div_euclid(CHUNK_WIDTH);

        if let Some(chunk) = self.chunk_manager.get_avl_chunk_ref(cx, cz) {

            let lx = gx.rem_euclid(CHUNK_WIDTH);
            let lz = gz.rem_euclid(CHUNK_WIDTH);

            return chunk.get_top_block(lx, lz)
        }

        (0, CHUNK_HEIGHT as u32 - 1)
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
        let scale: i32 = 6; // 6
        let width: i32 = 30; // 30
        let height: i32 = 10; // 10

        let half_width: i32 = width / 2;

        let mut grid = vec![false; (width * width) as usize];

        // generate maze
        for x in 0..width {
            for z in 0..width {
                let grid_index = (x * width + z % width) as usize;
                if x == 0 || x == width - 1
                || z == 0 || z == width - 1 {
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
                        let next_grid_index = ((x + 1) * width + z % width) as usize;
                        grid[next_grid_index] = true;
                    } else {
                        let next_grid_index = (x * width + (z + 1) % width) as usize;
                        grid[next_grid_index] = true;
                    }
                };
            }
        }

        // place maze
        for x in 0..width * scale {
            for z in 0..width * scale {
                let grid_index = ((x / scale) * width + (z / scale) % width) as usize;
                let block: u32 = if grid[grid_index] { 7 } else { 0 };

                for y in gy..gy + height {
                    self.set_block(gx + x - half_width, y, gz + z - half_width, block);
                }
                self.set_block(gx + x - half_width, gy - 1, gz + z - half_width, 6);
            }
        }

        // determine and place maze features
        /* for x in 0..width {
            for z in 0..width {
                let grid_index = (x * width + z % width) as usize;
                if grid[grid_index] == false {
                    if x <= 0 || z <= 0 || x >= width || z >= width {
                        continue;
                    }
                    let a = grid[((x + 1) * width + z % width) as usize];
                    let b = grid[((x - 1) * width + z % width) as usize];
                    let c = grid[(x * width + (z + 1) % width) as usize];
                    let d = grid[(x * width + (z - 1) % width) as usize];
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
                            (gx - half_width) + (x * scale) + 2,
                            gy,
                            (gz - half_width) + (z * scale) + 2,
                            5,
                        );
                    }
                    if walls == 1 {
                        self.set_block((gx - half_width) + (x * scale), gy - 1, (gz - half_width) + (z * scale), 0);
                        self.set_block(
                            (gx - half_width) + (x * scale) + 1,
                            gy - 1,
                            (gz - half_width) + (z * scale),
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale) + 2,
                            gy - 1,
                            (gz - half_width) + (z * scale),
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale),
                            gy - 1,
                            (gz - half_width) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale) + 1,
                            gy - 1,
                            (gz - half_width) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale) + 2,
                            gy - 1,
                            (gz - half_width) + (z * scale) + 1,
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale),
                            gy - 1,
                            (gz - half_width) + (z * scale) + 2,
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale) + 1,
                            gy - 1,
                            (gz - half_width) + (z * scale) + 2,
                            0,
                        );
                        self.set_block(
                            (gx - half_width) + (x * scale) + 2,
                            gy - 1,
                            (gz - half_width) + (z * scale) + 2,
                            0,
                        );
                    }
                }
            }
        }*/
    }
}
