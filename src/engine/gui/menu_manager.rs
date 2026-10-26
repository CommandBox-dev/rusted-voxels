use glam::Mat4;
use glfw::{Key, MouseButton, Window};

use crate::engine::{
    core::{
        input::Input,
        window::{self, GameWindow},
    }, gui::{
        basic_ui::{
            UIAnchor::{AnchorCentered, AnchorMidRight, AnchorTopLeft}, UIButton, UIElement, UILabel, UITexturedRect, UITransformC,
        }, menu_manager::Menu::Closed,
    }, resources::{
        resource_manager::{self, ResourceManager},
        texture,
    }, world,
};

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
}

pub trait UIMenu {}

pub struct MenuManager {
    pub selected_menu: Menu,
    last_menus: Vec<Menu>, // stores what menus where openened in order.

    pub main_menu: MainMenu,
    escape_menu: EscapeMenu,
    world_loading_screen: WorldLoadingScreenMenu,
}

impl MenuManager {
    pub fn new(window: &GameWindow) -> Self {
        let window_transform = UITransformC::new(0, 0, window.width, window.height, AnchorTopLeft);

        Self {
            selected_menu: Menu::Closed,
            last_menus: Vec::new(),
            main_menu: MainMenu::new(&window_transform),
            escape_menu: EscapeMenu::new(&window_transform),
            world_loading_screen: WorldLoadingScreenMenu::new(&window_transform),
        }
    }

    pub fn rebuild_menus() {}

    pub fn update(&mut self, input: &Input, window: &mut GameWindow, chunk_count: u32) {
        match &self.selected_menu {
            Menu::Closed => {}

            Menu::Main => {
                if let Some(new_menu) = self.main_menu.update(input, window) {
                    self.selected_menu = new_menu;
                }
            }

            Menu::WorldLoadingScreen => self.world_loading_screen.update(chunk_count),

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

    pub fn render(&self, resource_manager: &ResourceManager) {
        match &self.selected_menu {
            Menu::Closed => return,

            Menu::Main => self.main_menu.render(resource_manager),

            Menu::Pause => self.escape_menu.render(resource_manager),

            Menu::WorldLoadingScreen => self.world_loading_screen.draw(resource_manager),

            _ => return,
        }
    }

    pub fn set_menu(&mut self, new_menu: Menu) {
        self.selected_menu = new_menu;
    }
}



pub struct MainMenu {
    version_label: UILabel,
    pub button1: UIButton,
    button2: UIButton,
}

impl MainMenu {
    fn new(window_transform: &UITransformC) -> Self {
        Self {
            version_label: UILabel::new(
                15,
                900,
                "Rusted-Voxels-v0.01",
                AnchorMidRight,
                window_transform,
            ),
            button1: UIButton::new(
                0,
                -200,
                42 * 10,
                13 * 10,
                "Play",
                AnchorCentered,
                window_transform,
            ),
            button2: UIButton::new(
                0,
                200,
                42 * 10,
                13 * 10,
                "Settings",
                AnchorCentered,
                window_transform,
            ),
        }
    }

    fn update(&mut self, input: &Input, window: &GameWindow) -> Option<Menu> {
        let mx = (window.fb_width as f32 * (input.mouse_x as f32 / window.width as f32)) as u32;
        let my = (window.fb_height as f32 * (input.mouse_y as f32 / window.height as f32)) as u32;

        self.button1
            .cursor_update(mx, my, input.is_mouse_button_down(MouseButton::Left as i32));
        self.button2.cursor_update(
            mx,
            my,
            input.is_mouse_button_pressed(MouseButton::Left as i32),
        );
        if self.button1.pressed {
            return Some(Menu::Closed); // WorldLoadingScreen
        }
        None
    }

    fn render(&self, resource_manager: &ResourceManager) {
        self.button1.draw();
        self.button2.draw();
        resource_manager.font_tex.bind(0);
        self.button1.draw_text();
        self.button2.draw_text();
    }
}

pub struct EscapeMenu {
    //label_game_state: UILabel,
    button_continue: UIButton,
    button_settings: UIButton,
    button_world_options: UIButton,
    button_leave: UIButton,
}

impl EscapeMenu {
    fn new(window_transform: &UITransformC) -> EscapeMenu {
        Self {
            button_continue: UIButton::new(
                0,
                -300,
                42 * 10,
                13 * 10,
                "Continue",
                AnchorCentered,
                window_transform,
            ),
            button_settings: UIButton::new(
                0,
                -100,
                42 * 10,
                13 * 10,
                "Settings",
                AnchorCentered,
                window_transform,
            ),
            button_world_options: UIButton::new(
                0,
                100,
                42 * 10,
                13 * 10,
                "World Options",
                AnchorCentered,
                window_transform,
            ),
            button_leave: UIButton::new(
                0,
                300,
                42 * 10,
                13 * 10,
                "Leave",
                AnchorCentered,
                window_transform,
            ),
        }
    }

    fn render(&self, resource_manager: &ResourceManager) {
        self.button_continue.draw();
        self.button_settings.draw();
        self.button_world_options.draw();
        self.button_leave.draw();
        resource_manager.font_tex.bind(0);
        self.button_continue.draw_text();
        self.button_settings.draw_text();
        self.button_world_options.draw_text();
        self.button_leave.draw_text();
    }
}

struct WorldLoadingScreenMenu {
    background: UITexturedRect,
    label: UILabel,
}

impl WorldLoadingScreenMenu {
    pub fn new(window_transform: &UITransformC) -> Self {
        let height_aspect = window_transform.w as f32 / window_transform.h as f32;
        let mut s = Self {
            background: UITexturedRect::new(
                0,
                0,
                window_transform.w as u32,
                (window_transform.h as f32 * height_aspect) as u32,
                AnchorTopLeft,
                window_transform,
            ),
            label: UILabel::new(
                0,
                0,
                "label.loading_progress",
                AnchorCentered,
                window_transform,
            ),
        };
        s
    }

    fn draw(&self, resource_manager: &ResourceManager) {
        resource_manager.background_tex.bind(0);
        self.background.draw();
        resource_manager.font_tex.bind(0);
        self.label.draw();
    }

    fn update(&mut self, chunk_count: u32) {
        self.label.build_mesh(
            &format!(
                "Loaded Chunks: {}/{}",
                chunk_count,
                world::world::RENDER_AREA
            ),
        );
    }
}
