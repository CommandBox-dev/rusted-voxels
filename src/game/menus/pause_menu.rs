use crate::engine::{gui::{basic_ui::{UIAnchor::AnchorCentered, UIElement, UITransformC}, elements::button::UIButton}, resources::resource_manager::ResourceManager};

pub struct EscapeMenu {
    //label_game_state: UILabel,
    button_continue: UIButton,
    button_settings: UIButton,
    button_world_options: UIButton,
    button_leave: UIButton,
}

impl EscapeMenu {
    pub fn new(window_transform: &UITransformC) -> EscapeMenu {
        Self {
            button_continue: UIButton::new(
                0,
                -300,
                42 * 10,
                13 * 10,
                String::from("Continue"),
                AnchorCentered,
                window_transform,
            ),
            button_settings: UIButton::new(
                0,
                -100,
                42 * 10,
                13 * 10,
                String::from("Settings"),
                AnchorCentered,
                window_transform,
            ),
            button_world_options: UIButton::new(
                0,
                100,
                42 * 10,
                13 * 10,
                String::from("World Options"),
                AnchorCentered,
                window_transform,
            ),
            button_leave: UIButton::new(
                0,
                300,
                42 * 10,
                13 * 10,
                String::from("Leave"),
                AnchorCentered,
                window_transform,
            ),
        }
    }

    pub fn render(&self, resource_manager: &ResourceManager) {
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

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.button_continue.realign(parent_transform);
        self.button_settings.realign(parent_transform);
        self.button_world_options.realign(parent_transform);
        self.button_leave.realign(parent_transform);
    }
}