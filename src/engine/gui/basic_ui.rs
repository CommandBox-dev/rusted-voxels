use crate::engine::{gui::basic_ui::{self, UIAnchor::{AnchorTopLeft, AnchorTopMid, AnchorTopRight, AnchorMidLeft, AnchorCentered, AnchorMidRight, AnchorBottomLeft, AnchorBottomMid, AnchorBottomRight,}}, resources::texture::AtlasTile, core::window::GameWindow};


// Components
pub struct UITransformC {x: u32, y: u32, w: u32, h: u32, anchor: UIAnchor}
pub struct UIContainerC {elements: Vec<Box<dyn UIElement>>,}
pub struct UIModelC {vao: u32, vbo: u32, vertex_count: u32}

#[derive(Copy, Clone)]
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

impl UITransformC {
    fn new(x: u32, y: u32, w: u32, h: u32, anchor: UIAnchor) -> Self {Self {x, y, w, h, anchor}}
    fn update_transform(&mut self, parent_transform: &UITransformC) {
        match &self.anchor {
            AnchorTopLeft => {
                self.x = parent_transform.x;
                self.y = parent_transform.y;
            },
            AnchorTopMid => {
                self.x = parent_transform.x + parent_transform.w / 2;
                self.y = parent_transform.y;
            },
            AnchorTopRight => {
                self.x = parent_transform.x + parent_transform.w;
                self.y = parent_transform.y;
            },
            AnchorMidLeft => {
                self.x = parent_transform.x;
                self.y = parent_transform.y + parent_transform.h / 2;
            },
            AnchorCentered => {
                self.x = parent_transform.x + parent_transform.w / 2;
                self.y = parent_transform.y + parent_transform.h / 2;
            },
            AnchorMidRight => {
                self.x = parent_transform.x + parent_transform.w;
                self.y = parent_transform.y + parent_transform.h / 2;
            },
            AnchorBottomLeft => {
                self.x = parent_transform.x;
                self.y = parent_transform.y + parent_transform.h;
            },
            AnchorBottomMid => {
                self.x = parent_transform.x + parent_transform.w / 2;
                self.y = parent_transform.y + parent_transform.h;
            },
            AnchorBottomRight => {
                self.x = parent_transform.x + parent_transform.w;
                self.y = parent_transform.y + parent_transform.h;
            },
        }
    }
    pub fn align_transform(&mut self) {
        match &self.anchor {
            AnchorTopLeft => {
                return;
            },
            AnchorTopMid => {
                self.x -= self.w / 2;
            },
            AnchorTopRight => {
                self.x -= self.w;
            },
            AnchorMidLeft => {
                self.y -= self.h / 2;
            },
            AnchorCentered => {
                self.x -= self.w / 2;
                self.y -= self.h / 2;
            },
            AnchorMidRight => {
                self.x -= self.w;
                self.y -= self.h / 2;
            },
            AnchorBottomLeft => {
                self.y -= self.h;
            },
            AnchorBottomMid => {
                self.x -= self.w / 2;
                self.y -= self.h;
            },
            AnchorBottomRight => {
                self.x -= self.w;
                self.y -= self.h;
            },
        }
    }
}

impl UIContainerC {
    fn new() -> Self {Self {elements: Vec::new()}}
    //fn add_element<E: UIElement + 'static>(&mut self, element: E) -> &E {
    //    self.elements.push(Box::new(element));
        //self.elements.last().unwrap().as_ref()
    //}
}

impl UIModelC {
    fn new() -> Self {
        Self {vao: 0, vbo: 0, vertex_count: 0}
    }
    fn draw(&self) {
        if self.vao != 0 && self.vbo != 0 {
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                gl::BindVertexArray(0);
            }
        }
    }
}

// ---------- Nodes ----------

// Containers
pub struct UIFrame {transform: UITransformC, container: UIContainerC}
pub struct UIGrid {transform: UITransformC, container: UIContainerC}
pub struct UIVStack {transform: UITransformC, container: UIContainerC}
pub struct UIHStack {transform: UITransformC, container: UIContainerC}

// Widgets
pub struct UIButton {transform: UITransformC, model: UIModelC, text: UILabel, pub down: bool, pub hovered: bool}
// symbol button
pub struct UIToggle {transform: UITransformC}
pub struct UICheckBox {transform: UITransformC, model: UIModelC, pub state: bool}
pub struct UILabel {transform: UITransformC, model: UIModelC}
pub struct UIItemDisplay {transform: UITransformC}
pub struct UIEntityDisplay {transform: UITransformC}
pub struct UITextPanel {transform: UITransformC}
pub struct UIProgressBar {transform: UITransformC}
pub struct UIOptionButton {transform: UITransformC}
pub struct UISlider {transform: UITransformC}
pub struct UITextBox {transform: UITransformC}
pub struct UITexturedRect {transform: UITransformC}

// Layout
struct UIPanel {transform: UITransformC}
pub struct UISeperator {transform: UITransformC}

// Inventory
pub struct UIContainerSlot {transform: UITransformC} // a single inventory slot with a textured background
pub struct UIContainerGrid {transform: UITransformC} // a grid of basic inventory slots

