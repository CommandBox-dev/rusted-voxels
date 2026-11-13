use glfw::MouseButton;

use crate::engine::{core::input::Input, gui::basic_ui::{UI_TEX_PIXEL_SIZE, UIAnchor, UIElement, UIModelC, UITransformC}};



pub struct UICheckBox {
    transform: UITransformC,
    model: UIModelC,
    pub hovered: bool,
    pub pressed: bool,
    pub state: bool,
}



impl UIElement for UICheckBox {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }

    fn update(&mut self, input: &Input) {
        self.update_hover(input);

        let hovered = self.transform.hovered;
        let clicked = input.is_mouse_button_pressed(MouseButton::Left as i32);
        
        if clicked && self.hovered {
            self.state = !self.state;
            self.create_model();
        }
    }

    fn draw(&self) {
        self.model.draw();
    }
}

impl UICheckBox {
    pub fn new(x: i32, y: i32, s: u32, state: bool, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {
        let mut button = Self {
            transform: UITransformC::new(x, y, s, s, anchor),
            model: UIModelC::new(),
            hovered: false,
            pressed: false,
            state,
        };
        button.transform.update_transform(parent_transform);
        button.create_model();
        button
    }

    fn create_model(&mut self) {
        let x = self.transform.visual_x as f32;
        let y = self.transform.visual_y as f32;
        let w = self.transform.width as f32;
        let h = self.transform.height as f32;

        let u;
        let v;
        let uw;
        let vh;

        if self.state == true {
            u = 45 as f32 / UI_TEX_PIXEL_SIZE;
            v = 2 as f32 / UI_TEX_PIXEL_SIZE;
            uw = 58 as f32 / UI_TEX_PIXEL_SIZE;
            vh = 15 as f32 / UI_TEX_PIXEL_SIZE;
        } else {
            u = 45 as f32 / UI_TEX_PIXEL_SIZE;
            v = 16 as f32 / UI_TEX_PIXEL_SIZE;
            uw = 58 as f32 / UI_TEX_PIXEL_SIZE;
            vh = 29 as f32 / UI_TEX_PIXEL_SIZE;
        }

        let rect = UIModelC::get_rect_verts(x, y, w, h, u, v, uw, vh);
        
        self.model.upload(&rect);
    }
}
