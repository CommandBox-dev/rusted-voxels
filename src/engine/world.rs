use crate::engine::{chunk::{self, Chunk}, chunk_renderer, world};

pub const RENDER_DISTANCE: u32 = 3;

pub struct World {
    // chunk storage
    pub chunks: [Option<Chunk>; (RENDER_DISTANCE * RENDER_DISTANCE) as usize], //Vec<Chunk>,
    circvec_start_x: u32,
    circvec_start_z: u32,
    chunks_x_offset: i32,
    chunks_z_offset: i32,
}

impl World {
    pub fn new() -> Self {

        let mut chunks: [Option<Chunk>; (RENDER_DISTANCE * RENDER_DISTANCE) as usize]
         = [None; (RENDER_DISTANCE * RENDER_DISTANCE) as usize];

        for x in 0..RENDER_DISTANCE {
            for z in 0..RENDER_DISTANCE {
                chunks[World::chunk_index(x as i32, z as i32)] = Some(Chunk::new(x as i32, z as i32));
            }
        }

        for x in 0..world::RENDER_DISTANCE {
            for z in 0..world::RENDER_DISTANCE {
                let index = World::chunk_index(x as i32, z as i32);

                if let Some(chunk) = chunks[index].as_mut() {
                    let mesh = chunk.build_mesh();
                    chunk_renderer::upload_chunk_mesh(chunk, mesh);
                }
            }
        }

        Self {
            chunks, //Vec::new(),
            circvec_start_x: 0,
            circvec_start_z: 0,
            chunks_x_offset: 0,
            chunks_z_offset: 0
        }
    }

    pub fn chunk_index(x: i32, z: i32) -> usize {
        ((x * RENDER_DISTANCE as i32) + (z % RENDER_DISTANCE as i32)) as usize
    }

    pub fn get_chunk(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
        if x < 0 || x >= RENDER_DISTANCE as i32 ||
           z < 0 || z >= RENDER_DISTANCE as i32 {return None;}

        self.chunks[World::chunk_index(x, z)].as_mut()
    }

    pub fn get_block(&mut self, gx: i32, gy: i32, gz: i32) -> u32 {
        let cx = gx / 16;
        let cz = gz / 16;
        let lx = gx % 16;
        let lz = gz % 16;

        let chunk = self.get_chunk(cx, cz);

        if let Some(chunk) = chunk {
            return chunk.get_block(lx, gy, lz);
        } else {
            return 0;
        }
    }

    pub fn set_block(&mut self, gx: i32, gy: i32, gz: i32, block: u32) {
        let cx = gx / 16;
        let cz = gz / 16;
        let lx = gx % 16;
        let lz = gz % 16;

        let chunk = self.get_chunk(cx, cz);

        if let Some(mut chunk) = chunk {
            chunk.set_block(lx, gy, lz, block);
            let mesh = chunk.build_mesh();
            chunk_renderer::upload_chunk_mesh(&mut chunk, mesh);
        }
    }
}