// ---------------------------


pub trait UIElement {
    fn get_transform(&self) -> &UITransformC;
    fn get_mut_transform(&mut self) -> &mut UITransformC;
    fn cursor_update(&mut self, mx: u32, my: u32, down: bool);

    fn hovered(&self, mx: u32, my: u32) -> bool {
        self.get_transform();
        false
    }
}

pub trait UIContainer {
    fn get_elements(&self) -> &UIContainerC;
    fn get_mut_elements(&mut self) -> &mut UIContainerC;

    fn update_container_items(&mut self, mx: u32, my: u32) {
        let elements = &self.get_elements().elements;
        for item in elements {
            if item.hovered(mx, my) {return;}
        }
    }

    //fn add_element<E: UIElement + 'static>(&mut self, mut element: E) -> &E where Self: UIElement {
    //    element.get_mut_transform().update_transform(self.get_transform());
        //self.get_mut_elements().add_element(element)
    //}
}

impl UIFrame {
    pub fn new(w: u32, h: u32, anchor: UIAnchor) -> Self {
        Self {
            transform: UITransformC::new(0, 0, w, h, anchor),
            container: UIContainerC::new(),
        }
    }
}

/*impl UIElement for UIFrame {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIGrid {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIVStack {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIHStack {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIToggle {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UICheckBox {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIItemDisplay {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIEntityDisplay {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UITextPanel {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIProgressBar {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UIOptionButton {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UISlider {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UITextBox {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}
impl UIElement for UITexturedRect {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
}


impl UIContainer for UIFrame {
    fn get_elements(&self) -> &UIContainerC {&self.container}
    fn get_mut_elements(&mut self) -> &mut UIContainerC {&mut self.container}
}
impl UIContainer for UIVStack {
    fn get_elements(&self) -> &UIContainerC {&self.container}
    fn get_mut_elements(&mut self) -> &mut UIContainerC {&mut self.container}
}
impl UIContainer for UIHStack {
    fn get_elements(&self) -> &UIContainerC {&self.container}
    fn get_mut_elements(&mut self) -> &mut UIContainerC {&mut self.container}
}
impl UIContainer for UIGrid {
    fn get_elements(&self) -> &UIContainerC {&self.container}
    fn get_mut_elements(&mut self) -> &mut UIContainerC {&mut self.container}
}*/


fn upload_rect(window: &GameWindow, tex: &AtlasTile) -> (u32, u32) {
    let size: i32 = 40;
    let x: f32 = (window.fb_width / 2 - size / 2) as f32;
    let y: f32 = (window.fb_height / 2 - size / 2) as f32;
    let w: f32 = size as f32;
    let h: f32 = size as f32;

    let quad_mesh: [f32; 24] = [
        x + w, y + h, tex.xm, tex.ym,
        x + w, y, tex.xm, tex.y,
        x, y, tex.x,  tex.y,

        x, y, tex.xm, tex.ym,
        x, y + h, tex.x,  tex.ym,
        x + w, y + h, tex.x,  tex.y,
    ];

    let vertex_count = (quad_mesh.len() / 4) as u32;

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let float_size = std::mem::size_of::<f32>() as i32;
        let stride = 4 * float_size;

        gl::BufferData(gl::ARRAY_BUFFER, (quad_mesh.len() * float_size as usize) as isize, quad_mesh.as_ptr() as *const _, gl::STATIC_DRAW);

        // position
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // uv
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * float_size) as *const _
        );
        gl::EnableVertexAttribArray(1);
        
        gl::BindVertexArray(0);
    }
    (vao, vbo)
}

fn destroy_rect(model: &mut UIModelC) {
    unsafe {
        if model.vao != 0 {gl::DeleteVertexArrays(1, &mut model.vao);}
        if model.vbo != 0 {gl::DeleteBuffers(1, &mut model.vbo);}
    }
    model.vao = 0;
    model.vbo = 0;
}








impl UIElement for UILabel {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {}
}

impl UILabel {
    pub fn new(x: u32, y: u32, text: &str, anchor: UIAnchor) -> Self {
        let mut label = Self {
            transform: UITransformC::new(x, y, 0, 0, anchor), // can't align transform
            model: UIModelC::new(),
        };
        label.build_text_mesh(text);
        label
    }

    pub fn build_text_mesh(&mut self, text: &str) { // build model
        unsafe {
            if self.model.vao != 0 {gl::DeleteVertexArrays(1, &mut self.model.vao);}
            if self.model.vbo != 0 {gl::DeleteBuffers(1, &mut self.model.vbo);}
        }

        let mut verts: Vec<f32> = Vec::new();

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
                
                let x: f32 = self.transform.x as f32 + total_width;
                let y: f32 = self.transform.y as f32; 

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
                    x, y, tex_x,  tex_y,

                    x, y, tex_x, tex_y,
                    x, y + h, tex_x,  tex_my,
                    x + w, y + h, tex_mx,  tex_my,
                ];
                for f in rect {
                    verts.push(f);
                }
            }
            total_width += UILabel::get_char_width(c) * w;
        }

        self.transform.w = total_width as u32;
        self.transform.h = h as u32;

        self.model.vertex_count = (verts.len() / 4) as u32;
        
        unsafe {

            gl::GenVertexArrays(1, &mut self.model.vao);
            gl::GenBuffers(1, &mut self.model.vbo);
            
            gl::BindVertexArray(self.model.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);

            let float_size = std::mem::size_of::<f32>() as i32;
            let stride = 4 * float_size;

            gl::BufferData(gl::ARRAY_BUFFER, (verts.len() * float_size as usize) as isize, verts.as_ptr() as *const _, gl::STATIC_DRAW);

            // position
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * float_size) as *const _
            );
            gl::EnableVertexAttribArray(1);
            
            gl::BindVertexArray(0);
        }
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

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.model.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }
}



