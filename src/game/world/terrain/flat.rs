use rand::random_range;

use crate::engine::world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk}, generation::world_generator::TerrainGenerator};

#[derive(Debug, Clone)]
pub struct FlatGen {
    pub layers: Vec<TerrainLayer>,
}

impl TerrainGenerator for FlatGen {
    type GenOptions = Vec<TerrainLayer>;

    fn new(layers: Self::GenOptions) -> Self where Self: Sized {
        Self {
            layers
        }
    }
    
    fn generate_chunk_terrain(&self, chunk: &mut Chunk) {
        let mut y_offset = 0;
        for layer in &self.layers {

            let height = layer.height;
            let block = layer.blocktype;

            for y in y_offset..y_offset + height {
                for x in 0..CHUNK_WIDTH {
                    for z in 0..CHUNK_WIDTH {
                        let voxel_index = Chunk::voxel_index(x, y as i32, z);

                        chunk.blocktype_data[voxel_index] = block as u8;
                    }
                }
            }
            y_offset += height;
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerrainLayer {
    height: u32,
    blocktype: u32,
}

impl TerrainLayer {
    pub fn new(height: u32, blocktype: u32) -> Self {
        Self {
            height,
            blocktype,
        }
    }
}
// if random_range(0..40) == 0 { 19 } else { 4 };
// should terrain layers support having multiple blocktypes per layer?