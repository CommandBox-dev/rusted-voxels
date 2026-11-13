use glam::Mat4;
use glfw::{Key, MouseButton, Window};

use crate::{engine::{
    core::{
        input::Input,
        window::{self, GameWindow},
    }, entity::character_controller::{self, CharacterState}, gui::{
        basic_ui::{
            UIAnchor::{AnchorBottom, AnchorBottomLeft, AnchorCentered, AnchorRight, AnchorTop, AnchorTopLeft, AnchorTopRight}, UIElement, UITransformC,
        }, menu_manager::Menu::Closed,
    }, physic::aabb::AABB, rendering::renderering_server::{self, RenderingServer}, resources::{
        resource_manager::{self, ResourceManager}, texture::{self, AtlasTile, UV},
    }, world::{self, world::World},
}, game::{data::game_data::GameData, menus::{debug_screen::DebugScreen, game_hud::GameHUD, main_menu::MainMenu, menu_inventory::PlayerInventory, pause_menu::EscapeMenu, world_loading_screen::WorldLoadingScreenMenu}}};
use crate::engine::gui::elements::{label::UILabel, tex_rect::UITexturedRect, button::UIButton};

#[derive(PartialEq, Eq, Debug)]
pub enum Menu {
    Closed,
    Main,
    Pause,
    Settings,
    WorldCreations,
    WorldGenOptions,
    WorldExtraOptions,
    WorldLoadingScreen,
    Inventory,
}

pub struct MenuManager {
    pub selected_menu: Menu,
    last_menus: Vec<Menu>,

    pub main_menu: MainMenu,
    escape_menu: EscapeMenu,
    world_loading_screen: WorldLoadingScreenMenu,
    game_hud: GameHUD,
    debug_screen: DebugScreen,
    inventory: PlayerInventory,
}

impl MenuManager {
    pub fn new(window: &GameWindow, character_controller: &CharacterState) -> Self {
        let window_transform = UITransformC::new(0, 0, window.width, window.height, AnchorTopLeft);

        Self {
            selected_menu: Menu::Closed,
            last_menus: Vec::new(),
            main_menu: MainMenu::new(&window_transform),
            escape_menu: EscapeMenu::new(&window_transform),
            world_loading_screen: WorldLoadingScreenMenu::new(&window_transform),
            game_hud: GameHUD::new(&window_transform),
            debug_screen: DebugScreen::new(&window_transform),
            inventory: PlayerInventory::new(&window_transform, &character_controller.inventory.slots),
        }
    }

    pub fn set_menu(&mut self, new_menu: Menu) {
        self.selected_menu = new_menu;
    }

    pub fn rebuild_menus(&mut self, window: &GameWindow) {
        let window_transform = UITransformC::new(0, 0, window.width as u32, window.height as u32, AnchorTopLeft);

        self.main_menu.realign(&window_transform);
        self.escape_menu.realign(&window_transform);
        self.world_loading_screen.realign(&window_transform);
        self.game_hud.realign(&window_transform);
        self.debug_screen.realign(&window_transform);
    }

    pub fn update(
        &mut self,
        input: &Input,
        window: &mut GameWindow,
        character_controller: &mut CharacterState,
        world: Option<&World>
    ) {

        let window_transform = UITransformC::new(0, 0, window.width as u32, window.height as u32, AnchorTopLeft);

        self.game_hud.update(character_controller, &window_transform);

        self.debug_screen.update(world, Some(character_controller));

        match &self.selected_menu {
            Menu::Closed => {}

            Menu::Main => {
                if let Some(new_menu) = self.main_menu.update(input, window) {
                    self.selected_menu = new_menu;
                }
            }
            // Possible panic!!!
            Menu::WorldLoadingScreen => self.world_loading_screen.update(if world.is_some() {world.unwrap().chunk_manager.chunks_loaded} else {0}, &world.unwrap().chunk_manager),

            Menu::Inventory => self.inventory.update(&mut character_controller.inventory, input),

            _ => {}
        }
        if input.is_pressed(Key::Escape as i32) {
            match self.selected_menu {
                Menu::Closed => {
                    self.selected_menu = Menu::Pause;
                    window.set_cursor_mode(glfw::CursorMode::Normal);
                }
                Menu::Pause => {
                    self.selected_menu = Menu::Closed;
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                }
                _ => {}
            }
        }
    }

    pub fn render(
        &self,
        resource_manager: &ResourceManager,
        in_world: bool,
        character_controller: &CharacterState,
        rendering_server: &RenderingServer,
        game_data: &GameData,
        window: &GameWindow,
    ) {

        if self.selected_menu == Menu::Closed || self.selected_menu == Menu::Pause {
            self.game_hud.draw(resource_manager);
        }
        
        match &self.selected_menu {
            Menu::Closed => {},
            
            Menu::Main => self.main_menu.render(resource_manager),
            
            Menu::Pause => self.escape_menu.render(resource_manager),
            
            Menu::WorldLoadingScreen => self.world_loading_screen.draw(resource_manager),

            Menu::Inventory => self.inventory.draw(
                character_controller,
                resource_manager,
                rendering_server,
                game_data,
                &window.ui_projection,
                &window,
            ),
            
            _ => {},
        }

        self.debug_screen.render(resource_manager, self.selected_menu == Menu::Pause);
    }

}