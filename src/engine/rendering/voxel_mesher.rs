use glam::Vec3;
use image::{ImageBuffer, Rgba};

use crate::engine::{rendering::{chunk_mesher, mesh::BasicMesh}, resources::texture::{TextureAtlas, UV}};


pub fn voxelize_atlas(tex_atlas: &TextureAtlas) -> Vec<BasicMesh> {
    let mut items: Vec<BasicMesh> = Vec::new();
    for tile in &tex_atlas.tiles {
        println!("{}", tile.name);
        let voxel_size = 1.0 / 16.0;
        let uv = UV::new(tile.x, tile.y, tile.xm, tile.ym);
        let mut vertex_data: Vec<f32> = Vec::new();

        add_face(&mut vertex_data, &uv, 2, crate::engine::rendering::mesh::VERTICES); // top
        add_face(&mut vertex_data, &uv, 2, create_voxel_verts(Vec3::new(0.0, voxel_size * 15.0, 0.0), Vec3::new(1.0, voxel_size * 15.0, 1.0))); // bottom

        let atlas_pixel_width = (tex_atlas.tiles_per_row * 16) as f32;

        // turns uv aspect 0.0..1.0 to pixel coords
        let px = (uv.x * atlas_pixel_width) as u32;
        let py = (uv.y * atlas_pixel_width) as u32;
        let px_max = (uv.x_max * atlas_pixel_width) as u32;
        let py_max = (uv.y_max * atlas_pixel_width) as u32;

        if let Some(image_buffer) = &tex_atlas.image_buffer {

            for x in 0..16 {
                for z in 0..16 {
                    // scans one direction of the image
                    // and adds faces for every pixel side
                    let u = px + x;
                    let v = py + z;

                    let pixel = image_buffer.get_pixel(u, v);
                    let alpha = pixel[3];

                    let fx = x; // 15 - x;
                    let fz = 15 - z;

                    let tile_size = 16.0;

                    let voxel_min = Vec3::new((fx) as f32 / tile_size, voxel_size * 15.0, (fz) as f32 / tile_size);
                    let voxel_max = Vec3::new((fx + 1) as f32 / tile_size, voxel_size * 16.0, (fz + 1) as f32 / tile_size);
                    // uv
                    let pixel_u = u as f32 / atlas_pixel_width;
                    let pixel_v = v as f32 / atlas_pixel_width;
                    let pixel_u_max = (u + 1) as f32 / atlas_pixel_width;
                    let pixel_v_max = (v + 1) as f32 / atlas_pixel_width;

                    let pixel_uv = UV::new(
                        pixel_u,
                        pixel_v,
                        pixel_u_max,
                        pixel_v_max,
                    );

                    if alpha != 0 {
                        if is_cutout(u as i32 + 1, v as i32, &image_buffer) {
                            add_face(&mut vertex_data, &pixel_uv, 0, create_voxel_verts(voxel_min, voxel_max));
                        }
                        if is_cutout(u as i32 - 1, v as i32, &image_buffer) {
                            add_face(&mut vertex_data, &pixel_uv, 1, create_voxel_verts(voxel_min, voxel_max));
                        }
                        if is_cutout(u as i32, v as i32 + 1, &image_buffer) {
                            add_face(&mut vertex_data, &pixel_uv, 5, create_voxel_verts(voxel_min, voxel_max));
                        }
                        if is_cutout(u as i32, v as i32 - 1, &image_buffer) {
                            add_face(&mut vertex_data, &pixel_uv, 4, create_voxel_verts(voxel_min, voxel_max));
                        }
                    }
                }
            }
        } else {println!("couldn't voxelize item meshes")}

        items.push(BasicMesh::from_vertex_data(&vertex_data));
    }
    items
}

#[inline]
fn is_cutout(x: i32, y: i32, image_buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> bool {
    if x < 0
    || y < 0
    || x >= image_buffer.width() as i32
    || y >= image_buffer.height() as i32 {
        return true;
    } else {
        return image_buffer.get_pixel(x as u32, y as u32)[3] == 0;
    }
}

fn add_face(mesh: &mut Vec<f32>, uv: &UV, face: u32, voxel_data: [f32; 108]) {
    let start = 6 * face; // 6 vertices

    let nl = super::chunk_mesher::NORMAL_LIGHT[face as usize];

    // triangle 1 (lower)
    // left bottom corner
    add_vertex(mesh, uv.x, uv.y, nl, start + 0, voxel_data);
    // right bottom corner
    add_vertex(mesh, uv.x_max, uv.y, nl, start + 1, voxel_data);
    // right top corner
    add_vertex(mesh, uv.x_max, uv.y_max, nl, start + 2, voxel_data);

    // triangle 2 (upper)
    // left bottom corner
    add_vertex(mesh, uv.x, uv.y, nl, start + 3, voxel_data);
    // right top corner
    add_vertex(mesh, uv.x_max, uv.y_max, nl, start + 4, voxel_data);
    // left top corner
    add_vertex(mesh, uv.x, uv.y_max, nl, start + 5, voxel_data);
}

fn add_vertex(mesh: &mut Vec<f32>, u: f32, v: f32, light: f32, vertex: u32, voxel_data: [f32; 108]) {
    let vertex_sub_pos = (vertex * 3) as usize;
    // position
    mesh.push(voxel_data[vertex_sub_pos] - 0.5);
    mesh.push(voxel_data[vertex_sub_pos + 1] - 0.5);
    mesh.push(voxel_data[vertex_sub_pos + 2] - 0.5);
    // uv
    mesh.push(u);
    mesh.push(v);
    // light
    mesh.push(light);
}

fn create_voxel_verts(min_corner: Vec3, max_corner: Vec3) -> [f32; 108] {
    let x = min_corner.x;
    let y = min_corner.y;
    let z = min_corner.z;

    let xm = max_corner.x;
    let ym = max_corner.y;
    let zm = max_corner.z;

    
    [
    // right
    xm, y, zm,
    xm, y, z,
    xm, ym, z,

    xm, y, zm,
    xm, ym, z,
    xm, ym, zm,

    // left
    x, y, z,
    x, y, zm,
    x, ym, zm,

    x, y, z,
    x, ym, zm,
    x, ym, z,

    // top
    x, ym, zm,
    xm, ym, zm,
    xm, ym, z,

    x, ym, zm,
    xm, ym, z,
    x, ym, z,

    // bottom
    x, y, z,
    xm, y, z,
    xm, y, zm,

    x, y, z,
    xm, y, zm,
    x, y, zm,

    // front
    x, y, zm,
    xm, y, zm,
    xm, ym, zm,

    x, y, zm,
    xm, ym, zm,
    x, ym, zm,

    // back
    xm, y, z,
    x, y, z,
    x, ym, z,

    xm, y, z,
    x, ym, z,
    xm, ym, z
    ]
}