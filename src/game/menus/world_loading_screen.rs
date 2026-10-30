use crate::engine::{gui::{basic_ui::{UIAnchor::{AnchorCentered, AnchorTopLeft}, UIElement, UITransformC}, elements::{label::UILabel, tex_rect::UITexturedRect}}, resources::{resource_manager::ResourceManager, texture::UV}, world};

pub struct WorldLoadingScreenMenu {
    background: UITexturedRect,
    label: UILabel,
}

impl WorldLoadingScreenMenu {
    pub fn new(window_transform: &UITransformC) -> Self {
        let height_aspect = window_transform.width as f32 / window_transform.height as f32;
        let mut s = Self {
            background: UITexturedRect::new(
                0,
                0,
                window_transform.width as u32,
                (window_transform.height as f32 * height_aspect) as u32,
                UV::new(0.0, 0.0, 10.0, 10.0),
                AnchorTopLeft,
                window_transform,
            ),
            label: UILabel::new(
                0,
                0,
                String::from("label.loading_progress"),
                AnchorCentered,
                window_transform,
            ),
        };
        s
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        resource_manager.background_tex.bind(0);
        self.background.draw();
        resource_manager.font_tex.bind(0);
        self.label.draw();
    }

    pub fn update(&mut self, chunk_count: u32) {
        self.label.set_text(
            format!(
                "Loaded Chunks: {}/{}",
                chunk_count,
                world::world::RENDER_AREA - 100
            ),
        );
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.background.realign(parent_transform);
        self.label.realign(parent_transform);
    }
}