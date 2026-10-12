#![allow(unused)]

mod engine;
mod game;

use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use glam::{Mat4, Vec2, Vec3, IVec3};
use rand::rand_core::utils::Word;

use crate::engine::entities::character_controller::CharacterState;
use crate::engine::gui::basic_ui::UIAnchor::AnchorBottomLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorCentered;
use crate::engine::gui::basic_ui::UIAnchor::AnchorMidRight;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopLeft;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopMid;
use crate::engine::gui::basic_ui::UIAnchor::AnchorTopRight;
use crate::engine::gui::basic_ui::UIButton;
use crate::engine::gui::basic_ui::UICheckBox;
use crate::engine::gui::basic_ui::UIElement;
use crate::engine::gui::basic_ui::UIFrame;
use crate::engine::gui::basic_ui::UILabel;
use crate::engine::gui::basic_ui::UIContainer;
use crate::engine::gui::menu_manager;
use crate::engine::gui::menu_manager::Menu;
use crate::engine::gui::menu_manager::MenuManager;
use crate::engine::physics::aabb::*;
use crate::engine::core::input::Input;
use crate::engine::rendering::renderering_server::RenderingServer;
use crate::engine::rendering::renderering_server::render_crosshair;
use crate::engine::rendering::renderering_server::upload_crosshair;
use crate::engine::resources::resource_manager::ResourceManager;
use crate::engine::core::window::GameWindow;
use crate::engine::core::worker::Worker;
use crate::engine::rendering::camera::*;
use crate::engine::*;
use crate::engine::world::chunk::*;
use crate::engine::world::world::*;
use crate::engine::resources::texture::*;
use crate::engine::resources::shader::*;
use crate::engine::blocks::block_model::*;
use crate::engine::physics::*;
use crate::engine::physics::aabb::move_and_collide;
use crate::engine::physics::aabb::*;
use crate::engine::physics::aabb::AABB;

fn main() {
    
    let mut window = GameWindow::new("rusted-voxels", 1920, 1080);
    let mut input = Input::new(&window);
    let resource_manager = ResourceManager::new();
    let mut menu_manager = MenuManager::new(&window);
    menu_manager.set_menu(Menu::MenuMain);
    let mut rendering_server = RenderingServer::new();
    rendering_server.update_shader_unif_loc(&resource_manager);
    let (vao_crosshair, vbo_crosshair) = upload_crosshair(resource_manager.tile_atlas.get_tile(String::from("crosshair")), &window);
    let worker = Worker::new(4);
    let mut world: Option<World> = None;
    let mut character_controller = CharacterState::new();
    
    let world_mid = ((world::world::RENDER_DISTANCE / 2) * CHUNK_WIDTH as u32) as f32;
    let cam_spawn_pos = Vec3::new(world_mid, 200.0, world_mid);
    
    let mut camera = Camera::new(
        cam_spawn_pos,
        0.0,
        0.0,
        window.fb_width as u32,
        window.fb_height as u32,
        75.0
    );

    let mut frame = UIFrame::new(window.width, window.height, AnchorCentered);
    let mut label = UILabel::new(15, 15, "", AnchorBottomLeft);
    let mut chat_label = UILabel::new(15, 900, "Rusted-Voxels-v0.01", AnchorMidRight);
    let mut ui_button = UIButton::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2, 42 * 10, 13 * 10, "Return", AnchorCentered);
    let mut ui_toggle = UICheckBox::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2 + 300, 120, false, AnchorCentered);
    // impl get and set functions for some structs

    'game_loop: while !window.should_close() {
        window.poll_events();
        input.update(&window);

        if menu_manager.main_menu.button1.down && world.is_none() {
            world = Some(World::new());
            world.as_mut().unwrap().load(&resource_manager.block_models, &worker);
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        if let Some(world) = &mut world {world.update();}

        
        if ui_button.down {camera.position.y += 0.5;}
        
        if camera.position.y < -10.0 {camera.position.y = 200.0}
        if let Some(world) = &mut world {character_controller.update(&mut camera, &input, world, &mut window);}
        menu_manager.update(&input);
        
        label.build_text_mesh(&format!("x: {:.3}, y: {:.3}, z: {:.3}", camera.position.x, camera.position.y + AABB::HUMANOID.min.y, camera.position.z));
        if let Some(world) = &mut world {world.update_dirty_chunks(&resource_manager.block_models);}
        
        unsafe {gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);}
        resource_manager.base_shader.bind();

        // Yes, this horrible implementation will be changed.
        rendering_server.render(
            &resource_manager,
            &camera,
            &character_controller,
            &mut world,
            &window.ui_projection,
            vao_crosshair,
            &input,
            &mut ui_button,
            &mut ui_toggle,
            &label,
            &chat_label,
            &menu_manager,
        );

        window.swap_buffers();
    }
}