pub static CHUNK_WIDTH: i32 = 16;
pub static CHUNK_HEIGHT: i32 = 256;

pub static CHUNK_AREA: i32 = CHUNK_WIDTH * CHUNK_WIDTH;
pub static CHUNK_VOLUME: i32 = CHUNK_AREA * CHUNK_HEIGHT;

pub struct Chunk {
    x: i32,
    z: i32,
    block_data: [u32; CHUNK_VOLUME as usize]
}   

fn index(x: i32, y: i32, z: i32) -> usize {
    (x + z * CHUNK_WIDTH + (y % CHUNK_HEIGHT) * CHUNK_AREA) as usize
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            block_data: [1; CHUNK_VOLUME as usize]
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
}