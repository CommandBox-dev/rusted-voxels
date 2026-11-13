use crate::block::block_model::*;
use crate::engine::block::block::Block;
use crate::engine::rendering::chunk_mesher;
use crate::engine::world::chunk_region::ChunkRegion;
use crate::engine::world::world::*;
use crate::game::blocks::blocks::Blocktype;
use crate::world::chunk::*;

pub const VERTICES: [f32; 108 + 72] = [
    // right
    1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
    0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, // left
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top
    0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // bottom
    0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // front
    0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, // back
    1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
];

pub const NORMAL_LIGHT: [f32; 6] = [
    0.9, // right
    0.9, // left
    1.2, // top
    0.5, // bottom
    0.7, // front
    0.7, // back
];

pub const AO_CURVE: [f32; 4] = [
    //0.0, 0.3, 0.6, 1.0,
    0.2, 0.5, 0.7, 1.0
];

pub const LIGHT_CURVE: [f32; 16] = [
    0.01,
    0.02,
    0.03,
    0.045,
    0.06,
    0.08,
    0.12,
    0.15,
    0.2,
    0.26,
    0.30,
    0.42,
    0.62,
    0.75,
    0.85,
    0.95,
];

pub const FACE_RIGHT: usize = 0;
pub const FACE_LEFT: usize = 1;
pub const FACE_TOP: usize = 2;
pub const FACE_BOTTOM: usize = 3;
pub const FACE_FRONT: usize = 4;
pub const FACE_BACK: usize = 5;

pub const FACE_RIGHT_: u32 = 0;
pub const FACE_LEFT_: u32 = 1;
pub const FACE_TOP_: u32 = 2;
pub const FACE_BOTTOM_: u32 = 3;
pub const FACE_FRONT_: u32 = 4;
pub const FACE_BACK_: u32 = 5;

/*pub const LIGHT_CURVE: [f32; 16] = [
    0.01,
    0.02,
    0.03,
    0.045,
    0.06,
    0.08,
    0.12,
    0.15,
    0.2,
    0.26,
    0.30,
    0.42,
    0.62,
    0.75,
    0.85,
    1.2,
];*/

pub fn build_mesh(chunk: &Chunk, region: &ChunkRegion, block_models: &Vec<BlockModel>) -> Vec<f32> {

    let rotation = 2;
    let inverted_rotation = (4 - rotation) % 4;

    let mut mesh = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                let block = chunk.get_block(x, y, z);

                if block == Blocktype::Air {
                    continue;
                }

                let model = &block_models[block as usize];

                match model.shape {

                    BlockShape::Voxel => {
                        let mask = get_face_mask_voxel(chunk, x, y, z, region, block_models, inverted_rotation);
                        for face in 0..6 {
                            if mask & (1 << face) != 0 {
                                add_face(
                                    &mut mesh,
                                    face,
                                    x,
                                    y,
                                    z,
                                    block,
                                    &model,
                                    &chunk,
                                    &region,
                                );
                            }
                        }
                    }

                    BlockShape::CubeMesh => {

                        let mask = get_face_mask_cube_mesh(chunk, x, y, z, region, model, block_models, inverted_rotation);

                        for part in 0..model.parts {
                            for world_face in 0..6 {
                                let local_face = chunk_mesher::get_rotated_face_id(world_face, inverted_rotation);

                                let world_face_index = part * 6 + world_face;
                                let local_face_index = part * 6 + local_face;

                                if mask & (1 << world_face) != 0 || model.cullable_faces[local_face_index as usize] == false {
                                    add_face_cube_mesh(
                                        &mut mesh,
                                        world_face,
                                        local_face,
                                        world_face_index,
                                        local_face_index,
                                        rotation,
                                        x,
                                        y,
                                        z,
                                        block,
                                        &model,
                                        &chunk,
                                        &region,
                                    );
                                }
                            }
                        }
                    }

                    BlockShape::Custom => {}

                    BlockShape::Empty => {}
                }
            }
        }
    }
    mesh
}

