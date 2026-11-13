use crate::engine::{
    core::{input::Input, window::GameWindow}, gui::basic_ui::{
        self, UIAnchor::{
            AnchorBottom, AnchorBottomLeft, AnchorBottomRight, AnchorCentered, AnchorLeft, AnchorRight, AnchorTop, AnchorTopLeft, AnchorTopRight,
        },
    }, resources::{resource_manager::{self, ResourceManager}, texture::{AtlasTile, UV}},
};

pub const UI_TEX_PIXEL_SIZE: f32 = 256.0;
pub const UI_SCALE: f32 = 100.0;

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

    pub hovered: bool,
    pub mouse_enter: bool,
    pub mouse_exit: bool,
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

            hovered: false,
            mouse_enter: false,
            mouse_exit: false,
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

    fn set(&mut self, other: &Self) {
        self.offset_x = other.offset_x;
        self.offset_y = other.offset_y;
        self.width = other.width;
        self.height = other.height;
        self.anchor = other.anchor;
        self.visual_x = other.visual_x;
        self.visual_y = other.visual_y;
    }

    pub(super) fn is_hovered(&self, mx: u32, my: u32) -> bool {
           mx as i32 >= self.visual_x
        && my as i32 >= self.visual_y
        && mx as i32 <= self.visual_x + self.width as i32
        && my as i32 <= self.visual_y + self.height as i32
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
        self.vertex_count = (mesh.len() / 4) as u32;

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

    /// Returns an array of
    /// vertices that form a rectangle
    /// with the given size, position,
    /// and texture coordinates.
    pub const fn get_rect_verts(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        u: f32,
        v: f32,
        uw: f32,
        vh: f32,
    ) -> [f32; 24] {
        [
            x + w, y + h, uw, vh,
            x + w, y, uw, v,
            x, y, u, v,

            x, y, u, v,
            x, y + h, u, vh,
            x + w, y + h, uw, vh,
        ]
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

    // fn get_model() -> &UIModelC {todo!();}

    fn update(&mut self, input: &Input) {
        self.update_hover(input);
    }

    fn update_hover(&mut self, input: &Input) {
        let mut transform = self.get_mut_transform();
        let hover = transform.is_hovered(
            input.mouse_x,
            input.mouse_y
        );

        transform.mouse_enter = false;
        transform.mouse_exit = false;

        if hover {
            if !transform.hovered {
                transform.mouse_enter = true;
            }
        } else {
            if transform.hovered {
                transform.mouse_exit = true;
            }
        }
        transform.hovered = hover;
    }

    fn draw(&self) {}

    fn hovered(&self) -> bool {
        self.get_transform().hovered
    }

    fn in_focus(&self) -> bool {
        todo!()
    }

    fn mouse_entered(&self) -> bool {
        self.get_transform().mouse_enter
    }

    fn mouse_exited(&self) -> bool {
        self.get_transform().mouse_exit   
    }

    fn focus_entered(&self) {
        todo!()
    }

    fn focus_exited(&self) {
        todo!()
    }
}

pub trait UIContainer {
    fn get_elements(&self) -> &UIContainerC;

    fn get_mut_elements(&mut self) -> &mut UIContainerC;

    /*fn update_container_items(&mut self, input: &Input) {
        let elements = &self.get_elements().elements;
        for item in elements {
            if item.is_hovered() {
                return;
            }
        }
    }*/
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
pub struct UITextBox {
    transform: UITransformC,
}
// Layout
pub struct UISeperator {
    transform: UITransformC,
}
