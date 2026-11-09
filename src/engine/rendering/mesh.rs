use std::ops::Div;

use glam::{Mat4, Vec3};

use crate::engine::{block::block_model::BlockModel, physic::aabb::AABB};


pub const VERTICES: [f32; 108] = [
    // right
    1.0, 0.0, 1.0,
    1.0, 0.0, 0.0,
    1.0, 1.0, 0.0,

    1.0, 0.0, 1.0,
    1.0, 1.0, 0.0,
    1.0, 1.0, 1.0,

    // left
    0.0, 0.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, 1.0, 1.0,

    0.0, 0.0, 0.0,
    0.0, 1.0, 1.0,
    0.0, 1.0, 0.0,

    // top
    0.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0, 0.0,

    0.0, 1.0, 1.0,
    1.0, 1.0, 0.0,
    0.0, 1.0, 0.0,

    // bottom
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 0.0, 1.0,

    0.0, 0.0, 0.0,
    1.0, 0.0, 1.0,
    0.0, 0.0, 1.0,

    // front
    0.0, 0.0, 1.0,
    1.0, 0.0, 1.0,
    1.0, 1.0, 1.0,

    0.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
    0.0, 1.0, 1.0,

    // back
    1.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 1.0, 0.0,

    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    1.0, 1.0, 0.0,
];

pub const VERTICES_UV: [f32; 108 + 72] = [
    // right
    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 0.0, 1.0,

    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,

    // left
    0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 0.0, 1.0,

    // top
    0.0, 1.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,

    0.0, 1.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 0.0, 1.0,

    // bottom
    0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 0.0, 0.0, 1.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 1.0,
    0.0, 0.0, 1.0, 0.0, 1.0,

    // front
    0.0, 0.0, 1.0, 0.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 1.0, 0.0, 1.0,

    // back
    1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 1.0, 1.0,

    1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 0.0, 1.0,
];

// There should be multiple mesh types like translucent mesh, chunk mesh...
pub trait Mesh {
    fn upload();

    fn destroy();

    fn replace();
    
    fn render();
}

pub struct BasicMesh {
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: u32,
}

pub struct BasicModel {
    mesh: BasicMesh,
    pub model_matrix: Mat4,
}

impl BasicMesh {
    pub fn from_block_model(block_model: &BlockModel) -> Self {
        let mut vertex_data: Vec<f32> = Vec::new();
        let uvs = &block_model.uvs;

        for face in 0..6 {
            BasicMesh::add_face(&mut vertex_data, &uvs, face);
        }

        let (vao, vbo) = BasicMesh::upload_mesh(&vertex_data);
        let vertex_count = (vertex_data.len() / 6) as u32;

        Self {
            vao,
            vbo,
            vertex_count,
        }
    }

    pub fn from_vertex_data(vertex_data: &Vec<f32>) -> Self {
        let vertex_count = (vertex_data.len() / 6) as u32;
        let (vao, vbo) = BasicMesh::upload_mesh(&vertex_data);

        Self {
            vao,
            vbo,
            vertex_count,
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
        }
    }

