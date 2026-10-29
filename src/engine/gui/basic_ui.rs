use crate::engine::{
    core::window::GameWindow, gui::basic_ui::{
        self,
        UIAnchor::{
            AnchorBottomLeft, AnchorBottomMid, AnchorBottomRight, AnchorCentered, AnchorMidLeft,
            AnchorMidRight, AnchorTopLeft, AnchorTopMid, AnchorTopRight,
        },
    }, resources::{resource_manager::{self, ResourceManager}, texture::{AtlasTile, UV}},
};

// Components
#[derive(Clone, Debug)]
pub struct UITransformC {
    pub x: i32,
    pub y: i32,
    pub tx: i32, // the transformed x, the original offset x is kept so that
    pub ty: i32, // the mesh can be rebuild with a different window aspect.
    pub w: u32,
    pub h: u32,
    pub anchor: UIAnchor
}

pub struct UIContainerC {
    elements: Vec<Box<dyn UIElement>>,
}

pub struct UIModelC {
    vao: u32,
    vbo: u32,
    vertex_count: u32,
}

impl UITransformC {
    pub fn new(x: i32, y: i32, w: u32, h: u32, anchor: UIAnchor) -> Self {
        Self {x, y, tx: 0, ty: 0, w, h, anchor}
    }

    pub fn update_transform(&mut self, parent_transform: &UITransformC) {
        self.align_to_parent(parent_transform);
        self.align_self();
    }

    fn align_to_parent(&mut self, pt: &UITransformC) {
        match &self.anchor {
            AnchorTopLeft => {
                self.tx = pt.tx;
                self.ty = pt.ty;
            }
            AnchorTopMid => {
                self.tx = pt.tx + pt.w as i32 / 2;
                self.ty = pt.ty;
            }
            AnchorTopRight => {
                self.tx = pt.tx + pt.w as i32;
                self.ty = pt.ty;
            }
            AnchorMidLeft => {
                self.tx = pt.tx;
                self.ty = pt.ty + pt.h as i32 / 2;
            }
            AnchorCentered => {
                self.tx = pt.tx + pt.w as i32 / 2;
                self.ty = pt.ty + pt.h as i32 / 2;
            }
            AnchorMidRight => {
                self.tx = pt.tx + pt.w as i32;
                self.ty = pt.ty + pt.h as i32 / 2;
            }
            AnchorBottomLeft => {
                self.tx = pt.tx;
                self.ty = pt.ty + pt.h as i32;
            }
            AnchorBottomMid => {
                self.tx = pt.tx + pt.w as i32 / 2;
                self.ty = pt.ty + pt.h as i32;
            }
            AnchorBottomRight => {
                self.tx = pt.tx + pt.w as i32;
                self.ty = pt.ty + pt.h as i32;
            }
        }
        // offset
        self.tx += self.x;
        self.ty += self.y;
    }

    fn align_self(&mut self) {
        match &self.anchor {
            AnchorTopLeft => {
                return;
            }
            AnchorTopMid => {
                self.tx -= self.w as i32 / 2;
            }
            AnchorTopRight => {
                self.tx -= self.w as i32;
            }
            AnchorMidLeft => {
                self.ty -= self.h as i32 / 2;
            }
            AnchorCentered => {
                self.tx -= self.w as i32 / 2;
                self.ty -= self.h as i32 / 2;
            }
            AnchorMidRight => {
                self.tx -= self.w as i32;
                self.ty -= self.h as i32 / 2;
            }
            AnchorBottomLeft => {
                self.ty -= self.h as i32;
            }
            AnchorBottomMid => {
                self.tx -= self.w as i32 / 2;
                self.ty -= self.h as i32;
            }
            AnchorBottomRight => {
                self.tx -= self.w as i32;
                self.ty -= self.h as i32;
            }
        }
    }
}

impl UIContainerC {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl UIModelC {
    fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            vertex_count: 0,
        }
    }

    fn draw(&self) {
        if self.vao != 0 && self.vbo != 0 {
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
                gl::BindVertexArray(0);
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &mut self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &mut self.vbo);
            }
        }
    }

    fn upload(&mut self, mesh: &[f32]) {
        self.destroy();

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            let float_size = std::mem::size_of::<f32>() as i32;
            let stride = 4 * float_size;

            gl::BufferData(
                gl::ARRAY_BUFFER,
                mesh.len() as isize * float_size as isize,
                mesh.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // position
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * float_size) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }
}

