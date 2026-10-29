use glam::Mat4;

use crate::engine::blocks::block_model::BlockModel;


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
        let uvs = block_model.uvs;

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
}