fn get_face_mask_cube_mesh(chunk: &Chunk, x: i32, y: i32, z: i32, region: &ChunkRegion, model: &BlockModel, models: &[BlockModel], rotation: u32) -> u32 {
    let mut mask = 0;

    let right = chunk.get_block_global(region, x + 1, y, z);
    let left = chunk.get_block_global(region, x - 1, y, z);
    let top = chunk.get_block_global(region, x, y + 1, z);
    let bottom = chunk.get_block_global(region, x, y - 1, z);
    let front = chunk.get_block_global(region, x, y, z + 1);
    let back = chunk.get_block_global(region, x, y, z - 1);

    let face_right = chunk_mesher::get_rotated_face_id(FACE_RIGHT_, rotation);
    let face_left = chunk_mesher::get_rotated_face_id(FACE_LEFT_, rotation);
    let face_top = chunk_mesher::get_rotated_face_id(FACE_TOP_, rotation);
    let face_bottom = chunk_mesher::get_rotated_face_id(FACE_BOTTOM_, rotation);
    let face_front = chunk_mesher::get_rotated_face_id(FACE_FRONT_, rotation);
    let face_back = chunk_mesher::get_rotated_face_id(FACE_BACK_, rotation);

    // right face
    if right == Blocktype::Air {
        mask |= 1;
    } else  {
        let neighbor_model = &models[right as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_right, face_left) {
            mask |= 1;
        }
    }
    // left face
    if left == Blocktype::Air {
        mask |= 2;
    } else  {
        let neighbor_model = &models[left as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_left, face_right) {
            mask |= 2;
        }
    }
    // top face
    if top == Blocktype::Air {
        mask |= 4;
    } else  {
        let neighbor_model = &models[top as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_top, face_bottom) {
            mask |= 4;
        }
    }
    // bottom face
    if bottom == Blocktype::Air {
        mask |= 8;
    } else  {
        let neighbor_model = &models[bottom as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_bottom, face_top) {
            mask |= 8;
        }
    }
    // front face
    if front == Blocktype::Air {
        mask |= 16;
    } else  {
        let neighbor_model = &models[front as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_front, face_back) {
            mask |= 16;
        }
    }
    // back face
    if back == Blocktype::Air {
        mask |= 32;
    } else  {
        let neighbor_model = &models[back as usize];

        if neighbor_model.shape == BlockShape::Custom
        || neighbor_model.shape == BlockShape::CubeMesh
        && should_render_cube_face(model, neighbor_model, face_back, face_front) {
            mask |= 32;
        }
    }

    mask
}

#[inline(always)]
fn should_render_cube_face(
    current: &BlockModel,
    neighbor: &BlockModel,
    current_face: u32,
    neighbor_face: u32,
) -> bool {
    !current.occlusion.faces[current_face as usize]
    .is_fully_occluded(
        &neighbor.occlusion.faces[neighbor_face as usize]
    )
}

