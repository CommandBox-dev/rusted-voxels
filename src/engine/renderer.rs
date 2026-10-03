
use crate::engine::{texture::AtlasTile, window::GameWindow};


pub fn upload_crosshair(tile: &AtlasTile, window: &GameWindow) -> (u32, u32) {

    let size: i32 = 40;
    let x: f32 = (window.fb_width / 2 - size / 2) as f32;
    let y: f32 = (window.fb_height / 2 - size / 2) as f32;
    let w: f32 = size as f32;
    let h: f32 = size as f32;

    let quad_mesh: [f32; 24] = [
        x + w, y + h, tile.xm, tile.ym,
        x + w, y, tile.xm, tile.y,
        x, y, tile.x,  tile.y,

        x, y, tile.xm, tile.ym,
        x, y + h, tile.x,  tile.ym,
        x + w, y + h, tile.x,  tile.y,
    ];

    let vertex_count = (quad_mesh.len() / 4) as u32;

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let float_size = std::mem::size_of::<f32>() as i32;
        let stride = 4 * float_size;

        gl::BufferData(gl::ARRAY_BUFFER, (quad_mesh.len() * float_size as usize) as isize, quad_mesh.as_ptr() as *const _, gl::STATIC_DRAW);

        // position
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // uv
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * float_size) as *const _
        );
        gl::EnableVertexAttribArray(1);
        
        gl::BindVertexArray(0);
    }
    (vao, vbo)
}

pub fn render_crosshair(vao: u32) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        gl::BindVertexArray(0);
    }
}