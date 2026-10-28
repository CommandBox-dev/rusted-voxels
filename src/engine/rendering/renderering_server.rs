use std::ops::Add;

use glam::{Mat4, Vec3};
use glfw::MouseButton;

use crate::engine::{
    blocks::block_model::BlockModel, core::{input::Input, window::GameWindow}, entities::character_controller::CharacterState, gui::{
        basic_ui::{UIButton, UICheckBox, UIElement, UILabel},
        menu_manager::{self, Menu, MenuManager},
    }, rendering::{self, camera::Camera, chunk_mesher}, resources::{
        resource_manager::{self, ResourceManager},
        texture::*,
    }, world::world::World,
};

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
    // chunk shader
    chunk_model_loc: i32,
    chunk_view_loc: i32,
    chunk_proj_loc: i32,
    chunk_tex_loc: i32,
    chunk_fade_loc: i32,
    // wireframe shader
    wire_face_color_loc: i32,
    wire_line_color_loc: i32,
    wire_line_width_loc: i32,
    wire_model_loc: i32,
    wire_view_loc: i32,
    wire_proj_loc: i32,
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

            chunk_model_loc: 0,
            chunk_view_loc: 0,
            chunk_proj_loc: 0,
            chunk_tex_loc: 0,
            chunk_fade_loc: 0,

            wire_face_color_loc: 0,
            wire_line_color_loc: 0,
            wire_line_width_loc: 0,
            wire_model_loc: 0,
            wire_view_loc: 0,
            wire_proj_loc: 0,
        }
    }

    pub fn update_shader_unif_loc(&mut self, resource_manager: &ResourceManager) {

        // base shader
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

        // ui shader
        self.ui_proj_loc = resource_manager
            .ui_shader
            .get_unif_loc(c"u_projection".as_ptr());
        self.ui_tex_loc = resource_manager
            .base_shader
            .get_unif_loc(c"u_texture".as_ptr());

        // chunk shader
        self.chunk_model_loc = resource_manager
            .chunk_shader
            .get_unif_loc(c"u_model".as_ptr());
        self.chunk_view_loc = resource_manager
            .chunk_shader
            .get_unif_loc(c"u_view".as_ptr());
        self.chunk_proj_loc = resource_manager
            .chunk_shader
            .get_unif_loc(c"u_projection".as_ptr());
        self.chunk_tex_loc = resource_manager
            .chunk_shader
            .get_unif_loc(c"u_texture".as_ptr());
        self.chunk_fade_loc = resource_manager
            .chunk_shader
            .get_unif_loc(c"u_fade".as_ptr());

        // wireframe shader
        self.wire_line_color_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_line_color".as_ptr());
        self.wire_face_color_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_face_color".as_ptr());
        self.wire_line_width_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_line_width".as_ptr());
        self.wire_model_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_model".as_ptr());
        self.wire_view_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_view".as_ptr());
        self.wire_proj_loc = resource_manager
            .wireframe_shader
            .get_unif_loc(c"u_projection".as_ptr());
    }

    pub fn render(
        &self,
        resource_manager: &ResourceManager,
        camera: &Camera,
        character_controller: &CharacterState,
        world: &mut Option<World>,
        ui_proj: &Mat4,
        input: &Input,
        menu_manager: &MenuManager,
        vao_debug: u32,
    ) {
        // TODO: create player character that stores player position and move camera into RenderingServer
        // TODO: fix horrible parameter bloat
        resource_manager.chunk_shader.bind();
        
        unsafe {
            //gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            //gl::UseProgram(resource_manager.base_shader);

            resource_manager.tile_atlas.bind(0);
            gl::UniformMatrix4fv(
                self.chunk_view_loc,
                1,
                gl::FALSE,
                camera.view.to_cols_array().as_ptr(),
            );
            gl::UniformMatrix4fv(
                self.chunk_proj_loc,
                1,
                gl::FALSE,
                camera.projection.to_cols_array().as_ptr(),
            );

            gl::Uniform1i(self.chunk_tex_loc, 0);

            if let Some(world) = world {
                rendering::chunk_renderer::render_chunks(world, self.chunk_model_loc, self.chunk_fade_loc);
            }


            let block_mesh = &resource_manager.block_meshes[12];
            let mut model = Mat4::from_translation(Vec3::new(0.0, 150.0, 0.0));
            //model *= Mat4::from_scale(Vec3::splat(1.02));

            resource_manager.base_shader.bind();
            gl::UniformMatrix4fv(self.base_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.base_view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.base_proj_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            gl::BindVertexArray(block_mesh.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, block_mesh.vertex_count as i32);


            gl::Disable(gl::CULL_FACE);
            
            let target_pos = character_controller.target_block.clone();

            let mut target_pos = Vec3::new(target_pos.x as f32, target_pos.y as f32, target_pos.z as f32);
            target_pos -= Vec3::splat(0.005);
            let mut model = Mat4::from_translation(target_pos);
            model *= Mat4::from_scale(Vec3::splat(1.02));
            gl::BindVertexArray(vao_debug);
            resource_manager.wireframe_shader.bind();

            gl::Uniform4f(self.wire_line_color_loc, 0.0, 0.0, 0.0, 1.0);
            gl::Uniform4f(self.wire_face_color_loc, 1.0, 1.0, 1.0, 0.15);
            gl::Uniform1f(self.wire_line_width_loc, 0.02);

            gl::UniformMatrix4fv(self.wire_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.wire_view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.wire_proj_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, crate::VERTICES.len() as i32);


            gl::Enable(gl::CULL_FACE);


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
            resource_manager.font_tex.bind(0);

            gl::Enable(gl::DEPTH_TEST);
        }
    }
}
