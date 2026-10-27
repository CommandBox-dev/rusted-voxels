use crate::engine::world::{chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk}, chunk_region::ChunkRegion};

pub fn bake_direct_light(chunk: &mut Chunk) {
    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_WIDTH {

            let top = chunk.get_top_block(x, z).1 + 1;

            for y in top as i32..CHUNK_HEIGHT {

                chunk.set_light(x, y, z, 15);
            }
        }
    }
}

pub fn bake_indirect_light(chunk: &mut Chunk, region: &mut ChunkRegion) {
    
    let mut flood_light: Vec<QueuedLight> = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_WIDTH {
            let top = chunk.get_top_block(x, z).1;
            let right = chunk.get_top_block_global(region, x + 1, z);
            let left = chunk.get_top_block_global(region, x - 1, z);
            let front = chunk.get_top_block_global(region, x, z + 1);
            let back = chunk.get_top_block_global(region, x, z - 1);
            for y in 0..CHUNK_HEIGHT {
                // if open sky
                if y > top as i32 {
                    chunk.set_light(x, y, z, 15);
                    // indirect flood fill


                    // right
                    if y < right.1 as i32
                    && chunk.get_block_global(region, x + 1, y, z) == 0 {
                        flood_light.push(QueuedLight { x: x + 1, y, z, value: 14 });
                    }
                    // left
                    if y < left.1 as i32
                    && chunk.get_block_global(region, x - 1, y, z) == 0 {
                        flood_light.push(QueuedLight { x: x - 1, y, z, value: 14 });
                    }
                    // front
                    if y < front.1 as i32
                    && chunk.get_block_global(region, x, y, z + 1) == 0 {
                        flood_light.push(QueuedLight { x, y, z: z + 1, value: 14 });
                    }
                    // back
                    if y < back.1 as i32
                    && chunk.get_block_global(region, x, y, z - 1) == 0 {
                        flood_light.push(QueuedLight { x, y, z: z - 1, value: 14 });
                    }

                } /*else if chunk.get_block(x, y, z) == 8 {
                    flood_light.push(QueuedLight { x, y, z, value: 15 });
                }*/
            }


        }
    }

    while let Some(light) = flood_light.pop() {
        chunk.set_light_global(region, light.x, light.y, light.z, light.value);

        let next = light.value - 1;
        if next == 0 {continue;}

        if chunk.get_block_global(region, light.x + 1, light.y, light.z) == 0
        && chunk.get_light_global(region, light.x + 1, light.y, light.z) < next 
        {
            flood_light.push(QueuedLight { x: light.x + 1, y: light.y, z: light.z, value: next });
        }

        if chunk.get_block_global(region, light.x - 1, light.y, light.z) == 0 
        && chunk.get_light_global(region, light.x - 1, light.y, light.z) < next 
        {
            flood_light.push(QueuedLight { x: light.x - 1, y: light.y, z: light.z, value: next });
        }

        if chunk.get_block_global(region, light.x, light.y + 1, light.z) == 0
        && chunk.get_light_global(region, light.x, light.y + 1, light.z) < next
        {
            flood_light.push(QueuedLight { x: light.x, y: light.y + 1, z: light.z, value: next });
        }

        if chunk.get_block_global(region, light.x, light.y - 1, light.z) == 0
        && chunk.get_light_global(region, light.x, light.y - 1, light.z) < next 
        {
            flood_light.push(QueuedLight { x: light.x, y: light.y - 1, z: light.z, value: next });
        }

        if chunk.get_block_global(region, light.x, light.y, light.z + 1) == 0
        && chunk.get_light_global(region, light.x, light.y, light.z + 1) < next
        {
            flood_light.push(QueuedLight { x: light.x, y: light.y, z: light.z + 1, value: next });
        }

        if chunk.get_block_global(region, light.x, light.y, light.z - 1) == 0
        && chunk.get_light_global(region, light.x, light.y, light.z - 1) < next
        {
            flood_light.push(QueuedLight { x: light.x, y: light.y, z: light.z - 1, value: next });
        }
    }

}

struct QueuedLight {
    x: i32,
    y: i32,
    z: i32,
    value: u32,
}