pub trait UIElement {
    fn get_transform(&self) -> &UITransformC;

    fn get_mut_transform(&mut self) -> &mut UITransformC;

    fn cursor_update(&mut self, mx: u32, my: u32, down: bool);

    fn is_hovered(&self, mx: u32, my: u32) -> bool {
        let t = self.get_transform();
        
        mx as i32 >= t.tx
        && my as i32 >= t.ty
        && mx as i32 <= t.tx + t.w as i32
        && my as i32 <= t.ty + t.h as i32
    }

    fn draw(&self) {}
}

pub trait UIContainer {
    fn get_elements(&self) -> &UIContainerC;

    fn get_mut_elements(&mut self) -> &mut UIContainerC;

    fn update_container_items(&mut self, mx: u32, my: u32) {
        let elements = &self.get_elements().elements;
        for item in elements {
            if item.is_hovered(mx, my) {
                return;
            }
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum UIAnchor {
    AnchorTopLeft,
    AnchorTopMid,
    AnchorTopRight,
    AnchorMidLeft,
    AnchorCentered,
    AnchorMidRight,
    AnchorBottomLeft,
    AnchorBottomMid,
    AnchorBottomRight,
}

// ---------- Nodes ----------

// Container
pub struct UIFrame {
    transform: UITransformC,
    container: UIContainerC,
}

pub struct UIGrid {
    transform: UITransformC,
    container: UIContainerC,
}

pub struct UIVStack {
    transform: UITransformC,
    container: UIContainerC,
}

pub struct UIHStack {
    transform: UITransformC,
    container: UIContainerC,
}

// Widgets
// implemented
pub struct UIButton {
    transform: UITransformC,
    model: UIModelC,
    text: UILabel,
    pub hovered: bool,
    pub pressed: bool,
}
pub struct UIIconButton {
    transform: UITransformC,
    model: UIModelC,
    pub hovered: bool,
    pub pressed: bool,
}
// implemented
pub struct UIToggle {
    transform: UITransformC,
}
// implemented
pub struct UICheckBox {
    transform: UITransformC,
    model: UIModelC,
    pub hovered: bool,
    pub pressed: bool,
    pub state: bool,
}
// implemented
pub struct UILabel {
    transform: UITransformC,
    model: UIModelC,
}
pub struct UIItemDisplay {
    transform: UITransformC,
}
pub struct UIEntityDisplay {
    transform: UITransformC,
}
pub struct UITextPanel {
    transform: UITransformC,
}
pub struct UIProgressBar {
    transform: UITransformC,
}
pub struct UIOptionButton {
    transform: UITransformC,
}
pub struct UISlider {
    transform: UITransformC,
}
pub struct UITextBox {
    transform: UITransformC,
}
// implemented
pub struct UITexturedRect {
    transform: UITransformC,
    pub model: UIModelC,
}

// Layout
struct UIPanel {
    transform: UITransformC,
}
pub struct UISeperator {
    transform: UITransformC,
}

// Inventory
pub struct UIContainerSlot {
    transform: UITransformC,
} // a single inventory slot with a textured background
pub struct UIContainerGrid {
    transform: UITransformC,
} // a grid of basic inventory slots

// ---------------------------


impl UIElement for UILabel {
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

impl UILabel {
    pub fn new(x: i32, y: i32, text: &str, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {
        let mut label = Self {
            transform: UITransformC::new(x, y, UILabel::get_label_width(text), 25, anchor),
            model: UIModelC::new(),
        };
        label.transform.update_transform(parent_transform);
        label.build_mesh(text);
        label
    }

    pub fn build_mesh(&mut self, text: &str) {

        let mut mesh: Vec<f32> = Vec::new();

        let tex_width = 128;
        let char_tex_aspect_x = 7.0 / tex_width as f32;
        let char_tex_aspect_y = 16.0 / tex_width as f32;
        let char_width_y = 8.0 / tex_width as f32;
        let char_tex_offset_y = 9.0 / tex_width as f32;

        let h: f32 = 25.0;
        let w: f32 = 25.0;

        let mut total_width: f32 = 0.0;
        for c in text.chars() {
            if c.is_ascii() && c >= '!' {
                let x: f32 = self.transform.tx as f32 + total_width;
                let y: f32 = self.transform.ty as f32;

                // the font atlas starts at ascii char '!'
                let l = c as u8 - b'!';
                let col = (l % 18) as f32;
                let row = (l / 18) as f32;

                let tex_x: f32 = col * char_tex_aspect_x;
                let tex_y: f32 = row * char_tex_aspect_y + char_tex_aspect_y - char_tex_offset_y;
                let tex_mx: f32 = tex_x + char_tex_aspect_x;
                let tex_my: f32 = tex_y + char_width_y;

                let rect: [f32; 24] = [
                    x + w, y + h, tex_mx, tex_my,
                    x + w, y, tex_mx, tex_y,
                    x, y, tex_x, tex_y,
                    x, y, tex_x, tex_y,
                    x, y + h, tex_x, tex_my,
                    x + w, y + h, tex_mx, tex_my,
                ];

                for f in rect {
                    mesh.push(f);
                }
            }
            total_width += UILabel::get_char_width(c) * w;
        }

        self.transform.w = total_width as u32;
        self.transform.h = h as u32;
        
        self.model.vertex_count = (mesh.len() / 4) as u32;

        self.model.upload(&mesh);
    }

    pub fn get_label_width(text: &str) -> u32 {
        let mut total_width = 0;
        for c in text.chars() {
            total_width += (UILabel::get_char_width(c) * 25.0) as u32;
        }

        total_width
    }

    fn get_char_width(c: char) -> f32 {
        let w: f32;
        let ps: f32 = 0.2;
        match c {
            '!' => w = ps * 2.0,
            '"' => w = ps * 4.0,
            '\'' => w = ps * 4.0,
            '(' => w = ps * 3.0,
            ')' => w = ps * 3.0,
            '*' => w = ps * 4.0,
            '+' => w = ps * 4.0,
            ',' => w = ps * 2.0,
            '-' => w = ps * 3.0,
            '.' => w = ps * 2.0,

            'l' => w = ps * 2.0,
            'i' => w = ps * 2.0,
            ':' => w = ps * 2.0,
            _ => w = ps * 4.0,
        }
        w
    }
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
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: &str, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {

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

        let x = self.transform.tx as f32;
        let y = self.transform.ty as f32;
        let w = self.transform.w as f32;
        let h = self.transform.h as f32;

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
}



impl UIElement for UICheckBox {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }

    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {
        if down && self.is_hovered(mx, my)
        {
            self.state = !self.state;
            self.create_model();
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
        let x = self.transform.tx as f32;
        let y = self.transform.ty as f32;
        let w = self.transform.w as f32;
        let h = self.transform.h as f32;

        let tex_pixel_size = 256 as f32;

        let tex_x;
        let tex_y;
        let tex_mx;
        let tex_my;

        if self.state == true {
            tex_x = 45 as f32 / tex_pixel_size;
            tex_y = 2 as f32 / tex_pixel_size;
            tex_mx = 58 as f32 / tex_pixel_size;
            tex_my = 15 as f32 / tex_pixel_size;
        } else {
            tex_x = 45 as f32 / tex_pixel_size;
            tex_y = 16 as f32 / tex_pixel_size;
            tex_mx = 58 as f32 / tex_pixel_size;
            tex_my = 29 as f32 / tex_pixel_size;
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
    pub fn new(x: i32, y: i32, w: u32, h: u32, uv: &UV, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {
        let mut tex_rect = Self {
            transform: UITransformC::new(x, y, w, h, anchor),
            model: UIModelC::new(),
        };
        tex_rect.transform.update_transform(&parent_transform);
        tex_rect.rebuild_model(uv);
        tex_rect
    }

    pub fn rebuild_model(&mut self, uv: &UV) {
        let x = self.transform.tx as f32;
        let y = self.transform.ty as f32;
        let w = self.transform.w as f32;
        let h = self.transform.h as f32;

        let tex_pixel_size = 256 as f32;

        let tex_x = uv.x;
        let tex_y = uv.y;
        let tex_mx = uv.x_max;
        let tex_my = uv.y_max;

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
}
