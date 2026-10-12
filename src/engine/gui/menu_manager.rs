use glam::Mat4;
use glfw::MouseButton;

use crate::engine::{core::{input::Input, window::GameWindow}, gui::{basic_ui::{UIAnchor::{AnchorCentered, AnchorMidRight}, UIButton, UIElement, UILabel}, menu_manager::Menu::NoMenu}, resources::{resource_manager::{self, ResourceManager}, texture}};


pub enum Menu {
    NoMenu,
    MenuMain,
    MenuSettings,
    MenuWorldCreation,
    MenuWorldGenOption,
    MenuWorldExtraOption,
    MenuEscape,
}

pub struct MenuManager {
    selected_menu: Menu,
    last_menu_tree: Vec<Menu>, // stores what menus you openened, to open the last one if you press "back".
    pub main_menu: MainMenu,
}

impl MenuManager {
    pub fn new(window: &GameWindow) -> Self {
        //let mut label = UILabel::new(15, 15, "", AnchorBottomLeft);
        //let mut ui_button = UIButton::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2, 42 * 10, 13 * 10, "Return", AnchorCentered);
        //let mut ui_toggle = UICheckBox::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2 + 300, 120, false, AnchorCentered);
        Self {
            selected_menu: Menu::NoMenu,
            last_menu_tree: Vec::new(),
            main_menu: MainMenu::new(window),
        }
    }
    
    pub fn update(&mut self, input: &Input) {
        match &self.selected_menu {
            Menu::NoMenu => return,

            Menu::MenuMain => if let Some(new_menu) = self.main_menu.update(input) {self.selected_menu = new_menu;},

            _ => return,
        }
    }

    pub fn render(&self, resource_manager: &ResourceManager) {
        match &self.selected_menu {
            Menu::NoMenu => return,

            Menu::MenuMain => self.main_menu.render(resource_manager),

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
    fn new(window: &GameWindow) -> Self {
        Self {
            version_label: UILabel::new(15, 900, "Rusted-Voxels-v0.01", AnchorMidRight),
            button1: UIButton::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2 - 200, 42 * 10, 13 * 10, "Play", AnchorCentered),
            button2: UIButton::new(window.fb_width as u32 / 2, window.fb_height as u32 / 2 + 200, 42 * 10, 13 * 10, "Settings", AnchorCentered),
        }
    }

    fn update(&mut self, input: &Input) -> Option<Menu> {
        self.button1.cursor_update(input.mouse_x, input.mouse_y, input.is_mouse_button_down(MouseButton::Left as i32));
        self.button2.cursor_update(input.mouse_x, input.mouse_y, input.is_mouse_button_pressed(MouseButton::Left as i32));
        if self.button1.down {return Some(NoMenu);}
        None
    }

    fn render(&self, resource_manager: &ResourceManager) {
        self.button1.render();
        self.button2.render();
        resource_manager.font_tex.bind(0);
        self.button1.render_text();
        self.button2.render_text();
    }
}