use std::alloc::System;

use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::engine::{block_model::BlockModel, world::{self, World}};

pub static CHUNK_WIDTH: i32 = 16;
pub static CHUNK_HEIGHT: i32 = 256;

pub static CHUNK_AREA: i32 = CHUNK_WIDTH * CHUNK_WIDTH;
pub static CHUNK_VOLUME: i32 = CHUNK_AREA * CHUNK_HEIGHT;

const VERTICES: [f32; 108 + 72] = [

    // right
    1.0, 0.0, 1.0,   0.0, 0.0,
    1.0, 0.0, 0.0,   1.0, 0.0,
    1.0, 1.0, 0.0,   1.0, 1.0,

    1.0, 0.0, 1.0,   0.0, 0.0,
    1.0, 1.0, 0.0,   1.0, 1.0,
    1.0, 1.0, 1.0,   0.0, 1.0,

    // left
    0.0, 0.0, 0.0,   0.0, 0.0,
    0.0, 0.0, 1.0,   1.0, 0.0,
    0.0, 1.0, 1.0,   1.0, 1.0,

    0.0, 0.0, 0.0,   0.0, 0.0,
    0.0, 1.0, 1.0,   1.0, 1.0,
    0.0, 1.0, 0.0,   0.0, 1.0,

    // top
    0.0, 1.0, 1.0,   0.0, 0.0,
    1.0, 1.0, 1.0,   1.0, 0.0,
    1.0, 1.0, 0.0,   1.0, 1.0,

    0.0, 1.0, 1.0,   0.0, 0.0,
    1.0, 1.0, 0.0,   1.0, 1.0,
    0.0, 1.0, 0.0,   0.0, 1.0,

    // bottom
    0.0, 0.0, 0.0,   0.0, 0.0,
    1.0, 0.0, 0.0,   1.0, 0.0,
    1.0, 0.0, 1.0,   1.0, 1.0,

    0.0, 0.0, 0.0,   0.0, 0.0,
    1.0, 0.0, 1.0,   1.0, 1.0,
    0.0, 0.0, 1.0,   0.0, 1.0,

    // front
    0.0, 0.0, 1.0,   0.0, 0.0,
    1.0, 0.0, 1.0,   1.0, 0.0,
    1.0, 1.0, 1.0,   1.0, 1.0,

    0.0, 0.0, 1.0,   0.0, 0.0,
    1.0, 1.0, 1.0,   1.0, 1.0,
    0.0, 1.0, 1.0,   0.0, 1.0,

    // back
    1.0, 0.0, 0.0,   0.0, 0.0,
    0.0, 0.0, 0.0,   1.0, 0.0,
    0.0, 1.0, 0.0,   1.0, 1.0,

    1.0, 0.0, 0.0,   0.0, 0.0,
    0.0, 1.0, 0.0,   1.0, 1.0,
    1.0, 1.0, 0.0,   0.0, 1.0,
];

const NORMAL_LIGHT: [f32; 6] = [
    0.75, // right
    0.75, // left
    1.0, // top
    0.5, // bottom
    0.65, // front
    0.65, // back
];

const AO_CURVE: [f32; 4] = [
    0.0, 0.3, 0.6, 1.0 //0.2, 0.5, 0.7, 1.0
];

// cx, cy, cz = chunk coordinates
// cbx, cby, cbz = chunk coordinates in global block coordinates; chunk (cx: 1, cz: 3) = (cbx: 16, cbz: 48)
// lx, ly, lz = local block coordinates
// gx, gy, gz = global block coordinates

pub struct Chunk {
    pub cx: i32,
    pub cz: i32,
    pub cbx: i32,
    pub cbz: i32,
    block_data: Box<[u32; CHUNK_VOLUME as usize]>,
    // mesh
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: u32,
}   

fn index(x: i32, y: i32, z: i32) -> usize {
    (x + z * CHUNK_WIDTH + (y % CHUNK_HEIGHT) * CHUNK_AREA) as usize
}

impl Chunk {