fn get_face_mask_voxel(chunk: &Chunk, x: i32, y: i32, z: i32, region: &ChunkRegion, models: &[BlockModel], rotation: u32) -> u32 {

    let mut mask = 0;

    let right = chunk.get_block_global(region, x + 1, y, z);
    let left = chunk.get_block_global(region, x - 1, y, z);
    let top = chunk.get_block_global(region, x, y + 1, z);
    let bottom = chunk.get_block_global(region, x, y - 1, z);
    let front = chunk.get_block_global(region, x, y, z + 1);
    let back = chunk.get_block_global(region, x, y, z - 1);

    let face_right = chunk_mesher::get_rotated_face_id(FACE_RIGHT_, rotation) as usize;
    let face_left = chunk_mesher::get_rotated_face_id(FACE_LEFT_, rotation) as usize;
    let face_top = chunk_mesher::get_rotated_face_id(FACE_TOP_, rotation) as usize;
    let face_bottom = chunk_mesher::get_rotated_face_id(FACE_BOTTOM_, rotation) as usize;
    let face_front = chunk_mesher::get_rotated_face_id(FACE_FRONT_, rotation) as usize;
    let face_back = chunk_mesher::get_rotated_face_id(FACE_BACK_, rotation) as usize;

    // right face
    if right == Blocktype::Air {
        mask |= 1;
    } else  {
        let model = &models[right as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_left].is_full() {
            mask |= 1;
        }
    }
    // left face
    if left == Blocktype::Air {
        mask |= 2;
    } else  {
        let model = &models[left as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_right].is_full() {
            mask |= 2;
        }
    }
    // top face
    if top == Blocktype::Air {
        mask |= 4;
    } else  {
        let model = &models[top as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_bottom].is_full() {
            mask |= 4;
        }
    }
    // bottom face
    if bottom == Blocktype::Air {
        mask |= 8;
    } else  {
        let model = &models[bottom as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_top].is_full() {
            mask |= 8;
        }
    }
    // front face
    if front == Blocktype::Air {
        mask |= 16;
    } else  {
        let model = &models[front as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_back].is_full() {
            mask |= 16;
        }
    }
    // back face
    if back == Blocktype::Air {
        mask |= 32;
    } else  {
        let model = &models[back as usize];

        if model.shape == BlockShape::Empty
        || model.shape == BlockShape::Custom
        || model.shape == BlockShape::CubeMesh
        && !model.occlusion.faces[face_front].is_full() {
            mask |= 32;
        }
    }

    mask
}

#[inline(always)]
fn add_face(
    mesh: &mut Vec<f32>,
    face: u32,
    x: i32,
    y: i32,
    z: i32,
    block: Blocktype,
    model: &BlockModel,
    chunk: &Chunk,
    region: &ChunkRegion,
) {
    let start = 6 * face; // 6 vertices (two triangles) * face index
    let uvo = (face * 4) as usize;

    //let ao = get_ambient_occlusion(chunk, region, x, y, z, face);
    let sl = get_smooth_light(chunk, region, x, y, z, face);
    // let l = get_face_light(chunk, region, x, y, z, face);
    
    //let ao1 = AO_CURVE[ao.0 as usize];
    //let ao2 = AO_CURVE[ao.1 as usize];
    //let ao3 = AO_CURVE[ao.2 as usize];
    //let ao4 = AO_CURVE[ao.3 as usize];
    
    let sl1 = LIGHT_CURVE[sl.0 as usize];
    let sl2 = LIGHT_CURVE[sl.1 as usize];
    let sl3 = LIGHT_CURVE[sl.2 as usize];
    let sl4 = LIGHT_CURVE[sl.3 as usize];
    
    //let l = LIGHT_CURVE[l as usize];
    let nl = NORMAL_LIGHT[face as usize];

    let fl1 = nl * sl1; // * ao1;
    let fl2 = nl * sl2; // * ao2;
    let fl3 = nl * sl3; // * ao3;
    let fl4 = nl * sl4; // * ao4;

    let uv1 = model.uvs[uvo + 0];
    let uv2 = model.uvs[uvo + 1];
    let uv3 = model.uvs[uvo + 2];
    let uv4 = model.uvs[uvo + 3];

    let flipped = sl.4;

    if !flipped {
        // triangle 1 (lower)
        add_vertex(mesh, x, y, z, uv1, uv2, fl4, start + 0); // left bottom corner
        add_vertex(mesh, x, y, z, uv3, uv2, fl1, start + 1); // right bottom corner
        add_vertex(mesh, x, y, z, uv3, uv4, fl2, start + 2); // right top corner
        // triangle 2 (upper)
        add_vertex(mesh, x, y, z, uv1, uv2, fl4, start + 3); // left bottom corner
        add_vertex(mesh, x, y, z, uv3, uv4, fl2, start + 4); // right top corner
        add_vertex(mesh, x, y, z, uv1, uv4, fl3, start + 5); // left top corner
    } else {
        // triangle 1 (lower)
        add_vertex(mesh, x, y, z, uv1, uv4, fl3, start + 5); // left top corner
        add_vertex(mesh, x, y, z, uv1, uv2, fl4, start + 3); // left bottom corner
        add_vertex(mesh, x, y, z, uv3, uv2, fl1, start + 1); // right bottom corner
        // triangle 2 (upper)
        add_vertex(mesh, x, y, z, uv3, uv4, fl2, start + 2); // right top corner
        add_vertex(mesh, x, y, z, uv1, uv4, fl3, start + 5); // left top corner
        add_vertex(mesh, x, y, z, uv3, uv2, fl1, start + 1); // right bottom corner
    }
}

