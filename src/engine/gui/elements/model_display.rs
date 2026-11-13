use glam::Mat4;

use crate::engine::gui::basic_ui::{UIAnchor, UIElement, UITransformC};

pub struct UIModelDisplay {
    transform: UITransformC,
    model_matrix: Mat4,
    model_id: u32,
}

impl UIElement for UIModelDisplay {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }
}

impl UIModelDisplay {
    pub fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        model_id: u32,
        anchor: UIAnchor,
        parent_transform: &UITransformC
    ) -> Self {
        todo!();
    }

    pub fn draw() {
        
    }
}