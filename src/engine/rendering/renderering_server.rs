use std::ops::Add;

use glam::{Mat4, Quat, Vec3};
use glfw::MouseButton;

use crate::engine::{
    blocks::block_model::BlockModel, core::{input::Input, window::GameWindow}, entities::character_controller::CharacterState, gui::{
        elements::{button::UIButton, check_box::UICheckBox, label::UILabel},
        basic_ui::UIElement,
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
        window: &GameWindow,
    ) {

        self.render_solid(resource_manager, camera, world);
        self.render_transparent(resource_manager, camera, character_controller, vao_debug);

        unsafe {

            if menu_manager.selected_menu == Menu::Closed || menu_manager.selected_menu == Menu::Pause {
                // HAND BLOCK

                let slot = character_controller.selected_slot as usize;
                let item_slot = if slot < resource_manager.item_meshes.len() {slot} else {0};
                
                let block_mesh = &resource_manager.block_meshes[slot];
                let item_mesh = &resource_manager.item_meshes[item_slot];
                //gl::Disable(gl::DEPTH_TEST);
                gl::Disable(gl::CULL_FACE);

                resource_manager.base_shader.bind();
                resource_manager.tile_atlas.bind(0);
                //resource_manager.item_atlas.bind(0);

                let view =  Mat4::IDENTITY;
                let model = Mat4::from_scale_rotation_translation(
                    Vec3::splat(0.6), // scale
                    Quat::from_euler(glam::EulerRot::XYZ, (15.0_f32).to_radians(), 45.0_f32.to_radians(), 0.0), // rotation
                    Vec3::new(1.0, -0.6, -1.3), // position
                );

                gl::UniformMatrix4fv(self.base_view_loc, 1, gl::FALSE, view.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(self.base_proj_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(self.base_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());

                gl::BindVertexArray(block_mesh.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, block_mesh.vertex_count as i32);


                // 3D Item
                resource_manager.item_atlas.bind(0);
                gl::BindVertexArray(item_mesh.vao);

                let model = Mat4::from_scale_rotation_translation(
                    Vec3::splat(2.0), // scale
                    Quat::IDENTITY,
                    Vec3::new(0.0, 80.0, 0.0), // position
                );
                gl::UniformMatrix4fv(self.base_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(self.base_view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
                gl::DrawArrays(gl::TRIANGLES, 0, item_mesh.vertex_count as i32);


                //gl::Enable(gl::DEPTH_TEST);
                gl::Enable(gl::CULL_FACE);
            }

            // menus
            gl::Disable(gl::DEPTH_TEST);
            resource_manager.ui_shader.bind();
            gl::UniformMatrix4fv(
                self.ui_proj_loc,
                1,
                gl::FALSE,
                ui_proj.to_cols_array().as_ptr(),
            );
            resource_manager.ui_tex.bind(0);
            menu_manager.render(resource_manager, world.is_some());
            resource_manager.font_tex.bind(0);

            gl::Enable(gl::DEPTH_TEST);


            if menu_manager.selected_menu == Menu::Closed || menu_manager.selected_menu == Menu::Pause {
                // hotbar items
                gl::Disable(gl::DEPTH_TEST);
                resource_manager.base_shader.bind();
                resource_manager.tile_atlas.bind(0);

                let slot_size = 90; // 80
                let slot_half = (slot_size / 2) as f32;
                
                let view = glam::camera::rh::view::look_at_mat4( //Mat4::IDENTITY.to_cols_array().as_ptr()
                    Vec3::new(0.0, 0.0, 10.0), // camera position
                    Vec3::ZERO, // target
                    Vec3::Y, // up
                );
                
                gl::UniformMatrix4fv(self.base_view_loc, 1, gl::FALSE, view.to_cols_array().as_ptr()); // camera.view
                gl::UniformMatrix4fv(self.base_proj_loc, 1, gl::FALSE, ui_proj.to_cols_array().as_ptr()); // camera.projection

                for slot in 0..10 {
                    let block_mesh = &resource_manager.block_meshes[slot];
                    gl::BindVertexArray(block_mesh.vao);
                    let model = Mat4::from_scale_rotation_translation(
                        Vec3::splat(slot_size as f32 * 0.45), // scale
                        Quat::from_euler(glam::EulerRot::XYZ, (180.0_f32 + 45.0).to_radians(), 45.0_f32.to_radians(), 0.0), // rotation
                        Vec3::new(window.width as f32 / 2.0 - slot_half * 10.0 + slot_half + (slot_size * slot) as f32, window.height as f32 - 10.0 - slot_half, 0.0), // position
                    );
                    gl::UniformMatrix4fv(self.base_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
                    gl::DrawArrays(gl::TRIANGLES, 0, block_mesh.vertex_count as i32);
                }
                gl::Enable(gl::DEPTH_TEST);
            }

        }
    }

    fn render_solid(
        &self,
        resource_manager: &ResourceManager,
        camera: &Camera,
        world: &mut Option<World>,
    ) {
        unsafe {
            resource_manager.chunk_shader.bind();
            resource_manager.tile_atlas.bind(0);

            let view_ptr = camera.view.to_cols_array().as_ptr();
            let proj_ptr = camera.projection.to_cols_array().as_ptr();

            gl::UniformMatrix4fv(self.chunk_view_loc, 1, gl::FALSE, view_ptr);
            gl::UniformMatrix4fv(self.chunk_proj_loc, 1, gl::FALSE, proj_ptr);

            gl::Uniform1i(self.chunk_tex_loc, 0);

            if let Some(world) = world {
                rendering::chunk_renderer::render_chunks(
                    world, self.chunk_model_loc, self.chunk_fade_loc
                );
            }




        }
    }

    fn render_transparent(
        &self,
        resource_manager: &ResourceManager,
        camera: &Camera,
        character_controller: &CharacterState,
        vao_debug: u32,
    ) {
        unsafe {
            // debug wireframe mesh
            
            let target_pos = character_controller.target_block.clone();
            let mut target_pos = Vec3::new(target_pos.x as f32, target_pos.y as f32, target_pos.z as f32);
            target_pos -= Vec3::splat(0.005);
            
            let mut model = Mat4::from_translation(target_pos);
            model *= Mat4::from_scale(Vec3::splat(1.02));
            
            resource_manager.wireframe_shader.bind();

            gl::Disable(gl::CULL_FACE);
            
            gl::Uniform4f(self.wire_line_color_loc, 0.0, 0.0, 0.0, 1.0);
            gl::Uniform4f(self.wire_face_color_loc, 1.0, 1.0, 1.0, 0.15);
            gl::Uniform1f(self.wire_line_width_loc, 0.02);
            
            gl::UniformMatrix4fv(self.wire_model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.wire_view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(self.wire_proj_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());
            
            gl::BindVertexArray(vao_debug);
            gl::DrawArrays(gl::TRIANGLES, 0, crate::VERTICES.len() as i32);

            gl::Enable(gl::CULL_FACE);
        }
    }

    fn render_ui() {
        
    }
}