#[inline(always)]
fn add_vertex(
    mesh: &mut Vec<f32>,
    x: i32,
    y: i32,
    z: i32,
    u: f32,
    v: f32,
    light: f32,
    vertex: u32,
) {
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


#[inline(always)]
fn interpolate_light(
    l1: f32,
    l2: f32,
    l3: f32,
    l4: f32,
    u: f32,
    v: f32,
) -> f32 {
    let a = l4 + (l1 - l4) * u;
    let b = l3 + (l2 - l3) * u;

    a + (b - a) * v
}

#[inline(always)]
fn add_face_cube_mesh(
    mesh: &mut Vec<f32>,
    world_face: u32,
    local_face: u32,
    world_face_index: u32,
    local_face_index: u32,
    rotation: u32,
    x: i32,
    y: i32,
    z: i32,
    block: Blocktype,
    model: &BlockModel,
    chunk: &Chunk,
    region: &ChunkRegion,
) {
    let vert_offset = local_face_index * 6;
    let uv_offset = (local_face_index * 4) as usize;

    let sl = get_smooth_light(chunk, region, x, y, z, world_face);

    let rotated_triangles = sl.4;
    
    let sl1 = LIGHT_CURVE[sl.0 as usize];
    let sl2 = LIGHT_CURVE[sl.1 as usize];
    let sl3 = LIGHT_CURVE[sl.2 as usize];
    let sl4 = LIGHT_CURVE[sl.3 as usize];
    
    let nl = NORMAL_LIGHT[world_face as usize];

    let uv1 = model.uvs[uv_offset + 0];
    let uv2 = model.uvs[uv_offset + 1];
    let uv3 = model.uvs[uv_offset + 2];
    let uv4 = model.uvs[uv_offset + 3];

    let fl1 = nl * sl1;
    let fl2 = nl * sl2;
    let fl3 = nl * sl3;
    let fl4 = nl * sl4;

    if !rotated_triangles {
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv2, fl4, vert_offset + 0, rotation, model, world_face, fl1, fl2, fl3, fl4); // left bottom corner
        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv2, fl1, vert_offset + 1, rotation, model, world_face, fl1, fl2, fl3, fl4); // right bottom corner
        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv4, fl2, vert_offset + 2, rotation, model, world_face, fl1, fl2, fl3, fl4); // right top corner
        
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv2, fl4, vert_offset + 3, rotation, model, world_face, fl1, fl2, fl3, fl4); // left bottom corner
        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv4, fl2, vert_offset + 4, rotation, model, world_face, fl1, fl2, fl3, fl4); // right top corner
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv4, fl3, vert_offset + 5, rotation, model, world_face, fl1, fl2, fl3, fl4); // left top corner
    } else {
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv4, fl3, vert_offset + 5, rotation, model, world_face, fl1, fl2, fl3, fl4); // left top corner
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv2, fl4, vert_offset + 3, rotation, model, world_face, fl1, fl2, fl3, fl4); // left bottom corner
        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv2, fl1, vert_offset + 1, rotation, model, world_face, fl1, fl2, fl3, fl4); // right bottom corner

        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv4, fl2, vert_offset + 2, rotation, model, world_face, fl1, fl2, fl3, fl4); // right top corner
        add_vertex_cube_mesh(mesh, x, y, z, uv1, uv4, fl3, vert_offset + 5, rotation, model, world_face, fl1, fl2, fl3, fl4); // left top corner
        add_vertex_cube_mesh(mesh, x, y, z, uv3, uv2, fl1, vert_offset + 1, rotation, model, world_face, fl1, fl2, fl3, fl4); // right bottom corner
    }
}

