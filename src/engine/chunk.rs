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

        let mut block_data = Box::new([0; CHUNK_VOLUME as usize]);

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    if y < 100 { 
                        block_data[index(x, y, z)] = rand::random_range(1..=2);
                    }
                }
            }
        }

        Self {
            cx,
            cz,
            cbx: cx * CHUNK_WIDTH,
            cbz: cz * CHUNK_WIDTH,
            block_data,
            vao: 0,
            vbo: 0,
            vertex_count: 0,
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {
        // return if outside chunk bounds
        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return;}

        self.block_data[index(x, y, z)] = block;
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u32 {
         // return if outside chunk bounds
        if x < 0 || x >= CHUNK_WIDTH ||
           z < 0 || z >= CHUNK_WIDTH ||
           y < 0 || y >= CHUNK_HEIGHT
        {return 0}

        self.block_data[index(x, y, z)]
    }

    fn get_block_global(&self, world: &mut World, lx: i32, ly: i32, lz: i32) -> u32 {
        // returns global world blocks if position is outside chunk bounds
        if ly < 0 || ly >= CHUNK_HEIGHT {return 0;}

        if lx < 0 || lx >= CHUNK_WIDTH || lz < 0 || lz >= CHUNK_WIDTH {
            let gx = self.cbx + lx;
            let gz = self.cbz + lz;
            return world.get_block(gx, ly, gz);
        }

        self.get_block(lx, ly, ly)

    }

    pub fn build_mesh(&self, world: &World, block_models: &[BlockModel; 2]) -> Vec<f32> {
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
                            Chunk::add_face(&mut mesh, face, x, y, z, block, &block_models);
                        }
                    }
                }
            }
        }
        mesh
    }

    fn get_face_mask(&self, x: i32, y: i32, z: i32, world: &World) -> u32 {
        let mut mask = 0;

        if self.get_block(x + 1, y, z) == 0 {mask |= 1;}
        if self.get_block(x - 1, y, z) == 0 {mask |= 2;}
        if self.get_block(x, y + 1, z) == 0 {mask |= 4;}
        if self.get_block(x, y - 1, z) == 0 {mask |= 8;}
        if self.get_block(x, y, z + 1) == 0 {mask |= 16;}
        if self.get_block(x, y, z - 1) == 0 {mask |= 32;}

        mask
    }

    fn add_face(mesh: &mut Vec<f32>, face: u32, x: i32, y: i32, z: i32, block: u32, block_models: &[BlockModel; 2]) {
        let start = 6 * face; // 6 vertices (two triangles) * face index
        let b = block as usize - 1;

        // triangle 1 (lower)
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_min_x, block_models[b].uv_min_y, face, start);
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_max_x, block_models[b].uv_min_y, face, start + 1);
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_max_x, block_models[b].uv_max_y, face, start + 2);
        // triangle 2 (upper)
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_min_x, block_models[b].uv_min_y, face, start + 3);
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_max_x, block_models[b].uv_max_y, face, start + 4);
        Chunk::add_vertex(mesh, x, y, z, block_models[b].uv_min_x, block_models[b].uv_max_y, face, start + 5);

        /*Chunk::add_vertex(mesh, x, y, z, 1.0, 0.5, face, start + 3);
        Chunk::add_vertex(mesh, x, y, z, 0.5, 0.5, face, start + 4);
        Chunk::add_vertex(mesh, x, y, z, 0.5, 0.0, face, start + 5);*/
    }

    fn add_vertex(mesh: &mut Vec<f32>, x: i32, y: i32, z: i32, u: f32, v: f32, face: u32, vertex: u32) {
        let vertex_sub_pos = (vertex * 5) as usize; // vec3 pos + vec2 uv (uv is replaced now but still in sample mesh data)
        // position
        mesh.push(VERTICES[vertex_sub_pos] + x as f32);
        mesh.push(VERTICES[vertex_sub_pos + 1] + y as f32);
        mesh.push(VERTICES[vertex_sub_pos + 2] + z as f32);
        // uv
        mesh.push(u);
        mesh.push(v);
        // light
        mesh.push(NORMAL_LIGHT[face as usize]);
    }
}