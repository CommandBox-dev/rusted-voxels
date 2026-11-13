// A visual grid of basic inventory container slots

use glam::Mat4;
use glfw::MouseButton;
use toml::to_string;

use crate::{engine::{core::{input::Input, window::GameWindow}, entity::character_controller::{self, CharacterState}, gui::{basic_ui::{UI_SCALE, UIAnchor, UIElement, UIModelC, UITransformC}, elements::label::{self, UILabel}}, item::item::{ItemData::{self, BlockData}, ItemInstance, ItemType}, rendering::renderering_server::{self, RenderingServer}, resources::resource_manager::{self, ResourceManager}, storage::container::Container}, game::data::game_data::GameData};

pub struct UIContainerGrid {
    transform: UITransformC,
    model: UIModelC,
    model_quant: UIModelC,
    pub slot_count: u32,
    pub slots_row: u32,
}



impl UIElement for UIContainerGrid {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }
    
    fn draw(&self) {
        self.model.draw();
    }
}

impl UIContainerGrid {
    pub fn new(x: i32, y: i32, slots: u32, slots_row: u32, anchor: UIAnchor, parent_transform: &UITransformC, container: &[Option<ItemInstance>]) -> Self {
        let slot_size = 100;

        let rows = (slots as f32 / slots_row as f32).ceil() as u32;
        let columns = slots_row;

        let row_width = columns * slot_size;
        let column_height = rows * slot_size; 

        let mut grid = Self {
            transform: UITransformC::new(x, y, row_width, column_height, anchor),
            model: UIModelC::new(),
            model_quant: UIModelC::new(),
            slot_count: slots,
            slots_row,
        };
        grid.transform.update_transform(parent_transform);
        grid.build_model();
        grid.build_quantity_model(container);
        grid
    }

    fn build_model(&mut self) {
        let slot_size = UI_SCALE as u32;
        let tex_pixel_size = 256 as f32;

        let x = self.transform.visual_x as f32;
        let y = self.transform.visual_y as f32;
        let w = slot_size as f32;
        let h = slot_size as f32;

        let u = 96 as f32 / tex_pixel_size;
        let v = 80 as f32 / tex_pixel_size;
        let uw = 112 as f32 / tex_pixel_size;
        let vh = 96 as f32 / tex_pixel_size;

        let mut verts: Vec<f32> = Vec::new();

        for slot in 0..self.slot_count {

            let sx = slot % self.slots_row;
            let sy = slot / self.slots_row;

            let x = x + (sx * slot_size) as f32;
            let y = y + (sy * slot_size) as f32;

            verts.extend_from_slice(&UIModelC::get_rect_verts(x, y, w, h, u, v, uw, vh));
        }

        self.model.upload(&verts);
    }

    fn build_quantity_model(
        &mut self,
        container: &[Option<ItemInstance>],
    ) {
        let pixel_scale = UI_SCALE;
        let half = pixel_scale / 2.0;

        let mut mesh = Vec::new();

        for slot in 0..container.len() {
            if let Some(item) = &container[slot] {

                let slot_x = slot as u32 % self.slots_row;
                let slot_y = slot as u32 / self.slots_row;

                let str_mesh = label::create_mesh_from_str(
                    &item.quantity.to_string(),
                    self.transform.visual_x as f32 + (slot_x + 1) as f32 * pixel_scale - 25.0 * 2.0,
                    self.transform.visual_y as f32 + (slot_y + 1) as f32 * pixel_scale - 25.0 * 1.0,
                );

                mesh.extend_from_slice(&str_mesh);
            }
        }

        self.model_quant.upload(&mesh);
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.build_model();
    }

    /// Returns the index of the hovered slot or `None`
    pub fn hovered_slot(&self, mx: u32, my: u32) -> Option<u32> {
        let slot_size = UI_SCALE as u32;

        let rows = (self.slot_count as f32 / self.slots_row as f32) as u32;
        let columns = self.slots_row;

        let row_width = columns * slot_size;
        let column_height = rows * slot_size; 

        if (mx as i32) < self.transform.visual_x
        || (mx as i32) > self.transform.visual_x + (self.transform.width as i32)
        || (my as i32) < self.transform.visual_y
        || (my as i32) > self.transform.visual_y + (self.transform.height as i32) {
            return None;
        }

        let slot_x = (((mx as i32 - self.transform.visual_x) as f32 / self.transform.width as f32) * columns as f32) as u32;
        let slot_y = (((my as i32 - self.transform.visual_y) as f32 / self.transform.height as f32) * rows as f32) as u32;

        if slot_x > columns
        || slot_y > rows {
            return None;
        }

        let index = slot_y * columns + slot_x;

        if index >= self.slot_count {
            return None;
        }

        Some(index)
    }

    pub fn draw_items(
        &self,
        container: &[Option<ItemInstance>],
        game_data: &GameData,
        ui_proj: &Mat4,
        window: &GameWindow,
        renderering_server: &RenderingServer,
        resource_manager: &ResourceManager,
    ) {
        let pixel_scale = UI_SCALE;
        let half = pixel_scale / 2.0;

        for slot in 0..container.len() {
            if let Some(item) = &container[slot] {
                let item_def = &game_data.items.entries[item.item_id as usize];

                if item_def.item_type == ItemType::Block
                && let ItemData::BlockData(data) = item_def.item_data {
                    let slot_x = slot as u32 % self.slots_row;
                    let slot_y = slot as u32 / self.slots_row;

                    renderering_server.render_item_ortho(
                        data as u32,
                        self.transform.visual_x as f32 + slot_x as f32 * pixel_scale + half,
                        self.transform.visual_y as f32 + slot_y as f32 * pixel_scale + half,
                        ui_proj,
                        window,
                        resource_manager
                    );
                }
            }
        }
    }

    pub fn draw_quantity(
        &self,
        resource_manager: &ResourceManager,
    ) {
        self.model_quant.draw();
    }

    pub fn input_container(&mut self, container: &mut Container, input: &Input) {
        if !input.is_mouse_button_down(MouseButton::Left as i32) {
            return;
        }
        if let Some(hovered_slot) = self.hovered_slot(input.mouse_x, input.mouse_y) {
            if hovered_slot >= container.slots.len() as u32 {
                return;
            }
            container.take_item(hovered_slot);
        }
    }
}
