#![allow(unused)]

mod engine;
mod game;

use glam::Quat;
use glam::{IVec3, Mat4, Vec2, Vec3};
use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use rand::rand_core::utils::Word;

use crate::engine::animation::animation::PlaybackMode::{self, Once, PingPong};
use crate::engine::animation::animation::{Animation, AnimationTrack, Bone, QuatKeyframe, VecKeyframe};
use crate::engine::animation::animation_player::{self, AnimationPlayer, PlayingAnimation};
use crate::engine::animation::skeleton::{self, Skeleton};
use crate::engine::audio::audio_server::AudioServer;
use crate::engine::block::block_model::*;
use crate::engine::core::input::Input;
use crate::engine::core::window::GameWindow;
use crate::engine::core::worker::Worker;
use crate::engine::entity::character_controller::CharacterState;
use crate::engine::gui::basic_ui::UIAnchor::AnchorBottomLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorCentered;
use crate::engine::gui::basic_ui::UIAnchor::AnchorLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorRight;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTop;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopRight;
use crate::engine::gui::basic_ui::UITransformC;
use crate::engine::gui::menu_manager;
use crate::engine::gui::menu_manager::Menu;
use crate::engine::gui::menu_manager::MenuManager;
use crate::engine::physics::aabb::AABB;
use crate::engine::physics::aabb::move_and_collide;
use crate::engine::physics::aabb::*;
use crate::engine::physics::aabb::*;
use crate::engine::physics::*;
use crate::engine::rendering::mesh::BasicMesh;
use crate::engine::rendering::{camera::*, mesh};
use crate::engine::rendering::chunk_mesher;
use crate::engine::rendering::renderering_server::RenderingServer;
use crate::engine::resources::resource_manager::ResourceManager;
use crate::engine::resources::shader::*;
use crate::engine::resources::texture::*;
use crate::engine::world::chunk::*;
use crate::engine::world::chunk_manager::ChunkLoader;
use crate::engine::world::generation::world_generator::TerrainGenerator;
use crate::engine::world::voxel_coordinates::BlockCoord;
use crate::engine::world::world::*;
use crate::engine::*;
use crate::game::blocks::blocks::Blocktype;
use crate::game::world::generation::Dimension::Flat;
use crate::game::world::terrain::flat::{FlatGen, TerrainLayer};

fn main() {
    let mut window = GameWindow::new("rusted-voxels", 1920, 1080);
    let mut input = Input::new(&window);

    let mut resource_manager = ResourceManager::new();
    let mut menu_manager = MenuManager::new(&window);
    let mut audio_manager = AudioServer::new();
    let mut rendering_server = RenderingServer::new();
    let mut animation_player = AnimationPlayer::new();
    
    let worker = Worker::new(8);
    let meshing_worker = Worker::new(5);

    let mut world: Option<World> = None;
    let mut character_controller = CharacterState::new();
    let mut camera = Camera::new(
        Vec3::ZERO,
        0.0,
        0.0,
        window.fb_width as u32,
        window.fb_height as u32,
        75.0,
    );
    
    menu_manager.set_menu(Menu::Main);
    rendering_server.update_shader_unif_loc(&resource_manager);

    // block animation
    let position_timeline: Vec<VecKeyframe> = vec![];
    let rotation_timeline: Vec<QuatKeyframe> = vec![
        QuatKeyframe::new(Vec3::new(0.0, 0.0, 0.0), 0.0),
        QuatKeyframe::new(Vec3::new(-45.0, 0.0, -45.0), 0.3),
        QuatKeyframe::new(Vec3::new(-45.0, 0.0, -45.0), 0.5),
        QuatKeyframe::new(Vec3::new(0.0, 0.0, 0.0), 1.0),
    ];
    let scale_timeline: Vec<VecKeyframe> = vec![];
    // sword animation
    let position_timeline: Vec<VecKeyframe> = vec![
        VecKeyframe::new(Vec3::new(0.0, 0.0, 0.0), 0.0)
    ];
    let rot = 0.0_f32.to_radians();
    let rotation_timeline: Vec<QuatKeyframe> = vec![
        QuatKeyframe::new(Vec3::new(rot, 0.0_f32.to_radians(), rot), 0.0),
        QuatKeyframe::new(Vec3::new(rot, -45.0_f32.to_radians(), rot), 0.25),
        QuatKeyframe::new(Vec3::new(rot, 45.0_f32.to_radians(), rot), 0.5),
        QuatKeyframe::new(Vec3::new(rot, 45.0_f32.to_radians(), rot), 0.75),
        QuatKeyframe::new(Vec3::new(rot, 0.0_f32.to_radians(), rot), 1.0),
    ];

    animation_player.add_animation(Animation {
        fps: 60,
        duration: 30.0,
        playback_mode: PlaybackMode::Once,
        tracks: vec![AnimationTrack {
            bone_id: 0,
            position: position_timeline,
            rotation: rotation_timeline,
            scale: scale_timeline,
        }]
    });

    let mut one_bone_skeleton = Skeleton {
        bones: vec![Bone::new(
            Vec3::ONE,
            Vec3::ZERO,
            Vec3::ONE,
            Some(Vec3::new(0.5, 0.5, 0.5)),
        )]
    };


    let mut playing_anim = PlayingAnimation {
        animation_id: 0,
        reversed: false,
        speed: 1.0,
        time: 0.0,
        playing: true,
    };


    let mut flat_gen = FlatGen::new(vec![
        TerrainLayer::new(1, Blocktype::CompressedStone),
        TerrainLayer::new(5, Blocktype::Stone),
        TerrainLayer::new(10, Blocktype::Grass),
    ]);

    //audio_manager.play_sound_flat(resource_manager.white_grass_music.clone());

    let debug_mesh = BasicMesh::new_cube_pos_uv();
    let entity_mesh = BasicMesh::new_cube_pos_uv();

    'game_loop: while !window.should_close() {
        window.poll_events(&mut input, &mut camera, &mut menu_manager);

        if menu_manager.main_menu.button1.pressed && world.is_none() {
            world = Some(World::new());
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        if let Some(world) = &mut world {
            animation_player.update(0.01666666667);
            animation_player.update_animation(&mut playing_anim, &mut one_bone_skeleton);

            world.update(&resource_manager.block_models, &worker);
            world.chunk_manager.load_chunks_in_radius(
                character_controller.position.x as i32,
                character_controller.position.z as i32,
                RENDER_DISTANCE,
                &worker,
                &resource_manager.block_models,
                &flat_gen,
            );
            world.update_dirty_chunks(&resource_manager.block_models, &meshing_worker);
            character_controller.update(
                &mut camera,
                &input,
                world,
                &mut window,
                &menu_manager,
                &mut audio_manager,
                &resource_manager,
                &mut playing_anim,
            );
        }
        
        if let Some(world) = &world {
            if world.chunk_manager.chunks_loaded >= world::world::RENDER_AREA - 100
            && menu_manager.selected_menu == Menu::WorldLoadingScreen
            {
                menu_manager.selected_menu = Menu::Closed;
                character_controller.position.y = 200.0;
            }
        }

        menu_manager.update(&input, &mut window, &character_controller, world.as_ref());

        rendering_server.render(
            &resource_manager,
            &camera,
            &character_controller,
            &mut world,
            &window.ui_projection,
            &input,
            &menu_manager,
            &debug_mesh,
            &window,
            &one_bone_skeleton.bones[0],
            &entity_mesh,
        );

        window.swap_buffers();
    }
}