    pub fn new_cube_pos_uv() -> Self {

        let mut vao = 0;
        let mut vbo = 0;
        let float_size = std::mem::size_of::<f32>();
        let mesh_byte_size = (VERTICES_UV.len() as usize * float_size) as isize;

        let stride = 5 * float_size as i32;

        unsafe {

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                mesh_byte_size,
                VERTICES_UV.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * float_size) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            vertex_count: (VERTICES_UV.len() / 5) as u32,
        }
    }

    pub fn from_aabb(aabbs: &[AABB]) -> Self {
        let mut verts = Vec::new();

        for cube in aabbs {
            verts.extend_from_slice(&Self::get_cube_verts(&cube.min.clone().div(16.0), &cube.max.clone().div(16.0)));
        };

        let mut vao = 0;
        let mut vbo = 0;
        let float_size = std::mem::size_of::<f32>();
        let mesh_byte_size = (verts.len() as usize * float_size) as isize;

        let stride = 5 * float_size as i32;

        unsafe {

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                mesh_byte_size,
                verts.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * float_size) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            vertex_count: (verts.len() / 5) as u32,
        }
    }
    /// `{pos, uv, light}` mesh
    pub fn upload_mesh(vertex_data: &Vec<f32>) -> (u32, u32) {

        let float_size = std::mem::size_of::<f32>();
        let mesh_byte_len = (vertex_data.len() as usize * float_size) as isize;
        let stride = 6 * float_size as i32; // pos, uv, light

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER, mesh_byte_len, vertex_data.as_ptr() as *const _, gl::STATIC_DRAW);

            // position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // uv
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * float_size) as *const _);
            gl::EnableVertexAttribArray(1);
            // light
            gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, stride, (5 * float_size) as *const _);
            gl::EnableVertexAttribArray(2);

            gl::BindVertexArray(0);
        }
        (vao, vbo)
    }

    fn add_face(mesh: &mut Vec<f32>, uvs: &[f32], face: u32) {
        let start = 6 * face; // 6 vertices
        let uvo = (face * 4) as usize;

        let nl = super::chunk_mesher::NORMAL_LIGHT[face as usize];

        // triangle 1 (lower)
        BasicMesh::add_vertex(mesh, uvs[uvo + 0], uvs[uvo + 1], nl, start + 0); // left bottom corner
        BasicMesh::add_vertex(mesh, uvs[uvo + 2], uvs[uvo + 1], nl, start + 1); // right bottom corner
        BasicMesh::add_vertex(mesh, uvs[uvo + 2], uvs[uvo + 3], nl, start + 2); // right top corner
        // triangle 2 (upper)
        BasicMesh::add_vertex(mesh, uvs[uvo + 0], uvs[uvo + 1], nl, start + 3); // left bottom corner
        BasicMesh::add_vertex(mesh, uvs[uvo + 2], uvs[uvo + 3], nl, start + 4); // right top corner
        BasicMesh::add_vertex(mesh, uvs[uvo + 0], uvs[uvo + 3], nl, start + 5); // left top corner
    }

    fn add_vertex(mesh: &mut Vec<f32>, u: f32, v: f32, light: f32, vertex: u32) {
        let vertex_sub_pos = (vertex * 3) as usize;
        // position
        mesh.push(VERTICES[vertex_sub_pos] - 0.5);
        mesh.push(VERTICES[vertex_sub_pos + 1] - 0.5);
        mesh.push(VERTICES[vertex_sub_pos + 2] - 0.5);
        // uv
        mesh.push(u);
        mesh.push(v);
        // light
        mesh.push(light);
    }

    fn get_cube_verts(min_corner: &Vec3, max_corner: &Vec3) -> Vec<f32> {

        let x = min_corner.x;
        let y = min_corner.y;
        let z = min_corner.z;

        let xm = max_corner.x;
        let ym = max_corner.y;
        let zm = max_corner.z;

        
        /*vec![
            // right
            xm, y, zm,  y, zm,
            xm, y, z,  y, z,
            xm, ym, z,  ym, z,

            xm, y, zm,  y, zm,
            xm, ym, z,  ym, z,
            xm, ym, zm,  ym, zm,

            // left
            x, y, z,  y, z,
            x, y, zm,  y, zm,
            x, ym, zm,  ym, zm,

            x, y, z,  y, z,
            x, ym, zm,  ym, zm,
            x, ym, z,  ym, z,

            // top
            x, ym, zm,  x, zm,
            xm, ym, zm,  xm, zm,
            xm, ym, z,  xm, z,

            x, ym, zm,  x, zm,
            xm, ym, z,  xm, z,
            x, ym, z,  x, z,

            // bottom
            x, y, z,  x, z,
            xm, y, z,  xm, z,
            xm, y, zm,  xm, zm,

            x, y, z,  x, z,
            xm, y, zm,  xm, zm,
            x, y, zm,  x, zm,

            // front
            x, y, zm,  x, y,
            xm, y, zm,  xm, y,
            xm, ym, zm,  xm, ym,

            x, y, zm,  x, y,
            xm, ym, zm,  xm, ym,
            x, ym, zm,  x, ym,

            // back
            xm, y, z,  xm, y,
            x, y, z,  x, y,
            x, ym, z,  x, ym,

            xm, y, z,  xm, y,
            x, ym, z,  x, ym,
            xm, ym, z,  xm, ym,
        ]*/

        vec![
            // right
            xm, y, zm,  0.0, 1.0,
            xm, y, z,  0.0, 0.0,
            xm, ym, z,  1.0, 0.0,

            xm, y, zm,  0.0, 1.0,
            xm, ym, z,  1.0, 0.0,
            xm, ym, zm,  1.0, 1.0,

            // left
            x, y, z,  0.0, 0.0,
            x, y, zm,  0.0, 1.0,
            x, ym, zm,  1.0, 1.0,

            x, y, z,  0.0, 0.0,
            x, ym, zm,  1.0, 1.0,
            x, ym, z,  1.0, 0.0,

            // top
            x, ym, zm,  0.0, 1.0,
            xm, ym, zm,  1.0, 1.0,
            xm, ym, z,  1.0, 0.0,

            x, ym, zm,  0.0, 1.0,
            xm, ym, z,  1.0, 0.0,
            x, ym, z,  0.0, 0.0,

            // bottom
            x, y, z,  0.0, 0.0,
            xm, y, z,  1.0, 0.0,
            xm, y, zm,  1.0, 1.0,

            x, y, z,  0.0, 0.0,
            xm, y, zm,  1.0, 1.0,
            x, y, zm,  0.0, 1.0,

            // front
            x, y, zm,  0.0, 0.0,
            xm, y, zm,  1.0, 0.0,
            xm, ym, zm,  1.0, 1.0,

            x, y, zm,  0.0, 0.0,
            xm, ym, zm,  1.0, 1.0,
            x, ym, zm,  0.0, 1.0,

            // back
            xm, y, z,  1.0, 0.0,
            x, y, z,  0.0, 0.0,
            x, ym, z,  0.0, 1.0,

            xm, y, z,  1.0, 0.0,
            x, ym, z,  0.0, 1.0,
            xm, ym, z,  1.0, 1.0,
        ]
    }
}