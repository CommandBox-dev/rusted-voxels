use glam::Mat4;


// There should be multiple mesh types like translucent mesh, chunk mesh...
pub trait Mesh {
    fn upload();

    fn destroy();

    fn replace();
    
    fn render();
}

pub struct BasicMesh {
    vao: u32,
    vbo: u32,
    pub vertex_count: u32,

    pub model_matrix: Mat4,
}