#[inline(always)]
fn add_vertex_cube_mesh(
    mesh: &mut Vec<f32>,
    x: i32,
    y: i32,
    z: i32,
    u: f32,
    v: f32,
    light: f32,
    vertex: u32,
    rotation: u32,
    model: &BlockModel,
    face: u32,
    l1: f32,
    l2: f32,
    l3: f32,
    l4: f32,
) {
    let vert_sub_pos = (vertex * 3) as usize;

    let vert_x = model.verts[vert_sub_pos + 0];
    let vert_y = model.verts[vert_sub_pos + 1];
    let vert_z = model.verts[vert_sub_pos + 2];

    let rotated = chunk_mesher::rotate_vertex(vert_x, vert_z, rotation);
    let vert_x = rotated.0;
    let vert_z = rotated.1;

    let final_light = match face {
        FACE_RIGHT_ => interpolate_light(l1, l2, l3, l4, vert_z, vert_y),
        FACE_LEFT_ => interpolate_light(l1, l2, l3, l4, vert_z, vert_y),
        FACE_TOP_ => interpolate_light(l1, l2, l3, l4, vert_x, vert_z),
        FACE_BOTTOM_ => interpolate_light(l1, l2, l3, l4, vert_x, vert_z),
        FACE_FRONT_ => interpolate_light(l1, l2, l3, l4, vert_x, vert_y),
        FACE_BACK_ => interpolate_light(l1, l2, l3, l4, vert_x, vert_y),
        _ => 0.0,
    };

    // position
    mesh.push(vert_x + x as f32);
    mesh.push(vert_y + y as f32);
    mesh.push(vert_z + z as f32);
    // uv
    mesh.push(u);
    mesh.push(v);
    // light
    mesh.push(final_light);
}

#[inline(always)]
fn rotate_vertex(x: f32, z: f32, rotation: u32) -> (f32, f32) {
    let deg_0 = 0;
    let deg_90 = 1;
    let deg_180 = 2;
    let deg_270 = 3;

    if rotation == deg_90 {
        return (-z + 1.0, x);
    }
    if rotation == deg_180 {
        return (-x + 1.0, -z + 1.0);
    }
    if rotation == deg_270 {
        return (z, -x + 1.0);
    }

    (x, z)
}

#[inline(always)]
fn get_rotated_face_id(face: u32, rotation: u32) -> u32 {
    match rotation {
        0 => { face } // 0

        1 => { // 90
            match face {
                FACE_RIGHT_ => FACE_FRONT_,
                FACE_LEFT_ => FACE_BACK_,
                FACE_FRONT_ => FACE_LEFT_,
                FACE_BACK_ => FACE_RIGHT_,
                _ => face,
            }
        }
        2 => { // 180
            match face {
                FACE_RIGHT_ => FACE_LEFT_,
                FACE_LEFT_ => FACE_RIGHT_,
                FACE_FRONT_ => FACE_BACK_,
                FACE_BACK_ => FACE_FRONT_,
                _ => face,
            }
        }
        3 => { // 270
            match face {
                FACE_RIGHT_ => FACE_BACK_,
                FACE_LEFT_ => FACE_FRONT_,
                FACE_FRONT_ => FACE_RIGHT_,
                FACE_BACK_ => FACE_LEFT_,
                _ => face,
            }
        }
        _ => face,
    }
}



#[inline]
fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u32 {
    if side1 && side2 {
        return 0;
    }

    3 - (if side1 { 1 } else { 0 } + if side2 { 1 } else { 0 } + if corner { 1 } else { 0 })
}

