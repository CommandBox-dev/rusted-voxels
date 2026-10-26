use crate::engine::world::chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk};

pub fn bake_direct_light(chunk: &mut Chunk) {
    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_WIDTH {
            let top = chunk.get_top_block(x, z).1 + 1;
            for y in top as i32..CHUNK_HEIGHT {
                chunk.set_light(x, y, z, 16);
            }
        }
    }
}