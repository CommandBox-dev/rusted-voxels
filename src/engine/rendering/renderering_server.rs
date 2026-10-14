use glam::Mat4;
use glfw::MouseButton;

use crate::engine::{
    core::{input::Input, window::GameWindow},
    entities::character_controller::CharacterState,
    gui::{
        basic_ui::{UIButton, UICheckBox, UIElement, UILabel},
        menu_manager::{self, Menu, MenuManager},
    },
    rendering::{self, camera::Camera},
    resources::{
        resource_manager::{self, ResourceManager},
        texture::*,
    },
    world::world::World,
};

pub fn upload_crosshair(tile: &AtlasTile, window: &GameWindow) -> (u32, u32) {
    let size: i32 = 40;
    let x: f32 = (window.fb_width / 2 - size / 2) as f32;
    let y: f32 = (window.fb_height / 2 - size / 2) as f32;
    let w: f32 = size as f32;
    let h: f32 = size as f32;

    let quad_mesh: [f32; 24] = [
        x + w,
        y + h,
        tile.xm,
        tile.ym,
        x + w,
        y,
        tile.xm,
        tile.y,
        x,
        y,
        tile.x,
        tile.y,
        x,
        y,
        tile.xm,
        tile.ym,
        x,
        y + h,
        tile.x,
        tile.ym,
        x + w,
        y + h,
        tile.x,
        tile.y,
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

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_mesh.len() * float_size as usize) as isize,
            quad_mesh.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

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
            (2 * float_size) as *const _,
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

pub struct RenderingServer {
    // --- shader uniform locations ---
    // base shader
    base_model_loc: i32,
    base_view_loc: i32,
    base_proj_loc: i32,
    base_tex_loc: i32,
    // ui shader
    ui_proj_loc: i32,
    ui_tex_loc: i32,
}

impl RenderingServer {
    pub fn new() -> Self {
        Self {
            base_model_loc: 0,
            base_view_loc: 0,
            base_proj_loc: 0,
            base_tex_loc: 0,

            ui_proj_loc: 0,
            ui_tex_loc: 0,
        }
    }

    pub fn update_shader_unif_loc(&mut self, resource_manager: &ResourceManager) {
        self.base_model_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_model".as_ptr());
        self.base_view_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_view".as_ptr());
        self.base_proj_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_projection".as_ptr());
        self.base_tex_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_texture".as_ptr());

        self.ui_proj_loc = resource_manager
            .ui_shader
            .get_unif_loc(c"u_projection".as_ptr());
        self.ui_tex_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_texture".as_ptr());
    }

    pub fn render(
        &self,
        resource_manager: &ResourceManager,
        camera: &Camera,
        character_controller: &CharacterState,
        world: &mut Option<World>,
        ui_proj: &Mat4,
        vao_crosshair: u32,
        input: &Input,
        label: &UILabel,
        chat_label: &UILabel,
        menu_manager: &MenuManager,
    ) {
        // TODO: create player character that stores player position and move camera into RenderingServer
        // TODO: fix horrible parameter bloat
        unsafe {
            //gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            //gl::UseProgram(resource_manager.base_shader);

            resource_manager.tile_atlas.bind(0);
            gl::UniformMatrix4fv(
                self.base_view_loc,
                1,
                gl::FALSE,
                camera.view.to_cols_array().as_ptr(),
            );
            gl::UniformMatrix4fv(
                self.base_proj_loc,
                1,
                gl::FALSE,
                camera.projection.to_cols_array().as_ptr(),
            );

            gl::Uniform1i(self.base_tex_loc, 0);

            if let Some(world) = world {
                rendering::chunk_renderer::render_chunks(world, self.base_model_loc);
            }

            gl::Disable(gl::DEPTH_TEST);
            resource_manager.ui_shader.bind();
            gl::UniformMatrix4fv(
                self.ui_proj_loc,
                1,
                gl::FALSE,
                ui_proj.to_cols_array().as_ptr(),
            );
            resource_manager.ui_tex.bind(0);
            menu_manager.render(resource_manager);

            if world.is_some() && menu_manager.selected_menu == Menu::Closed {
                resource_manager.tile_atlas.bind(0);
                gl::BlendFunc(gl::ONE_MINUS_DST_COLOR, gl::ZERO);
                render_crosshair(vao_crosshair);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }
            resource_manager.font_tex.bind(0);
            label.draw();
            chat_label.draw();
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}
