use glfw::*;

use glam::{Mat4, Vec2, Vec3};
use crate::engine::{chunk::{self, Chunk}, chunk_renderer};
use crate::engine::{world::{self, World}};


pub fn upload_chunk_mesh(chunk: &mut Chunk, mesh: Vec<f32>) {

    let float_size = std::mem::size_of::<f32>();
    let mesh_byte_size = (mesh.len() as usize * float_size) as isize;
    
    let stride = 6 * float_size as i32; // pos, uv, l

    chunk.vertex_count = (mesh.len() / 6) as u32;

    unsafe {
        
        // delete old mesh if one exists
        if chunk.vao != 0 {gl::DeleteVertexArrays(1, &chunk.vao);}
        if chunk.vbo != 0 {gl::DeleteBuffers(1, &chunk.vbo);}

        gl::GenVertexArrays(1, &mut chunk.vao);
        gl::GenBuffers(1, &mut chunk.vbo);
        
        gl::BindVertexArray(chunk.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, chunk.vbo);

        gl::BufferData(gl::ARRAY_BUFFER, mesh_byte_size, mesh.as_ptr() as *const _, gl::STATIC_DRAW);

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
}

pub fn render_chunk_mesh(chunk: &Chunk, model_loc: i32) {
    unsafe{
        let model = Mat4::from_translation(Vec3::new((chunk.cbx) as f32, 0.0, (chunk.cbz) as f32));
        gl::BindVertexArray(chunk.vao);
        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
        gl::DrawArrays(gl::TRIANGLES, 0, chunk.vertex_count as i32);
    }
}

pub fn render_chunks(world: &mut World, model_loc: i32) {

    for x in 0..world::RENDER_DISTANCE {
        for z in 0..world::RENDER_DISTANCE {

            let chunk = world.get_chunk(x as i32, z as i32);
            if let Some(chunk) = chunk {
                chunk_renderer::render_chunk_mesh(&chunk, model_loc);
            }
        }
    }

    unsafe {gl::BindVertexArray(0);}
}