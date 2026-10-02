#![allow(unused)]

mod engine;
use crate::engine::physics::aabb::*;

use crate::engine::input::Input;
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

use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use glam::{Mat4, Vec2, Vec3, IVec3};
use rand::rand_core::utils::Word;

fn main() {

    let mut is_on_floor = false;
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);
    let mut escape = false;
    let mut selected_block: u32 = 3;
    
    let mut window = GameWindow::new("rusted-voxels", 1920, 1080);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    let mut input = Input::new(&window);

    let resource_manager = ResourceManager::new();    
    
    let model_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_model".as_ptr())};
    let view_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_view".as_ptr())};
    let projection_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"u_projection".as_ptr())};
    let texture_loc = unsafe {gl::GetUniformLocation(resource_manager.base_shader, c"baseTexture".as_ptr())};
    
    let mut world = World::new(&resource_manager.block_models);
    
    let world_mid = ((world::RENDER_DISTANCE / 2) * CHUNK_WIDTH as u32) as f32;
    let mut camera = engine::camera::Camera::new(
        Vec3::new(world_mid, 105.0, world_mid),
        180.0,
        0.0,
        window.fb_width as u32,
        window.fb_height as u32,
        75.0
    );

    // game loop
    while !window.should_close() {
        window.poll_events();
        input.update(&window);

        if input.is_mouse_button_pressed(MouseButton::Left as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                &world.set_block(
                    result.hit_position.x,
                    result.hit_position.y,
                    result.hit_position.z,
                    0,
                );
            }
        }

        if input.is_mouse_button_pressed(MouseButton::Right as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                &world.set_block(
                    result.hit_position.x + result.hit_normal.x,
                    result.hit_position.y + result.hit_normal.y,
                    result.hit_position.z + result.hit_normal.z,
                    selected_block,
                );
            }
        }

        if input.is_mouse_button_pressed(MouseButton::Middle as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                selected_block = result.hit_block;
            }
        }

        if input.is_pressed(Key::Escape as i32) {
            escape = !escape;
            if escape {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            } else {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            }
        }

        let (move_x, move_y) = input.get_last_mouse_movement();
        if !escape {
            camera.yaw += -move_x;
            camera.pitch += -move_y;

            camera.pitch = camera.pitch.clamp(-89.0, 89.0);
            camera.recalculate_vectors();
        }

        let mut front: i32 = 0;
        let mut back: i32 = 0;
        let mut right: i32 = 0;
        let mut left: i32 = 0;

        if input.is_down(Key::W as i32) {
            front = 1;
        }
        if input.is_down(Key::S as i32) {
            back = 1;
        }
        if input.is_down(Key::D as i32) {
            right = 1;
        }
        if input.is_down(Key::A as i32) {
            left = 1;
        }
        if input.is_down(Key::Space as i32) {
            velocity.y = 0.1;
        } else {
            velocity.y += (-9.8 * 0.045 * window.delta);
        }
        
        let movement_input = Vec2::new((front - back) as f32, -(right - left) as f32);
        let mut speed = 5.0;
        
        if input.is_down(Key::LeftShift as i32) {
            speed *= 3.0;
        }
        
        let vel_y = velocity.y;
        velocity.y *= 0.90;

        velocity = camera.forward * movement_input.x * window.delta * speed;
        velocity += camera.right * movement_input.y * window.delta * speed;
        velocity.y += vel_y;

        let result = move_and_collide(&world, &camera.position, &mut velocity, &AABB::HUMANOID);
        is_on_floor = result.is_on_floor;

        camera.position += velocity;

        camera.recalculate_view();






        world.update_dirty_chunks(&resource_manager.block_models);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(resource_manager.base_shader);
            bind(resource_manager.tile_atlas.id, 0);
            
            // bind camera matrices
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            gl::Uniform1i(texture_loc, 0);

            chunk_renderer::render_chunks(&mut world, model_loc);
            
            gl::BindVertexArray(0);
        }
        
        window.swap_buffers();
    }
}