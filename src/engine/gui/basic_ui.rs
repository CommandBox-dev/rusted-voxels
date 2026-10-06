use crate::engine::{gui::basic_ui, texture::AtlasTile, window::GameWindow};


// Components
struct UITransformC {x: u32, y: u32, w: u32, h: u32, anchor: UIAnchor}
struct UIContainerC {elements: Vec<Box<dyn UIElement>>,}
struct UIModelC {vao: u32, vbo: u32, vertex_count: u32}

pub enum UIAnchor {
    AnchorTopLeft,
    AnchorTopMid,
    AnchorTopRight,
    AnchorMidLeft,
    AnchorMiddle,
    AnchorMidRight,
    AnchorBottomLeft,
    AnchorBottomMid,
    AnchorBottomRight,
}

impl UITransformC {
    fn new(x: u32, y: u32, w: u32, h: u32, anchor: UIAnchor) -> Self {Self {x, y, w, h, anchor}}
}

impl UIContainerC {
    fn new() -> Self {Self {elements: Vec::new()}}
    fn add_element<T: UIElement + 'static>(&mut self, element: T) {
        self.elements.push(Box::new(element));
    }
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
pub struct UIButton {transform: UITransformC}
pub struct UIToggle {transform: UITransformC}
pub struct UICheckBox {transform: UITransformC}
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


trait UIElement {
    fn get_transform(&self) -> &UITransformC;
    fn get_mut_transform(&mut self) -> &mut UITransformC;

    fn hovered(&self, mx: u32, my: u32) -> bool {
        self.get_transform();
        false
    }
}

trait UIContainer {
    fn get_elements(&self) -> &UIContainerC;
    fn get_mut_elements(&mut self) -> &mut UIContainerC;

    fn update_container_items(container: &UIContainerC, mx: u32, my: u32) {
        for item in &container.elements {
            if item.hovered(mx, my) {return;}
        }
    }
}

impl UIFrame {
    fn new(x: u32, y: u32, w: u32, h: u32, anchor: UIAnchor) -> Self {
        Self {
            transform: UITransformC::new(x, y, w, h, anchor),
            container: UIContainerC::new(),
        }
    }
}

impl UIElement for UIFrame {
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
impl UIElement for UIButton {
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
}

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

impl UILabel {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            transform: UITransformC::new(x, y, 0, 0, UIAnchor::AnchorMiddle),
            model: UIModelC::new(),
        }
    }

    pub fn build_text_mesh(&mut self, text: &str) {
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
        
        let mut index: f32 = 0.0;
        for c in text.chars() {
            if c.is_ascii() && c >= '!' {
                
                let x: f32 = self.transform.x as f32 + index;
                let y: f32 = self.transform.y as f32; 

                // the atlas starts at ascii char '!'
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
            index += UILabel::get_char_width(c) * w;
        }

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
            '-' => w = ps * 4.0,
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
            //println!("{}", self.model.vertex_count);

            gl::BindVertexArray(0);
        }
    }
}