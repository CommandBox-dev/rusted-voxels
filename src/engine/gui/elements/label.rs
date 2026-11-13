use crate::engine::gui::basic_ui::{UIAnchor, UIElement, UIModelC, UITransformC};


// TODO: Labels should support newlines
pub struct UILabel {
    text: String,
    transform: UITransformC,
    model: UIModelC,
}

impl UIElement for UILabel {
    fn get_transform(&self) -> &UITransformC {
        &self.transform
    }

    fn get_mut_transform(&mut self) -> &mut UITransformC {
        &mut self.transform
    }
    
    fn draw(&self) {
        self.model.draw();
    }
}

impl UILabel {
    pub fn new(x: i32, y: i32, text: String, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {
        let text_width = UILabel::get_label_width(&text);

        let mut label = Self {
            text: text,
            transform: UITransformC::new(x, y, text_width, 25, anchor),
            model: UIModelC::new(),
        };
        label.transform.update_transform(parent_transform);
        label.build_mesh();
        label
    }

    pub fn build_mesh(&mut self) {

        let text = &self.text;

        self.transform.width = UILabel::get_label_width(&self.text);
        self.transform.height = 25.0 as u32;

        let mesh = create_mesh_from_str(
            &self.text,
            self.transform.visual_x as f32,
            self.transform.visual_y as f32,
        );
    
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

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.build_mesh();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.build_mesh();
    }
}

#[must_use]
pub fn create_mesh_from_str(text: &str, x: f32, y: f32) -> Vec<f32> {
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
            let x: f32 = x + total_width;
            let y: f32 = y;

            // the font atlas starts at ascii char '!'
            let l = c as u8 - b'!';
            let col = (l % 18) as f32;
            let row = (l / 18) as f32;

            let u: f32 = col * char_tex_aspect_x;
            let v: f32 = row * char_tex_aspect_y + char_tex_aspect_y - char_tex_offset_y;
            let uw: f32 = u + char_tex_aspect_x;
            let vh: f32 = v + char_width_y;

            let rect = UIModelC::get_rect_verts(x, y, w, h, u, v, uw, vh);

            for f in rect {
                mesh.push(f);
            }
        }
        total_width += UILabel::get_char_width(c) * w;
    }
    mesh
}