fn get_ambient_occlusion(
    chunk: &Chunk,
    region: &ChunkRegion,
    x: i32,
    y: i32,
    z: i32,
    face: u32,
) -> (u32, u32, u32, u32, bool) {
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
        0 => {
            // x+ face
            b1 = chunk.get_block_global(region, x + 1, y - 1, z + 0) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x + 1, y - 1, z - 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x + 1, y + 0, z - 1) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x + 1, y + 1, z - 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x + 1, y + 1, z + 0) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x + 1, y + 1, z + 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x + 1, y + 0, z + 1) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x + 1, y - 1, z + 1) != Blocktype::Air;
        }
        1 => {
            // x- face
            b1 = chunk.get_block_global(region, x - 1, y - 1, z + 0) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x - 1, y - 1, z + 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x - 1, y + 0, z + 1) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x - 1, y + 1, z + 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x - 1, y + 1, z + 0) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x - 1, y + 1, z - 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x - 1, y + 0, z - 1) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x - 1, y - 1, z - 1) != Blocktype::Air;
        }
        2 => {
            // y+ face
            b1 = chunk.get_block_global(region, x + 0, y + 1, z + 1) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x + 1, y + 1, z + 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x + 1, y + 1, z + 0) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x + 1, y + 1, z - 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x + 0, y + 1, z - 1) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x - 1, y + 1, z - 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x - 1, y + 1, z + 0) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x - 1, y + 1, z + 1) != Blocktype::Air;
        }
        3 => {
            // -y face
            b1 = chunk.get_block_global(region, x - 0, y - 1, z - 1) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x + 1, y - 1, z - 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x + 1, y - 1, z + 0) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x + 1, y - 1, z + 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x - 0, y - 1, z + 1) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x - 1, y - 1, z + 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x - 1, y - 1, z + 0) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x - 1, y - 1, z - 1) != Blocktype::Air;
        }
        4 => {
            // z+ face
            b1 = chunk.get_block_global(region, x + 0, y - 1, z + 1) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x + 1, y - 1, z + 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x + 1, y + 0, z + 1) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x + 1, y + 1, z + 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x + 0, y + 1, z + 1) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x - 1, y + 1, z + 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x - 1, y + 0, z + 1) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x - 1, y - 1, z + 1) != Blocktype::Air;
        }
        5 => {
            // z- face
            b1 = chunk.get_block_global(region, x + 0, y - 1, z - 1) != Blocktype::Air;
            b2 = chunk.get_block_global(region, x - 1, y - 1, z - 1) != Blocktype::Air;
            b3 = chunk.get_block_global(region, x - 1, y + 0, z - 1) != Blocktype::Air;
            b4 = chunk.get_block_global(region, x - 1, y + 1, z - 1) != Blocktype::Air;
            b5 = chunk.get_block_global(region, x + 0, y + 1, z - 1) != Blocktype::Air;
            b6 = chunk.get_block_global(region, x + 1, y + 1, z - 1) != Blocktype::Air;
            b7 = chunk.get_block_global(region, x + 1, y + 0, z - 1) != Blocktype::Air;
            b8 = chunk.get_block_global(region, x + 1, y - 1, z - 1) != Blocktype::Air;
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

    let flipped = (ao1 + ao3 > ao2 + ao4);

    (ao1, ao2, ao3, ao4, flipped)
}

#[inline]
fn vertex_smthl(fl: u32, side1: u32, side2: u32, corner: u32) -> u32 {
    (fl + side1 + side2 + corner) / 4
}

