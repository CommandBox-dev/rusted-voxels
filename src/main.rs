#![allow(unused)]

mod engine;

use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use glam::{Mat4, Vec2, Vec3, IVec3};
use rand::rand_core::utils::Word;

use crate::engine::character_controller::CharacterState;
use crate::engine::physics::aabb::*;
use crate::engine::input::Input;
use crate::engine::renderer::render_crosshair;
use crate::engine::renderer::upload_crosshair;
use crate::engine::resource_manager::ResourceManager;
use crate::engine::window::GameWindow;
use crate::engine::*;
use crate::engine::chunk::*;
use crate::engine::world::*;
use crate::engine::shader::*;
use crate::engine::texture::*;
use crate::engine::block_model::*;
use crate::engine::physics::*;
use crate::engine::physics::aabb::move_and_collide;
use crate::engine::physics::aabb::*;
use crate::engine::physics::aabb::AABB;

fn main() {
    
    let mut window = GameWindow::new("rusted-voxels", 1920, 1080);
    let resource_manager = ResourceManager::new();
    let mut input = Input::new(&window);
    let mut character_controller = CharacterState::new();
    let mut world = World::new(&resource_manager.block_models);
    
    let world_mid = ((world::RENDER_DISTANCE / 2) * CHUNK_WIDTH as u32) as f32;
    
    let mut camera = engine::camera::Camera::new(
        Vec3::new(world_mid, 200.0, world_mid),
        90.0,
        0.0,
        window.fb_width as u32,
        window.fb_height as u32,
        75.0
    );

    window.set_cursor_mode(glfw::CursorMode::Disabled);
    
    let model_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_model".as_ptr())};
    let view_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_view".as_ptr())};
    let projection_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_projection".as_ptr())};
    let texture_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"baseTexture".as_ptr())};
    let ui_projection_loc = unsafe {gl::GetUniformLocation(resource_manager.ui_shader, c"u_projection".as_ptr())};

    let (vao_crosshair, vbo_crosshair) = upload_crosshair(resource_manager.tile_atlas.get_tile(String::from("crosshair")), & window);

    // game loop
    while !window.should_close() {
        window.poll_events();
        input.update(&window);

        character_controller.update(&mut camera, &input, &mut world, &mut window);

        world.update_dirty_chunks(&resource_manager.block_models);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(resource_manager.base_shader);
           
            bind(resource_manager.tile_atlas.id, 0);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            gl::Uniform1i(texture_loc, 0);

            chunk_renderer::render_chunks(&mut world, model_loc);
            
            gl::BlendFunc(gl::ONE_MINUS_DST_COLOR, gl::ZERO);
            gl::UseProgram(resource_manager.ui_shader);
            gl::UniformMatrix4fv(ui_projection_loc, 1, gl::FALSE, window.ui_projection.to_cols_array().as_ptr());
            render_crosshair(vao_crosshair);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        
        window.swap_buffers();
    }
}