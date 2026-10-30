use crate::engine::{
    core::window::GameWindow, gui::basic_ui::{
        self,
        UIAnchor::{
            AnchorBottomLeft, AnchorBottom, AnchorBottomRight, AnchorCentered, AnchorLeft,
            AnchorRight, AnchorTopLeft, AnchorTop, AnchorTopRight,
        },
    }, resources::{resource_manager::{self, ResourceManager}, texture::{AtlasTile, UV}},
};

// Components
#[derive(Clone, Debug)]
pub struct UITransformC {
    pub offset_x: i32,
    pub offset_y: i32,

    pub width: u32,
    pub height: u32,

    pub anchor: UIAnchor,

    pub visual_x: i32,
    pub visual_y: i32,
}

impl UITransformC {
    pub fn new(offset_x: i32, offset_y: i32, width: u32, height: u32, anchor: UIAnchor) -> Self {
        Self {
            offset_x,
            offset_y,

            width,
            height,

            anchor,

            visual_x: 0,
            visual_y: 0,
        }
    }

    pub fn update_transform(&mut self, parent_transform: &UITransformC) {
        self.align_to_parent(parent_transform);
        self.align_self();
    }

    fn align_to_parent(&mut self, parent_transform: &UITransformC) {
        let pt = parent_transform;

        match &self.anchor {
            AnchorTopLeft => {
                self.visual_x = pt.visual_x;
                self.visual_y = pt.visual_y;
            }
            AnchorTop => {
                self.visual_x = pt.visual_x + pt.width as i32 / 2;
                self.visual_y = pt.visual_y;
            }
            AnchorTopRight => {
                self.visual_x = pt.visual_x + pt.width as i32;
                self.visual_y = pt.visual_y;
            }
            AnchorLeft => {
                self.visual_x = pt.visual_x;
                self.visual_y = pt.visual_y + pt.height as i32 / 2;
            }
            AnchorCentered => {
                self.visual_x = pt.visual_x + pt.width as i32 / 2;
                self.visual_y = pt.visual_y + pt.height as i32 / 2;
            }
            AnchorRight => {
                self.visual_x = pt.visual_x + pt.width as i32;
                self.visual_y = pt.visual_y + pt.height as i32 / 2;
            }
            AnchorBottomLeft => {
                self.visual_x = pt.visual_x;
                self.visual_y = pt.visual_y + pt.height as i32;
            }
            AnchorBottom => {
                self.visual_x = pt.visual_x + pt.width as i32 / 2;
                self.visual_y = pt.visual_y + pt.height as i32;
            }
            AnchorBottomRight => {
                self.visual_x = pt.visual_x + pt.width as i32;
                self.visual_y = pt.visual_y + pt.height as i32;
            }
        }
        
        self.visual_x += self.offset_x;
        self.visual_y += self.offset_y;
    }

    fn align_self(&mut self) {
        match &self.anchor {
            AnchorTopLeft => {
                return;
            }
            AnchorTop => {
                self.visual_x -= self.width as i32 / 2;
            }
            AnchorTopRight => {
                self.visual_x -= self.width as i32;
            }
            AnchorLeft => {
                self.visual_y -= self.height as i32 / 2;
            }
            AnchorCentered => {
                self.visual_x -= self.width as i32 / 2;
                self.visual_y -= self.height as i32 / 2;
            }
            AnchorRight => {
                self.visual_x -= self.width as i32;
                self.visual_y -= self.height as i32 / 2;
            }
            AnchorBottomLeft => {
                self.visual_y -= self.height as i32;
            }
            AnchorBottom => {
                self.visual_x -= self.width as i32 / 2;
                self.visual_y -= self.height as i32;
            }
            AnchorBottomRight => {
                self.visual_x -= self.width as i32;
                self.visual_y -= self.height as i32;
            }
        }
    }
}

pub struct UIModelC {
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: u32,
}

impl UIModelC {
    pub fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            vertex_count: 0,
        }
    }

    pub fn draw(&self) {
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

    pub fn upload(&mut self, mesh: &[f32]) {
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

pub struct UIContainerC {
    elements: Vec<Box<dyn UIElement>>,
}

impl UIContainerC {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}


pub trait UIElement {
    fn get_transform(&self) -> &UITransformC;

    fn get_mut_transform(&mut self) -> &mut UITransformC;

    fn cursor_update(&mut self, mx: u32, my: u32, down: bool);

    fn is_hovered(&self, mx: u32, my: u32) -> bool {
        let t = self.get_transform();
        
           mx as i32 >= t.visual_x
        && my as i32 >= t.visual_y
        && mx as i32 <= t.visual_x + t.width as i32
        && my as i32 <= t.visual_y + t.height as i32
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
    AnchorTop,
    AnchorTopRight,
    AnchorLeft,
    AnchorCentered,
    AnchorRight,
    AnchorBottomLeft,
    AnchorBottom,
    AnchorBottomRight,
}

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

pub struct UIIconButton {
    transform: UITransformC,
    model: UIModelC,
    pub hovered: bool,
    pub pressed: bool,
}
pub struct UIToggle {
    transform: UITransformC,
}
pub struct UIAnimatedModelDisplay {
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
// Layout
struct UIPanel {
    transform: UITransformC,
}
pub struct UISeperator {
    transform: UITransformC,
}
// Inventory
// a single inventory slot with a textured background
pub struct UIContainerSlot {
    transform: UITransformC,
}
// a grid of basic inventory slots
pub struct UIContainerGrid {
    transform: UITransformC,
}
