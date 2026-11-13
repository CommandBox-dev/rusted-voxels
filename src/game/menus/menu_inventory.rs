use glam::Mat4;

use crate::{engine::{core::{input::Input, window::GameWindow}, entity::character_controller::CharacterState, gui::{basic_ui::{UI_SCALE, UIAnchor::{self, AnchorBottom, AnchorCentered, AnchorTopRight}, UIElement, UITransformC}, elements::{container_grid::UIContainerGrid, slider::UISlider, tex_rect::UITexturedRect}, layout::panel::UIPanel}, item::item::ItemInstance, rendering::renderering_server::RenderingServer, resources::{resource_manager::ResourceManager, texture::UV}, storage::container::Container}, game::data::game_data::GameData};

pub struct PlayerInventory {
    inventory: UIContainerGrid,
    background: UIPanel,
    hovered_slot: UITexturedRect,
    test_slider: UISlider,
    black_background: UITexturedRect,
}

impl PlayerInventory {
    pub fn new(
        window_transform: &UITransformC,
        container: &[Option<ItemInstance>],
    ) -> Self {
        let tex_size = 256.0;
        let slot_size = UI_SCALE as u32; // 90 // 80
        let slot_half = slot_size / 2;

        let uv = UV::new(128.0 / tex_size, 80.0 / tex_size, 144.0 / tex_size, 96.0 / tex_size);
        let black_uv = UV::new(96.0 / tex_size, 96.0 / tex_size, 112.0 / tex_size, 112.0 / tex_size);

        Self {
            inventory: UIContainerGrid::new(0, 0, 60, 10, UIAnchor::AnchorCentered, window_transform, container),
            background: UIPanel::new(0, 0, (10 + 1) * 100, (6 + 3) * 100, UIAnchor::AnchorCentered, window_transform),
            hovered_slot: UITexturedRect::new(0, 0, slot_size, slot_size, uv, UIAnchor::AnchorTopLeft, window_transform),
            test_slider: UISlider::new(0, 380, 800, 100, String::from("Slider"), UIAnchor::AnchorCentered, window_transform),
            black_background: UITexturedRect::new(0, 0, window_transform.width, window_transform.height, black_uv, UIAnchor::AnchorTopLeft, window_transform),
        }
    }

    pub fn update(&mut self, container: &mut Container, input: &Input) {
        let slot_size = 100;
        let hovered_slot = self.inventory.hovered_slot(input.mouse_x, input.mouse_y);

        self.inventory.input_container(container, input);

        if let Some(hovered_slot) = hovered_slot {
            let slot_x = hovered_slot % self.inventory.slots_row;
            let slot_y = hovered_slot / self.inventory.slots_row;

            let mut t = self.hovered_slot.get_mut_transform();
            let o = self.inventory.get_transform();

            t.visual_x = o.visual_x + slot_x as i32 * slot_size;
            t.visual_y = o.visual_y + slot_y as i32 * slot_size;
            self.hovered_slot.rebuild_model();
        }
    }

    pub fn draw(
        &self,
        character_controller: &CharacterState,
        resource_manager: &ResourceManager,
        renderering_server: &RenderingServer,
        game_data: &GameData,
        ui_proj: &Mat4,
        window: &GameWindow,
        
    ) {
        resource_manager.ui_tex.bind(0);

        self.black_background.draw();
        self.background.draw();
        self.inventory.draw();
        self.inventory.draw_items(
            &character_controller.inventory.slots,
            game_data,
            ui_proj,
            window,
            renderering_server,
            resource_manager
        );
        resource_manager.ui_shader.bind();
        resource_manager.font_tex.bind(0);

        self.inventory.draw_quantity(resource_manager);

        resource_manager.ui_tex.bind(0);
        self.hovered_slot.draw();
        self.test_slider.draw();
        resource_manager.font_tex.bind(0);
        self.test_slider.draw_text();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.inventory.realign(parent_transform);
        self.background.realign(parent_transform);
    }
}