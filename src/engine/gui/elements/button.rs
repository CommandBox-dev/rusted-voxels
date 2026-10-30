use crate::engine::gui::{basic_ui::{UIAnchor::{self, *}, UIElement, UIModelC, UITransformC}, elements::label::UILabel};

pub struct UIButton {
    transform: UITransformC,
    model: UIModelC,
    text: UILabel,
    pub hovered: bool,
    pub pressed: bool,
}

impl UIElement for UIButton {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }

    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {
        if down && self.is_hovered(mx, my) 
        {
            self.hovered = true;
            self.pressed = true;
            //self.create_model();
        } else {
            self.hovered = false;
            self.pressed = false;
            //self.create_model();
        }
    }

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.model.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }
}

impl UIButton {
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: String, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {

        let mut transform = UITransformC::new(x, y, w, h, anchor);
        transform.update_transform(parent_transform);
        let mut label = UILabel::new(0, 0, text, AnchorCentered, &transform);

        let mut button = Self {
            transform,
            model: UIModelC::new(),
            text: label,
            hovered: false,
            pressed: false,
        };
        button.create_model();
        button
    }

    fn create_model(&mut self) {

        let x = self.transform.visual_x as f32;
        let y = self.transform.visual_y as f32;
        let w = self.transform.width as f32;
        let h = self.transform.height as f32;

        let tex_pixel_size = 256 as f32;

        let tex_x;
        let tex_y;
        let tex_mx;
        let tex_my;

        if self.pressed {
            tex_x = 2 as f32 / tex_pixel_size;
            tex_y = 16 as f32 / tex_pixel_size;
            tex_mx = 44 as f32 / tex_pixel_size;
            tex_my = 29 as f32 / tex_pixel_size;
        } else {
            tex_x = 2 as f32 / tex_pixel_size;
            tex_y = 2 as f32 / tex_pixel_size;
            tex_mx = 44 as f32 / tex_pixel_size;
            tex_my = 15 as f32 / tex_pixel_size;
        }

        let rect: [f32; 24] = [
            x + w, y + h, tex_mx, tex_my,
            x + w, y, tex_mx, tex_y,
            x, y, tex_x, tex_y,
            x, y, tex_x, tex_y,
            x, y + h, tex_x, tex_my,
            x + w, y + h, tex_mx, tex_my,
        ];

        self.model.vertex_count = (rect.len() / 4) as u32;

        self.model.upload(&rect);
    }

    pub fn draw_text(&self) {
        self.text.draw();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.text.realign(&self.transform);
        self.create_model();
    }
}
