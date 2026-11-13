use crate::engine::{gui::basic_ui::{UIAnchor, UIElement, UIModelC, UITransformC}, resources::texture::UV};



pub struct UITexturedRect {
    pub uv: UV,
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

        let u = self.uv.x;
        let v = self.uv.y;
        let uw = self.uv.x_max;
        let vh = self.uv.y_max;

        let rect = UIModelC::get_rect_verts(x, y, w, h, u, v, uw, vh);

        self.model.upload(&rect);
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.rebuild_model();
    }
}