    pub fn new(cx: i32, cz: i32) -> Self {
        let cbx = cx * CHUNK_WIDTH;
        let cbz = cz * CHUNK_WIDTH;

        let mut block_data = Box::new([0; CHUNK_VOLUME as usize]);

        let height = rand::random_range(100..104); // 108
        let noise = Perlin::new(12345);

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let mut placed = false;
                for y in 0..CHUNK_HEIGHT {
                    //if y < 100 {
                       // block_data[index(x, y, z)] = rand::random_range(1..=2);
                    let value = noise.get([(x + cbx) as f64 * 0.025, (z + cbz) as f64 * 0.025]);
                       //println!("Noise value: {}", value);

                    if (y as f64) < ((value + 5.0) * 20.0) {
                        block_data[index(x, y, z)] = 1;
                        placed = true;
                    } else {
                        if placed {
                            placed = false;
                            block_data[index(x, y, z)] = 4;
                        }
                    }
                    //}
                }
            }
        }


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

    pub fn set_block_and_get(&mut self, x: i32, y: i32, z: i32, block: u32) -> u32 {
        // sets block and returns the block it replaced

        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return u32::MAX;}

        let index = index(x, y, z);

        let replaced_block = self.block_data[index];
        self.block_data[index] = block;

        replaced_block
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {

        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return;}

        self.block_data[index(x, y, z)] = block;
    }

    pub fn get_block(&self, lx: i32, ly: i32, lz: i32) -> u32 {
       
        if lx < 0 || lx >= CHUNK_WIDTH ||
           lz < 0 || lz >= CHUNK_WIDTH ||
           ly < 0 || ly >= CHUNK_HEIGHT
        {return 0}

        self.block_data[index(lx, ly, lz)]
    }

    fn get_block_global(&self, world: &World, lx: i32, ly: i32, lz: i32) -> u32 {
        // returns global world blocks if position is outside chunk bounds
        if ly < 0 || ly >= CHUNK_HEIGHT {return u32::MAX;}

        if lx < 0 || lx >= CHUNK_WIDTH || lz < 0 || lz >= CHUNK_WIDTH {
            let gx = self.cbx + lx;
            let gz = self.cbz + lz;
            return world.get_block(gx, ly, gz);
        }

        self.get_block(lx, ly, lz)

    }

    pub fn build_mesh(&self, world: &World, block_models: &Vec<BlockModel>) -> Vec<f32> {
        // the passing of the block model 

        let mut mesh = Vec::new();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {

                    let block = self.get_block(x, y, z);

                    if block == 0 {continue;}

                    let mask = self.get_face_mask(x, y, z, world);

                    for face in 0..6 {
                        if mask & (1 << face) != 0 {
                            Chunk::add_face(&mut mesh, face, x, y, z, block, &block_models, self.get_ambient_occlusion(world, x, y, z, face));
                        }
                    }
                }
            }
        }
        mesh
    }

    fn get_face_mask(&self, x: i32, y: i32, z: i32, world: &World) -> u32 {
        let mut mask = 0;

        if self.get_block_global(world, x + 1, y, z) == 0 {mask |= 1;}
        if self.get_block_global(world, x - 1, y, z) == 0 {mask |= 2;}
        if self.get_block_global(world, x, y + 1, z) == 0 {mask |= 4;}
        if self.get_block_global(world, x, y - 1, z) == 0 {mask |= 8;}
        if self.get_block_global(world, x, y, z + 1) == 0 {mask |= 16;}
        if self.get_block_global(world, x, y, z - 1) == 0 {mask |= 32;}

        mask
    }

    fn add_face(mesh: &mut Vec<f32>, face: u32, x: i32, y: i32, z: i32, block: u32, block_models: &Vec<BlockModel>, ao: (u32, u32, u32, u32)) {
        let start = 6 * face; // 6 vertices (two triangles) * face index
        let b = block as usize - 1;

        let ao1 = AO_CURVE[ao.0 as usize];
        let ao2 = AO_CURVE[ao.1 as usize];
        let ao3 = AO_CURVE[ao.2 as usize];
        let ao4 = AO_CURVE[ao.3 as usize];

        // to disable ao
        /*let ao1 = 1.0;
        let ao2 = 1.0;
        let ao3 = 1.0;
        let ao4 = 1.0;*/

        let nl = NORMAL_LIGHT[face as usize];

        let uvo = (face * 4) as usize;

        // triangle 1 (lower)
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 1], nl * ao4, start); // left bottom corner
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 1], nl * ao1, start + 1); // right bottom corner
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 3], nl * ao2, start + 2); // right top corner
        // triangle 2 (upper)
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 1], nl * ao4, start + 3); // left bottom corner
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 3], nl * ao2, start + 4); // right top corner
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 3], nl * ao3, start + 5); // left top corner
    }

    fn add_vertex(mesh: &mut Vec<f32>, x: i32, y: i32, z: i32, u: f32, v: f32, light: f32, vertex: u32) {
        let vertex_sub_pos = (vertex * 5) as usize; // vec3 pos + vec2 uv (uv is replaced now but still in sample mesh data)
        // position
        mesh.push(VERTICES[vertex_sub_pos] + x as f32);
        mesh.push(VERTICES[vertex_sub_pos + 1] + y as f32);
        mesh.push(VERTICES[vertex_sub_pos + 2] + z as f32);
        // uv
        mesh.push(u);
        mesh.push(v);
        // light
        mesh.push(light);
    }

    fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u32 {

        if side1 && side2 {return 0;}

        3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})
    }

    fn get_ambient_occlusion(&self, world: &World, x: i32, y: i32, z: i32, face: u32) -> (u32, u32, u32, u32) {
        // Just copied this method from my Java voxel engine and had almost nothing to change.

        let b1: bool;
        let b2: bool;
        let b3: bool;
        let b4: bool;
        let b5: bool;
        let b6: bool;
        let b7: bool;
        let b8: bool;

        // 2D view how blocks relative to the face are checked.
        // B is the block, the surrounding cells are the blocks
        // that get checked to generate the AO. 

        // | v | < | < |
        // | v | B | ^ |
        // | E | S | > |

        // Or

        // | b6 | b5 | b4 |
        // | b7 | -- | b3 |
        // | b8 | b1 | b2 |

        match face {
            0 => { // x+ face
                b1 = self.get_block_global(world, x + 1, y - 1, z + 0) != 0;
                b2 = self.get_block_global(world, x + 1, y - 1, z - 1) != 0;
                b3 = self.get_block_global(world, x + 1, y + 0, z - 1) != 0;
                b4 = self.get_block_global(world, x + 1, y + 1, z - 1) != 0;
                b5 = self.get_block_global(world, x + 1, y + 1, z + 0) != 0;
                b6 = self.get_block_global(world, x + 1, y + 1, z + 1) != 0;
                b7 = self.get_block_global(world, x + 1, y + 0, z + 1) != 0;
                b8 = self.get_block_global(world, x + 1, y - 1, z + 1) != 0;
            }
            1 => { // x- face
                b1 = self.get_block_global(world, x - 1, y - 1, z + 0) != 0;
                b2 = self.get_block_global(world, x - 1, y - 1, z + 1) != 0;
                b3 = self.get_block_global(world, x - 1, y + 0, z + 1) != 0;
                b4 = self.get_block_global(world, x - 1, y + 1, z + 1) != 0;
                b5 = self.get_block_global(world, x - 1, y + 1, z + 0) != 0;
                b6 = self.get_block_global(world, x - 1, y + 1, z - 1) != 0;
                b7 = self.get_block_global(world, x - 1, y + 0, z - 1) != 0;
                b8 = self.get_block_global(world, x - 1, y - 1, z - 1) != 0;
            }
            2 => { // y+ face
                b1 = self.get_block_global(world, x + 0, y + 1, z + 1) != 0;
                b2 = self.get_block_global(world, x + 1, y + 1, z + 1) != 0;
                b3 = self.get_block_global(world, x + 1, y + 1, z + 0) != 0;
                b4 = self.get_block_global(world, x + 1, y + 1, z - 1) != 0;
                b5 = self.get_block_global(world, x + 0, y + 1, z - 1) != 0;
                b6 = self.get_block_global(world, x - 1, y + 1, z - 1) != 0;
                b7 = self.get_block_global(world, x - 1, y + 1, z + 0) != 0;
                b8 = self.get_block_global(world, x - 1, y + 1, z + 1) != 0;
            }
            3 => { // -y face
                b1 = self.get_block_global(world, x + 1, y - 1, z + 0) != 0;
                b2 = self.get_block_global(world, x + 1, y - 1, z - 1) != 0;
                b3 = self.get_block_global(world, x + 0, y - 1, z - 1) != 0;
                b4 = self.get_block_global(world, x - 1, y - 1, z - 1) != 0;
                b5 = self.get_block_global(world, x - 1, y - 1, z + 0) != 0;
                b6 = self.get_block_global(world, x - 1, y - 1, z + 1) != 0;
                b7 = self.get_block_global(world, x + 0, y - 1, z + 1) != 0;
                b8 = self.get_block_global(world, x + 1, y - 1, z + 1) != 0;
            }
            4 => { // z+ face
                b1 = self.get_block_global(world, x + 0, y - 1, z + 1) != 0;
                b2 = self.get_block_global(world, x + 1, y - 1, z + 1) != 0;
                b3 = self.get_block_global(world, x + 1, y + 0, z + 1) != 0;
                b4 = self.get_block_global(world, x + 1, y + 1, z + 1) != 0;
                b5 = self.get_block_global(world, x + 0, y + 1, z + 1) != 0;
                b6 = self.get_block_global(world, x - 1, y + 1, z + 1) != 0;
                b7 = self.get_block_global(world, x - 1, y + 0, z + 1) != 0;
                b8 = self.get_block_global(world, x - 1, y - 1, z + 1) != 0;
            }
            5 => { // z- face
                b1 = self.get_block_global(world, x + 0, y - 1, z - 1) != 0;
                b2 = self.get_block_global(world, x - 1, y - 1, z - 1) != 0;
                b3 = self.get_block_global(world, x - 1, y + 0, z - 1) != 0;
                b4 = self.get_block_global(world, x - 1, y + 1, z - 1) != 0;
                b5 = self.get_block_global(world, x + 0, y + 1, z - 1) != 0;
                b6 = self.get_block_global(world, x + 1, y + 1, z - 1) != 0;
                b7 = self.get_block_global(world, x + 1, y + 0, z - 1) != 0;
                b8 = self.get_block_global(world, x + 1, y - 1, z - 1) != 0;
            }
            _ => {
                b1 = false;
                b2 = false;
                b3 = false;
                b4 = false;
                b5 = false;
                b6 = false;
                b7 = false;
                b8 = false;
            }
        }

        let ao1 = Chunk::vertex_ao(b1, b3, b2); // right bottom
        let ao2 = Chunk::vertex_ao(b3, b5, b4); // right top
        let ao3 = Chunk::vertex_ao(b5, b7, b6); // left top
        let ao4 = Chunk::vertex_ao(b7, b1, b8); // left bottom

        (ao1, ao2, ao3, ao4)
    }
}