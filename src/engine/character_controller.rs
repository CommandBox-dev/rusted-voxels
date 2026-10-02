use glam::{Vec2, Vec3};
use glfw::*;

use crate::engine::camera::Camera;
use crate::engine::input::Input;
use crate::engine::physics::aabb::{AABB, move_and_collide};
use crate::engine::window::GameWindow;
use crate::engine::world::World;
use crate::engine::{raycast, window};

    
// this is just temporarily, i will fix it later

pub struct CharacterState {
    pub escape: bool,

    pub selected_block: u32,
    pub is_on_floor: bool,
    pub velocity: Vec3,
}

impl CharacterState {

    pub fn new() -> Self {
        Self {
            escape: false,
            selected_block: 3,
            is_on_floor: false,
            velocity: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, input: &Input, mut world: &mut World, window: &mut GameWindow) {

        // breaking
        if input.is_mouse_button_pressed(MouseButton::Left as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                world.set_block(
                    result.hit_position.x,
                    result.hit_position.y,
                    result.hit_position.z,
                    0,
                );
            }
        }

        // placing
        if input.is_mouse_button_pressed(MouseButton::Right as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                world.set_block(
                    result.hit_position.x + result.hit_normal.x,
                    result.hit_position.y + result.hit_normal.y,
                    result.hit_position.z + result.hit_normal.z,
                    self.selected_block,
                );
            }
        }

        // block selecting
        if input.is_mouse_button_pressed(MouseButton::Middle as i32) {
            let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
            if result.hit {
                self.selected_block = result.hit_block;
            }
        }

        // escape
        if input.is_pressed(Key::Escape as i32) {
            self.escape = !self.escape;
            if self.escape {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            } else {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            }
        }

        // head rotation
        let (move_x, move_y) = input.get_last_mouse_movement();
        if !self.escape {
            camera.yaw += -move_x;
            camera.pitch += -move_y;

            camera.pitch = camera.pitch.clamp(-89.0, 89.0);
            camera.recalculate_vectors();
        }

        // movement
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
            self.velocity.y = 0.1;
        } else {
            self.velocity.y += (-9.8 * 0.045 * window.delta);
        }
        
        let movement_input = Vec2::new((front - back) as f32, -(right - left) as f32);
        let mut speed = 5.0;
        
        if input.is_down(Key::LeftShift as i32) {
            speed *= 3.0;
        }
        
        let vel_y = self.velocity.y;
        self.velocity.y *= 0.90;

        self.velocity = camera.forward * movement_input.x * window.delta * speed;
        self.velocity += camera.right * movement_input.y * window.delta * speed;
        self.velocity.y += vel_y;

        let result = move_and_collide(world, &camera.position, &mut self.velocity, &AABB::HUMANOID);
        self.is_on_floor = result.is_on_floor;

        camera.position += self.velocity;

        camera.recalculate_view();
    }
}