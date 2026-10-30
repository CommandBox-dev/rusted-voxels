use crate::engine::{gui::basic_ui::{UIAnchor, UIElement, UIModelC, UITransformC}, resources::texture::UV};



pub struct UITexturedRect {
    uv: UV,
    transform: UITransformC,
    pub model: UIModelC,
}

impl UIElement for UITexturedRect {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }

    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {}

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.model.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }
}

impl UITexturedRect {
    pub fn new(x: i32, y: i32, w: u32, h: u32, uv: UV, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {
        let mut tex_rect = Self {
            uv,
            transform: UITransformC::new(x, y, w, h, anchor),
            model: UIModelC::new(),
        };
        tex_rect.transform.update_transform(&parent_transform);
        tex_rect.rebuild_model();
        tex_rect
    }

    pub fn rebuild_model(&mut self) {
        let x = self.transform.visual_x as f32;
        let y = self.transform.visual_y as f32;
        let w = self.transform.width as f32;
        let h = self.transform.height as f32;

        let tex_pixel_size = 256 as f32;

        let tex_x = self.uv.x;
        let tex_y = self.uv.y;
        let tex_mx = self.uv.x_max;
        let tex_my = self.uv.y_max;

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

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.rebuild_model();
    }
}