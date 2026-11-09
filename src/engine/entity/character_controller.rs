use std::ops::Add;

use glam::{IVec2, IVec3, Vec2, Vec3};
use glfw::*;

use crate::engine::animation::animation_player::PlayingAnimation;
use crate::engine::audio::audio_server::AudioServer;
use crate::engine::core::input::Input;
use crate::engine::core::window::GameWindow;
use crate::engine::gui::menu_manager::{Menu, MenuManager};
use crate::engine::physic::aabb::{AABB, move_and_collide};
use crate::engine::physic::raycast;
use crate::engine::rendering::camera::Camera;
use crate::engine::resources::resource_manager::{self, ResourceManager};
use crate::engine::storage::container::Container;
use crate::engine::world::chunk::CHUNK_WIDTH;
use crate::engine::world::chunk_region::ChunkRegion;
use crate::engine::world::voxel_coordinates::ChunkCoord;
use crate::engine::world::{chunk_region, lighting_system};
use crate::engine::world::world::World;
use crate::game;
use crate::game::blocks::blocks::Blocktype;


pub struct CharacterState {
    pub selected_slot: u32,
    pub inventory: Container,
    pub target_block: IVec3,
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_on_floor: bool,
    pub is_flying: bool,
}

impl CharacterState {
    pub fn new() -> Self {
        Self {
            selected_slot: 2,
            inventory: Container::new(10),
            target_block: IVec3::ZERO,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            is_on_floor: false,
            is_flying: false,
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
        viewmodel_anim: &mut PlayingAnimation,
    ) {
        let ray_result = raycast::cast_ray(camera.position, camera.front, 10.0, &mut world);
        self.target_block = ray_result.hit_position;

        let in_focus = menu_manager.selected_menu == Menu::Closed;

        // block breaking
        if in_focus && input.is_mouse_button_pressed(MouseButton::Left as i32) {
            viewmodel_anim.time = 0.0;
            if ray_result.hit {
                world.set_block(
                    ray_result.hit_position.x,
                    ray_result.hit_position.y,
                    ray_result.hit_position.z,
                    Blocktype::Air,
                );
                audio_manager.play_sound_flat(resource_manager.break_sound.clone());

                let pos_above = IVec3::new(
                    ray_result.hit_position.x,
                    ray_result.hit_position.y + 1,
                    ray_result.hit_position.z,
                );
                let block_above = world.get_block(pos_above.x, pos_above.y, pos_above.z);
                game::blocks::blocks::on_break(block_above, self.target_block, world);

                let cx = ray_result.hit_position.x.div_euclid(CHUNK_WIDTH);
                let cz = ray_result.hit_position.z.div_euclid(CHUNK_WIDTH);

                let lx = ray_result.hit_position.x.rem_euclid(CHUNK_WIDTH);
                let lz = ray_result.hit_position.z.rem_euclid(CHUNK_WIDTH);

                let coord = ChunkCoord::new(cx, cz);
                if let Some(mut region) = chunk_region::ChunkRegion::take_chunk_region(&mut world.chunk_manager, &coord) {
                    if let Some(mut chunk) = world.chunk_manager.get_avl_chunk_mut(cx, cz) {
                        //lighting_system::update_light(chunk, &mut region, &IVec3::new(lx, ray_result.hit_position.y, lz));
                        chunk.clear_light();
                        lighting_system::bake_indirect_light(chunk, &mut region);
                        ChunkRegion::insert_chunk_region(&mut world.chunk_manager, &coord, region);
                    }
                }
            }
        }

        if input.is_pressed(Key::F as i32) {
            self.is_flying = !self.is_flying;
        }

        // block placing
        if in_focus && input.is_mouse_button_pressed(MouseButton::Right as i32) {
            if ray_result.hit {
                let place_pos = ray_result.hit_position + ray_result.hit_normal;

                let mut place_aabb = AABB::CUBE;
                let mut character_aabb = AABB::HUMANOID;

                place_aabb.add_offset(place_pos.as_vec3());
                character_aabb.add_offset(self.position);

                if true { // !place_aabb.intersects(&character_aabb) {
                    viewmodel_anim.time = 0.0;
                    world.set_block(
                        place_pos.x,
                        place_pos.y,
                        place_pos.z,
                        Blocktype::Table, //self.selected_slot + 1,
                    );
                    audio_manager.play_sound_flat(resource_manager.place_sound.clone());
                    game::blocks::blocks::on_place(Blocktype::Stone, place_pos, world);

                    let cx = ray_result.hit_position.x.div_euclid(CHUNK_WIDTH);
                    let cz = ray_result.hit_position.z.div_euclid(CHUNK_WIDTH);

                    let lx = ray_result.hit_position.x.rem_euclid(CHUNK_WIDTH);
                    let lz = ray_result.hit_position.z.rem_euclid(CHUNK_WIDTH);

                    let coord = ChunkCoord::new(cx, cz);
                    if let Some(mut region) = chunk_region::ChunkRegion::take_chunk_region(&mut world.chunk_manager, &coord) {
                        if let Some(mut chunk) = world.chunk_manager.get_avl_chunk_mut(cx, cz) {
                            //lighting_system::update_light(chunk, &mut region, &IVec3::new(lx, ray_result.hit_position.y, lz));
                            chunk.clear_light();
                            lighting_system::bake_indirect_light(chunk, &mut region);
                            ChunkRegion::insert_chunk_region(&mut world.chunk_manager, &coord, region);
                        }
                    }
                }
            }
        }

        // block selecting
        if in_focus && input.is_mouse_button_pressed(MouseButton::Middle as i32) {
            if ray_result.hit {
                //self.selected_slot = Blocktype::Stone;
            }
        }

        // hotbar
        if in_focus {
            if input.is_pressed(Key::Right as i32) {
                self.selected_slot += 1;
                if self.selected_slot >= 10 {
                    self.selected_slot = 0;
                }
            }
            if input.is_pressed(Key::Left as i32) {
                if self.selected_slot == 0 {
                    self.selected_slot = 9;
                } else {
                    self.selected_slot -= 1;
                }
            }

            if input.is_pressed(Key::Num1 as i32) {
                self.selected_slot = 0;
            }
            if input.is_pressed(Key::Num2 as i32) {
                self.selected_slot = 1;
            }
            if input.is_pressed(Key::Num3 as i32) {
                self.selected_slot = 2;
            }
            if input.is_pressed(Key::Num4 as i32) {
                self.selected_slot = 3;
            }
            if input.is_pressed(Key::Num5 as i32) {
                self.selected_slot = 4;
            }
            if input.is_pressed(Key::Num6 as i32) {
                self.selected_slot = 5;
            }
            if input.is_pressed(Key::Num7 as i32) {
                self.selected_slot = 6;
            }
            if input.is_pressed(Key::Num8 as i32) {
                self.selected_slot = 7;
            }
            if input.is_pressed(Key::Num9 as i32) {
                self.selected_slot = 8;
            }
            if input.is_pressed(Key::Num0 as i32) {
                self.selected_slot = 9;
            }
        }

        // structure placement
        if in_focus && input.is_pressed(Key::P as i32) {
            if ray_result.hit {
                let hit = ray_result.hit_position.clone().add(ray_result.hit_normal);
                world.generate_maze(hit.x, hit.y, hit.z);
            }
        }

        // head rotation
        let (move_x, move_y) = input.get_last_mouse_movement();
        if in_focus {
            camera.yaw += -move_x;
            camera.pitch += -move_y;

            camera.pitch = camera.pitch.clamp(-89.0, 89.0);
            camera.recalculate_vectors();
        }

        // movement
        let mut speed = 850.0;

        let movement_input = IVec2::new(
            input.is_down(Key::W as i32) as i32 - input.is_down(Key::S as i32) as i32,
            input.is_down(Key::A as i32) as i32 - input.is_down(Key::D as i32) as i32,
        ).as_vec2();

        let vertical_movement_input = (
            input.is_down(Key::Space as i32) as i32
            - input.is_down(Key::LeftShift as i32) as i32
        ) as f32;



        if input.is_down(Key::LeftControl as i32) {
            speed *= 4.0;
        }

        let vel_y = self.velocity.y;
        self.velocity = camera.forward * movement_input.x * speed * window.delta;
        self.velocity += camera.right * movement_input.y * speed * window.delta;
        self.velocity.y += vel_y;

        if self.is_flying {
            self.velocity.y = vertical_movement_input * 1200.0 * window.delta;
        } else {
            self.velocity.y += -100.0 * window.delta;
            if input.is_pressed(Key::Space as i32) {
                self.velocity.y = 20.0;
            }
        }

        let mut final_velocity = (self.velocity.clone() * window.delta);
        
        let collision_result = move_and_collide(world, &self.position, &mut final_velocity, &AABB::HUMANOID);
        self.is_on_floor = collision_result.is_on_floor;
        self.position += final_velocity;
        
        camera.position = self.position;
        camera.recalculate_view();

        // self.velocity.x *= 0.90;
        // self.velocity.z *= 0.90;
        // self.velocity.y *= 0.90;

        // void
        if self.position.y < -10.0 {
            self.position.y = 200.0;
            self.velocity = Vec3::ZERO;
        }
    }
}
