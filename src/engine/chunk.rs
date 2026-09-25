pub static CHUNK_WIDTH: i32 = 16;
pub static CHUNK_HEIGHT: i32 = 256;

pub static CHUNK_AREA: i32 = CHUNK_WIDTH * CHUNK_WIDTH;
pub static CHUNK_VOLUME: i32 = CHUNK_AREA * CHUNK_HEIGHT;

const VERTICES: [f32; 108] = [
    // right
    0.5,  0.5,  0.5,
    0.5,  0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,

    // left
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,

    // top
    -0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,

    // bottom
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,

    // front
    -0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,

    // back
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5
];

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

    pub fn build_mesh(&self) -> Vec<f32> {

        let mut mesh = Vec::new();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {

                    let block = self.get_block(x, y, z);

                    if block == 0 {continue;}

                    let mask = self.get_face_mask(x, y, z);

                    for face in 0..6 {
                        if mask & (1 << face) != 0 {
                            Chunk::add_face(&mut mesh, face, x, y, z);
                        }
                    }
                    //println!("next_block, xyz: {}, {}, {}", x, y, z);
                }
            }
        }
        mesh
    }

    fn get_face_mask(&self, x: i32, y: i32, z: i32) -> u32 {
        let mut mask = 0;

        if self.get_block(x + 1, y, z) == 0 {mask |= 1;}
        if self.get_block(x - 1, y, z) == 0 {mask |= 2;}
        if self.get_block(x, y + 1, z) == 0 {mask |= 4;}
        if self.get_block(x, y - 1, z) == 0 {mask |= 8;}
        if self.get_block(x, y, z + 1) == 0 {mask |= 16;}
        if self.get_block(x, y, z - 1) == 0 {mask |= 32;}

        mask
    }

    fn add_face(mesh: &mut Vec<f32>, face: u32, x: i32, y: i32, z: i32) {
        let start = 6 * face; // 6 vertices (two triangles) * face index
        let end = start + 6;

        for i in start..end {
            mesh.push(VERTICES[(i * 3) as usize] + x as f32);
            mesh.push(VERTICES[(i * 3 + 1) as usize] + y as f32);
            mesh.push(VERTICES[(i * 3 + 2) as usize] + z as f32);
        }
    }
}