fn get_smooth_light(
    chunk: &Chunk,
    region: &ChunkRegion,
    x: i32,
    y: i32,
    z: i32,
    face: u32,
) -> (u32, u32, u32, u32, bool) {

    let b1: u32;
    let b2: u32;
    let b3: u32;
    let b4: u32;
    let b5: u32;
    let b6: u32;
    let b7: u32;
    let b8: u32;

    match face {
        0 => {
            // x+ face
            b1 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z + 0);
            b2 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z - 1);
            b3 = get_light_or_occ_global(chunk, region, x + 1, y + 0, z - 1);
            b4 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z - 1);
            b5 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z + 0);
            b6 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z + 1);
            b7 = get_light_or_occ_global(chunk, region, x + 1, y + 0, z + 1);
            b8 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z + 1);
        }
        1 => {
            // x- face
            b1 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z + 0);
            b2 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z + 1);
            b3 = get_light_or_occ_global(chunk, region, x - 1, y + 0, z + 1);
            b4 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z + 1);
            b5 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z + 0);
            b6 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z - 1);
            b7 = get_light_or_occ_global(chunk, region, x - 1, y + 0, z - 1);
            b8 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z - 1);
        }
        2 => {
            // y+ face
            b1 = get_light_or_occ_global(chunk, region, x + 0, y + 1, z + 1);
            b2 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z + 1);
            b3 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z + 0);
            b4 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z - 1);
            b5 = get_light_or_occ_global(chunk, region, x + 0, y + 1, z - 1);
            b6 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z - 1);
            b7 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z + 0);
            b8 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z + 1);
        }
        3 => {
            // -y face
            b1 = get_light_or_occ_global(chunk, region, x - 0, y - 1, z - 1);
            b2 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z - 1);
            b3 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z + 0);
            b4 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z + 1);
            b5 = get_light_or_occ_global(chunk, region, x - 0, y - 1, z + 1);
            b6 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z + 1);
            b7 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z + 0);
            b8 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z - 1);
        }
        4 => {
            // z+ face
            b1 = get_light_or_occ_global(chunk, region, x + 0, y - 1, z + 1);
            b2 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z + 1);
            b3 = get_light_or_occ_global(chunk, region, x + 1, y + 0, z + 1);
            b4 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z + 1);
            b5 = get_light_or_occ_global(chunk, region, x + 0, y + 1, z + 1);
            b6 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z + 1);
            b7 = get_light_or_occ_global(chunk, region, x - 1, y + 0, z + 1);
            b8 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z + 1);
        }
        5 => {
            // z- face
            b1 = get_light_or_occ_global(chunk, region, x + 0, y - 1, z - 1);
            b2 = get_light_or_occ_global(chunk, region, x - 1, y - 1, z - 1);
            b3 = get_light_or_occ_global(chunk, region, x - 1, y + 0, z - 1);
            b4 = get_light_or_occ_global(chunk, region, x - 1, y + 1, z - 1);
            b5 = get_light_or_occ_global(chunk, region, x + 0, y + 1, z - 1);
            b6 = get_light_or_occ_global(chunk, region, x + 1, y + 1, z - 1);
            b7 = get_light_or_occ_global(chunk, region, x + 1, y + 0, z - 1);
            b8 = get_light_or_occ_global(chunk, region, x + 1, y - 1, z - 1);
        }
        // just to keep the compiler happy! :)
        _ => {
            b1 = 0;
            b2 = 0;
            b3 = 0;
            b4 = 0;
            b5 = 0;
            b6 = 0;
            b7 = 0;
            b8 = 0;
        }
    }

    let fl = get_face_light(chunk, region, x, y, z, face);

    let sl1 = vertex_smthl(fl, b1, b3, b2); // right bottom
    let sl2 = vertex_smthl(fl, b3, b5, b4); // right top
    let sl3 = vertex_smthl(fl, b5, b7, b6); // left top
    let sl4 = vertex_smthl(fl, b7, b1, b8); // left bottom

    let flipped = (sl1 + sl3 > sl2 + sl4);

    (sl1, sl2, sl3, sl4, flipped)
}
// returns the light or a custom light value if the block is occluded
fn get_light_or_occ_global(chunk: &Chunk, region: &ChunkRegion, lx: i32, ly: i32, lz: i32) -> u32 {
    if chunk.get_block_global(region, lx, ly, lz) != Blocktype::Air {
        return 8
    }
    chunk.get_light_global(region, lx, ly, lz)
}

#[inline]
fn get_face_light(
    chunk: &Chunk,
    region: &ChunkRegion,
    x: i32,
    y: i32,
    z: i32,
    face: u32,
) -> u32 {
    match face {
        0 => {chunk.get_light_global(region, x + 1, y, z)}
        1 => {chunk.get_light_global(region, x - 1, y, z)}
        2 => {chunk.get_light_global(region, x, y + 1, z)}
        3 => {chunk.get_light_global(region, x, y - 1, z)}
        4 => {chunk.get_light_global(region, x, y, z + 1)}
        5 => {chunk.get_light_global(region, x, y, z - 1)}
        _ => {0}
    }
}