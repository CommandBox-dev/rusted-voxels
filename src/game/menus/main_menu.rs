use glfw::MouseButton;

use crate::engine::{core::{input::Input, window::GameWindow}, gui::{basic_ui::{UIAnchor::{AnchorCentered, AnchorRight, AnchorTopLeft}, UIElement, UITransformC}, elements::{button::UIButton, label::UILabel, tex_rect::UITexturedRect}, menu_manager::Menu}, resources::{resource_manager::ResourceManager, texture::UV}};

pub struct MainMenu {
    version_label: UILabel,
    pub button1: UIButton,
    pub button2: UIButton,
    background: UITexturedRect,
}

impl MainMenu {
    pub fn new(window_transform: &UITransformC) -> Self {
        let height_aspect = window_transform.width as f32 / window_transform.height as f32;
        Self {
            version_label: UILabel::new(
                15,
                900,
                String::from("Rusted-Voxels-v0.01"),
                AnchorRight,
                window_transform,
            ),
            button1: UIButton::new(
                0,
                -200,
                42 * 10,
                13 * 10,
                String::from("Play"),
                AnchorCentered,
                window_transform,
            ),
            button2: UIButton::new(
                0,
                200,
                42 * 10,
                13 * 10,
                String::from("Settings"),
                AnchorCentered,
                window_transform,
            ),
            background: UITexturedRect::new(
                0,
                0,
                window_transform.width as u32,
                (window_transform.height as f32 * height_aspect) as u32,
                UV::new(0.0, 0.0, 10.0, 10.0),
                AnchorTopLeft,
                window_transform,
            ),
        }
    }

    pub fn update(&mut self, input: &Input, window: &GameWindow) -> Option<Menu> {
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
            return Some(Menu::WorldLoadingScreen);
        }
        None
    }

    pub fn render(&self, resource_manager: &ResourceManager) {
        resource_manager.background_bricks_tex.bind(0);
        self.background.draw();
        resource_manager.ui_tex.bind(0);
        self.button1.draw();
        self.button2.draw();
        resource_manager.font_tex.bind(0);
        self.button1.draw_text();
        self.button2.draw_text();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.version_label.realign(parent_transform);
        self.button1.realign(parent_transform);
        self.button2.realign(parent_transform);
        self.background.realign(parent_transform);
    }
}