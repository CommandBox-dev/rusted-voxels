use std::ops::Add;

use glam::{IVec3, Vec2, Vec3};
use glfw::*;

use crate::engine::audio::audio_server::AudioServer;
use crate::engine::core::input::Input;
use crate::engine::core::window::GameWindow;
use crate::engine::gui::menu_manager::{Menu, MenuManager};
use crate::engine::physics::aabb::{AABB, move_and_collide};
use crate::engine::physics::raycast;
use crate::engine::rendering::camera::Camera;
use crate::engine::resources::resource_manager::{self, ResourceManager};
use crate::engine::world::world::World;


pub struct CharacterState {
    pub selected_block: u32,
    pub target_block: IVec3,
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_on_floor: bool,
}

impl CharacterState {
    pub fn new() -> Self {
        Self {
            selected_block: 16, // 5
            target_block: IVec3::ZERO,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            is_on_floor: false,
        }
    }

    pub fn update(
        &mut self,
        camera: &mut Camera,
        input: &Input,
        mut world: &mut World,
        window: &mut GameWindow,
        menu_manager: &MenuManager,
        audio_manager: &mut AudioServer,
        resource_manager: &ResourceManager,
    ) {
        let result = raycast::cast_ray(camera.position, camera.front, 6.0, &mut world);
        self.target_block = result.hit_position;

        // breaking
        if menu_manager.selected_menu == Menu::Closed
            && input.is_mouse_button_pressed(MouseButton::Left as i32)
        {
            if result.hit {
                world.set_block(
                    result.hit_position.x,
                    result.hit_position.y,
                    result.hit_position.z,
                    0,
                );
                audio_manager.play_sound_flat(resource_manager.break_sound.clone());
            }
        }

        // placing
        if menu_manager.selected_menu == Menu::Closed
            && input.is_mouse_button_pressed(MouseButton::Right as i32)
        {
            if result.hit {
                world.set_block(
                    result.hit_position.x + result.hit_normal.x,
                    result.hit_position.y + result.hit_normal.y,
                    result.hit_position.z + result.hit_normal.z,
                    self.selected_block,
                );
                audio_manager.play_sound_flat(resource_manager.place_sound.clone());
            }
        }

        // block selecting
        if menu_manager.selected_menu == Menu::Closed
            && input.is_mouse_button_pressed(MouseButton::Middle as i32)
        {
            if result.hit {
                self.selected_block = result.hit_block;
            }
        }

        // structure placement
        if menu_manager.selected_menu == Menu::Closed && input.is_pressed(Key::P as i32) {
            if result.hit {
                let hit = result.hit_position.clone().add(result.hit_normal);
                world.generate_maze(hit.x, hit.y, hit.z);
            }
        }

        // head rotation
        let (move_x, move_y) = input.get_last_mouse_movement();
        if menu_manager.selected_menu == Menu::Closed {
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

        if menu_manager.selected_menu == Menu::Closed && input.is_down(Key::Space as i32) {
            self.velocity.y = 0.1; // 0.1
        } else {
            self.velocity.y += (-9.8 * 0.045 * window.delta);
        }

        let movement_input = if menu_manager.selected_menu == Menu::Closed {
            Vec2::new((front - back) as f32, -(right - left) as f32)
        } else {
            Vec2::ZERO
        };
        let mut speed = 5.0;

        if input.is_down(Key::LeftShift as i32) {
            speed *= 15.0; // 3.0
        }

        let vel_y = self.velocity.y;
        self.velocity.y *= 0.90;
        self.velocity = camera.forward * movement_input.x * window.delta * speed;
        self.velocity += camera.right * movement_input.y * window.delta * speed;
        self.velocity.y += vel_y;

        // void
        if self.position.y < -10.0 {
            self.position.y = 200.0;
            self.velocity = Vec3::ZERO;
        }

        let result = move_and_collide(world, &self.position, &mut self.velocity, &AABB::HUMANOID);
        self.is_on_floor = result.is_on_floor;

        self.position += self.velocity;
        
        camera.position = self.position;
        camera.recalculate_view();
    }
}
