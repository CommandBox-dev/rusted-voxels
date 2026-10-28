#![allow(unused)]

mod engine;
mod game;

use glam::{IVec3, Mat4, Vec2, Vec3};
use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use rand::rand_core::utils::Word;

use crate::engine::audio::audio_server::AudioServer;
use crate::engine::blocks::block_model::*;
use crate::engine::core::input::Input;
use crate::engine::core::window::GameWindow;
use crate::engine::core::worker::Worker;
use crate::engine::entities::character_controller::CharacterState;
use crate::engine::gui::basic_ui::UIAnchor::AnchorBottomLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorCentered;
use crate::engine::gui::basic_ui::UIAnchor::AnchorMidLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorMidRight;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopMid;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopRight;
use crate::engine::gui::basic_ui::UIButton;
use crate::engine::gui::basic_ui::UICheckBox;
use crate::engine::gui::basic_ui::UIContainer;
use crate::engine::gui::basic_ui::UIElement;
use crate::engine::gui::basic_ui::UIFrame;
use crate::engine::gui::basic_ui::UILabel;
use crate::engine::gui::basic_ui::UITransformC;
use crate::engine::gui::menu_manager;
use crate::engine::gui::menu_manager::Menu;
use crate::engine::gui::menu_manager::MenuManager;
use crate::engine::physics::aabb::AABB;
use crate::engine::physics::aabb::move_and_collide;
use crate::engine::physics::aabb::*;
use crate::engine::physics::aabb::*;
use crate::engine::physics::*;
use crate::engine::rendering::camera::*;
use crate::engine::rendering::chunk_mesher;
use crate::engine::rendering::renderering_server::RenderingServer;
use crate::engine::resources::resource_manager::ResourceManager;
use crate::engine::resources::shader::*;
use crate::engine::resources::texture::*;
use crate::engine::world::chunk::*;
use crate::engine::world::chunk_manager::ChunkLoader;
use crate::engine::world::voxel_coordinates::BlockCoord;
use crate::engine::world::world::*;
use crate::engine::*;

pub const VERTICES: [f32; 108 + 72] = [
    // right
    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 0.0, 1.0,

    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,

    // left
    0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 0.0, 1.0,

    // top
    0.0, 1.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,

    0.0, 1.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 0.0, 1.0,

    // bottom
    0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 0.0, 0.0, 1.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 1.0,
    0.0, 0.0, 1.0, 0.0, 1.0,

    // front
    0.0, 0.0, 1.0, 0.0, 0.0,
    1.0, 0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0,

    0.0, 0.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 1.0, 0.0, 1.0,

    // back
    1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 1.0, 1.0,

    1.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 0.0, 1.0,
];

fn main() {
    let mut window = GameWindow::new("rusted-voxels", 1920, 1080);
    let mut input = Input::new(&window);

    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioServer::new();
    let mut menu_manager = MenuManager::new(&window);
    let mut rendering_server = RenderingServer::new();
    
    menu_manager.set_menu(Menu::Main);
    rendering_server.update_shader_unif_loc(&resource_manager);

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

    let window_transform = UITransformC::new(0, 0, window.width as u32, window.height as u32, AnchorTopLeft);
    //audio_manager.play_sound_flat(resource_manager.white_grass_music.clone());

    let mut vao_wireframe = 0;
    let mut vbo_wireframe = 0;
    let float_size = std::mem::size_of::<f32>();
    let mesh_byte_size = (crate::VERTICES.len() as usize * float_size) as isize;

    let stride = 5 * float_size as i32;

    unsafe {

        gl::GenVertexArrays(1, &mut vao_wireframe);
        gl::GenBuffers(1, &mut vbo_wireframe);

        gl::BindVertexArray(vao_wireframe);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_wireframe);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            mesh_byte_size,
            VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // uv
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * float_size) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
    }



    'game_loop: while !window.should_close() {
        window.poll_events(&mut input, &mut camera);

        if menu_manager.main_menu.button1.pressed && world.is_none() {
            world = Some(World::new());
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        if let Some(world) = &mut world {
            world.update(&resource_manager.block_models, &worker);
            world.chunk_manager.load_chunks_in_radius(
                character_controller.position.x as i32,
                character_controller.position.z as i32,
                RENDER_DISTANCE,
                &worker,
                &resource_manager.block_models
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
            );
        }
        
        if let Some(world) = &world {
            if world.chunk_manager.chunks_loaded >= world::world::RENDER_AREA - 100
            && menu_manager.selected_menu == Menu::WorldLoadingScreen
            {
                menu_manager.selected_menu = Menu::Closed;
                character_controller.position.y = 200.0;
            }
            menu_manager.update(&input, &mut window, world.chunk_manager.chunks_loaded);
        } else {
            menu_manager.update(&input, &mut window, 0);
        }

        rendering_server.render(
            &resource_manager,
            &camera,
            &character_controller,
            &mut world,
            &window.ui_projection,
            &input,
            &menu_manager,
            vao_wireframe,
        );

        window.swap_buffers();
    }
}