impl UIElement for UIButton {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {
        if down &&
        mx >= self.transform.x &&
        my >= self.transform.y &&
        mx <= self.transform.x + self.transform.w &&
        my <= self.transform.y + self.transform.h
        {self.down = true; self.create_model();} else
        {self.down = false; self.create_model();}
    }
}

impl UIButton {
    pub fn new(x: u32, y: u32, w: u32, h: u32, text: &str, anchor: UIAnchor) -> Self {
        let mut button = Self {
            transform: UITransformC::new(x, y, w, h, anchor),
            model: UIModelC::new(),
            text: UILabel::new(x, y, text, anchor),
            down: false,
            hovered: false,
        };
        button.transform.align_transform();
        button.text.transform.align_transform();
        button.text.build_text_mesh(text);
        button.create_model();
        button
    }

    fn create_model(&mut self) {
        let x = self.transform.x as f32;
        let y = self.transform.y as f32;
        let w = self.transform.w as f32;
        let h = self.transform.h as f32;

        let tex_pixel_size = 256 as f32;

        let tex_x;
        let tex_y;
        let tex_mx;
        let tex_my;

        if self.down {
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
            x, y, tex_x,  tex_y,

            x, y, tex_x, tex_y,
            x, y + h, tex_x,  tex_my,
            x + w, y + h, tex_mx,  tex_my,
        ];

        self.model.vertex_count = (rect.len() / 4) as u32;

        unsafe {
            if self.model.vao != 0 {gl::DeleteVertexArrays(1, &mut self.model.vao);}
            if self.model.vbo != 0 {gl::DeleteBuffers(1, &mut self.model.vbo);}
        }
        
        unsafe {

            gl::GenVertexArrays(1, &mut self.model.vao);
            gl::GenBuffers(1, &mut self.model.vbo);
            
            gl::BindVertexArray(self.model.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);

            let float_size = std::mem::size_of::<f32>() as i32;
            let stride = 4 * float_size;

            gl::BufferData(gl::ARRAY_BUFFER, (rect.len() * float_size as usize) as isize, rect.as_ptr() as *const _, gl::STATIC_DRAW);

            // position
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * float_size) as *const _
            );
            gl::EnableVertexAttribArray(1);
            
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.model.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }

    pub fn render_text(&self) {
        self.text.render();
    }
}


impl UIElement for UICheckBox {
    fn get_transform(&self) -> &UITransformC {&self.transform}
    fn get_mut_transform(&mut self) -> &mut UITransformC {&mut self.transform}
    fn cursor_update(&mut self, mx: u32, my: u32, down: bool) {
        if down &&
        mx >= self.transform.x &&
        my >= self.transform.y &&
        mx <= self.transform.x + self.transform.w &&
        my <= self.transform.y + self.transform.h
        {self.state = !self.state; self.create_model();}
    }
}

impl UICheckBox {
    pub fn new(x: u32, y: u32, s: u32, state: bool, anchor: UIAnchor) -> Self {
        let mut button = Self {
            transform: UITransformC::new(x, y, s, s, anchor),
            model: UIModelC::new(),
            state,
        };
        button.transform.align_transform();
        button.create_model();
        button
    }

    fn create_model(&mut self) {
        let x = self.transform.x as f32;
        let y = self.transform.y as f32;
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
            x, y, tex_x,  tex_y,

            x, y, tex_x, tex_y,
            x, y + h, tex_x,  tex_my,
            x + w, y + h, tex_mx,  tex_my,
        ];

        self.model.vertex_count = (rect.len() / 4) as u32;

        unsafe {
            if self.model.vao != 0 {gl::DeleteVertexArrays(1, &mut self.model.vao);}
            if self.model.vbo != 0 {gl::DeleteBuffers(1, &mut self.model.vbo);}
        }
        
        unsafe {

            gl::GenVertexArrays(1, &mut self.model.vao);
            gl::GenBuffers(1, &mut self.model.vbo);
            
            gl::BindVertexArray(self.model.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);

            let float_size = std::mem::size_of::<f32>() as i32;
            let stride = 4 * float_size;

            gl::BufferData(gl::ARRAY_BUFFER, (rect.len() * float_size as usize) as isize, rect.as_ptr() as *const _, gl::STATIC_DRAW);

            // position
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // uv
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * float_size) as *const _
            );
            gl::EnableVertexAttribArray(1);
            
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.model.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }
}