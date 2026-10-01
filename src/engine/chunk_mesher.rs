use crate::chunk::*;
use crate::world::*;
use crate::block_model::*;


pub const VERTICES: [f32; 108 + 72] = [

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

pub const NORMAL_LIGHT: [f32; 6] = [
    0.75, // right
    0.75, // left
    1.0, // top
    0.5, // bottom
    0.65, // front
    0.65, // back
];

pub const AO_CURVE: [f32; 4] = [
    0.0, 0.3, 0.6, 1.0 //0.2, 0.5, 0.7, 1.0
];


pub fn build_mesh(chunk: &Chunk, world: &World, block_models: &Vec<BlockModel>) -> Vec<f32> {
    // the passing of the block model 

    let mut mesh = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {

                let block = chunk.get_block(x, y, z);

                if block == 0 {continue;}

                let mask = get_face_mask(chunk, x, y, z, world);

                for face in 0..6 {
                    if mask & (1 << face) != 0 {
                        add_face(&mut mesh, face, x, y, z, block, &block_models, get_ambient_occlusion(chunk, world, x, y, z, face));
                    }
                }
            }
        }
    }
    mesh
}

fn get_face_mask(chunk: &Chunk, x: i32, y: i32, z: i32, world: &World) -> u32 {
    let mut mask = 0;

    if chunk.get_block_global(world, x + 1, y, z) == 0 {mask |= 1;}
    if chunk.get_block_global(world, x - 1, y, z) == 0 {mask |= 2;}
    if chunk.get_block_global(world, x, y + 1, z) == 0 {mask |= 4;}
    if chunk.get_block_global(world, x, y - 1, z) == 0 {mask |= 8;}
    if chunk.get_block_global(world, x, y, z + 1) == 0 {mask |= 16;}
    if chunk.get_block_global(world, x, y, z - 1) == 0 {mask |= 32;}

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
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 1], nl * ao4, start); // left bottom corner
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 1], nl * ao1, start + 1); // right bottom corner
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 3], nl * ao2, start + 2); // right top corner
    // triangle 2 (upper)
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 1], nl * ao4, start + 3); // left bottom corner
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 2], block_models[b].uvs[uvo + 3], nl * ao2, start + 4); // right top corner
    add_vertex(mesh, x, y, z, block_models[b].uvs[uvo + 0], block_models[b].uvs[uvo + 3], nl * ao3, start + 5); // left top corner
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

fn get_ambient_occlusion(chunk: &Chunk, world: &World, x: i32, y: i32, z: i32, face: u32) -> (u32, u32, u32, u32) {
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
            b1 = chunk.get_block_global(world, x + 1, y - 1, z + 0) != 0;
            b2 = chunk.get_block_global(world, x + 1, y - 1, z - 1) != 0;
            b3 = chunk.get_block_global(world, x + 1, y + 0, z - 1) != 0;
            b4 = chunk.get_block_global(world, x + 1, y + 1, z - 1) != 0;
            b5 = chunk.get_block_global(world, x + 1, y + 1, z + 0) != 0;
            b6 = chunk.get_block_global(world, x + 1, y + 1, z + 1) != 0;
            b7 = chunk.get_block_global(world, x + 1, y + 0, z + 1) != 0;
            b8 = chunk.get_block_global(world, x + 1, y - 1, z + 1) != 0;
        }
        1 => { // x- face
            b1 = chunk.get_block_global(world, x - 1, y - 1, z + 0) != 0;
            b2 = chunk.get_block_global(world, x - 1, y - 1, z + 1) != 0;
            b3 = chunk.get_block_global(world, x - 1, y + 0, z + 1) != 0;
            b4 = chunk.get_block_global(world, x - 1, y + 1, z + 1) != 0;
            b5 = chunk.get_block_global(world, x - 1, y + 1, z + 0) != 0;
            b6 = chunk.get_block_global(world, x - 1, y + 1, z - 1) != 0;
            b7 = chunk.get_block_global(world, x - 1, y + 0, z - 1) != 0;
            b8 = chunk.get_block_global(world, x - 1, y - 1, z - 1) != 0;
        }
        2 => { // y+ face
            b1 = chunk.get_block_global(world, x + 0, y + 1, z + 1) != 0;
            b2 = chunk.get_block_global(world, x + 1, y + 1, z + 1) != 0;
            b3 = chunk.get_block_global(world, x + 1, y + 1, z + 0) != 0;
            b4 = chunk.get_block_global(world, x + 1, y + 1, z - 1) != 0;
            b5 = chunk.get_block_global(world, x + 0, y + 1, z - 1) != 0;
            b6 = chunk.get_block_global(world, x - 1, y + 1, z - 1) != 0;
            b7 = chunk.get_block_global(world, x - 1, y + 1, z + 0) != 0;
            b8 = chunk.get_block_global(world, x - 1, y + 1, z + 1) != 0;
        }
        3 => { // -y face
            b1 = chunk.get_block_global(world, x + 1, y - 1, z + 0) != 0;
            b2 = chunk.get_block_global(world, x + 1, y - 1, z - 1) != 0;
            b3 = chunk.get_block_global(world, x + 0, y - 1, z - 1) != 0;
            b4 = chunk.get_block_global(world, x - 1, y - 1, z - 1) != 0;
            b5 = chunk.get_block_global(world, x - 1, y - 1, z + 0) != 0;
            b6 = chunk.get_block_global(world, x - 1, y - 1, z + 1) != 0;
            b7 = chunk.get_block_global(world, x + 0, y - 1, z + 1) != 0;
            b8 = chunk.get_block_global(world, x + 1, y - 1, z + 1) != 0;
        }
        4 => { // z+ face
            b1 = chunk.get_block_global(world, x + 0, y - 1, z + 1) != 0;
            b2 = chunk.get_block_global(world, x + 1, y - 1, z + 1) != 0;
            b3 = chunk.get_block_global(world, x + 1, y + 0, z + 1) != 0;
            b4 = chunk.get_block_global(world, x + 1, y + 1, z + 1) != 0;
            b5 = chunk.get_block_global(world, x + 0, y + 1, z + 1) != 0;
            b6 = chunk.get_block_global(world, x - 1, y + 1, z + 1) != 0;
            b7 = chunk.get_block_global(world, x - 1, y + 0, z + 1) != 0;
            b8 = chunk.get_block_global(world, x - 1, y - 1, z + 1) != 0;
        }
        5 => { // z- face
            b1 = chunk.get_block_global(world, x + 0, y - 1, z - 1) != 0;
            b2 = chunk.get_block_global(world, x - 1, y - 1, z - 1) != 0;
            b3 = chunk.get_block_global(world, x - 1, y + 0, z - 1) != 0;
            b4 = chunk.get_block_global(world, x - 1, y + 1, z - 1) != 0;
            b5 = chunk.get_block_global(world, x + 0, y + 1, z - 1) != 0;
            b6 = chunk.get_block_global(world, x + 1, y + 1, z - 1) != 0;
            b7 = chunk.get_block_global(world, x + 1, y + 0, z - 1) != 0;
            b8 = chunk.get_block_global(world, x + 1, y - 1, z - 1) != 0;
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

    let ao1 = vertex_ao(b1, b3, b2); // right bottom
    let ao2 = vertex_ao(b3, b5, b4); // right top
    let ao3 = vertex_ao(b5, b7, b6); // left top
    let ao4 = vertex_ao(b7, b1, b8); // left bottom

    (ao1, ao2, ao3, ao4)
}