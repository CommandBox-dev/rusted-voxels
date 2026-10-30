use crate::engine::{entities::character_controller::CharacterState, gui::{basic_ui::{UIAnchor::{AnchorBottom, AnchorCentered, AnchorTopRight}, UIElement, UITransformC}, elements::tex_rect::UITexturedRect}, resources::{resource_manager::ResourceManager, texture::UV}};

pub struct GameHUD {
    crosshair: UITexturedRect,
    hotbar: UITexturedRect,
    selected_slot: UITexturedRect,
    special_slot_one: UITexturedRect,
    special_slot_two: UITexturedRect,
    health_bar: UITexturedRect,
}

impl GameHUD {
    pub fn new(window_transform: &UITransformC) -> Self {
        let tex_size = 256.0;
        let slot_size = 90; // 80
        let slot_half = slot_size / 2;

        // TODO: remove all "mid" from UIAnchor names, AnchorBottom -> AnchorBottom

        Self {
            crosshair: UITexturedRect::new(0, 0, 40, 40, UV::new(0.0, 0.0, 1.0, 1.0), AnchorCentered, window_transform),
            hotbar: UITexturedRect::new(0, -10, slot_size * 10, slot_size, UV::new(96.0 / tex_size, 0.0 / tex_size, tex_size / tex_size, 16.0 / tex_size), AnchorBottom, window_transform),
            selected_slot: UITexturedRect::new(-(4 * slot_size as i32 + slot_half as i32), -10, slot_size, slot_size, UV::new(96.0 / tex_size, 16.0 / tex_size, 112.0 / tex_size, 32.0 / tex_size), AnchorBottom, window_transform),
            special_slot_one: UITexturedRect::new(-(slot_half as i32 * 3), (slot_half) as i32, slot_size, slot_size, UV::new(112.0 / tex_size, 16.0 / tex_size, 128.0 / tex_size, 32.0 / tex_size), AnchorTopRight, window_transform),
            special_slot_two: UITexturedRect::new(-(slot_half as i32), (slot_size + slot_half) as i32, slot_size, slot_size, UV::new(112.0 / tex_size, 16.0 / tex_size, 128.0 / tex_size, 32.0 / tex_size), AnchorTopRight, window_transform),
            health_bar: UITexturedRect::new(-(5 * slot_half as i32), -(10 + slot_size as i32), slot_size * 5, slot_size, UV::new(98.0 / tex_size, 36.0 / tex_size, 110.0 / tex_size, 47.0 / tex_size), AnchorBottom, window_transform),
            
        }
    }

    pub fn update(&mut self, character_controller: &CharacterState, window_transform: &UITransformC) {
        let tex_size = 256.0;
        let slot_size: i32 = 90; // 80

        self.selected_slot.model.destroy();
        self.selected_slot = UITexturedRect::new((-(4 * slot_size + slot_size / 2) + character_controller.selected_slot as i32 * slot_size), -10, slot_size as u32, slot_size as u32, UV::new(96.0 / tex_size, 16.0 / tex_size, 112.0 / tex_size, 32.0 / tex_size), AnchorBottom, window_transform);
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        resource_manager.crosshair_tex.bind(0);
        unsafe {gl::BlendFunc(gl::ONE_MINUS_DST_COLOR, gl::ZERO)};

        self.crosshair.draw();

        unsafe {gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)};
        resource_manager.ui_tex.bind(0);

        self.hotbar.draw();
        self.selected_slot.draw();
        self.special_slot_one.draw();
        self.special_slot_two.draw();
        //self.health_bar.draw();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.crosshair.realign(parent_transform);
        self.hotbar.realign(parent_transform);
        self.selected_slot.realign(parent_transform);
        self.special_slot_one.realign(parent_transform);
        self.special_slot_two.realign(parent_transform);
        self.health_bar.realign(parent_transform);
    }
}