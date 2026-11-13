use crate::engine::{gui::{basic_ui::{UI_TEX_PIXEL_SIZE, UIAnchor::{self, *}, UIElement, UIModelC, UITransformC}, elements::label::UILabel, layout::panel::get_dyn_textured_rect}, resources::texture::UV};

pub struct UISlider {
    transform: UITransformC,
    model: UIModelC,
    text: UILabel,
    pub hovered: bool,
    pub pressed: bool,
}

impl UIElement for UISlider {
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

impl UISlider {
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
        let tex_pixel_size = UI_TEX_PIXEL_SIZE;
        
        // top left corner
        let tl_corner = UV::new(
            112 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // top right corner
        let tr_corner = UV::new(
            124 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // bottom right corner
        let br_corner = UV::new(
            124 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // bottom left corner
        let bl_corner = UV::new(
            112 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // top side
        let t_side = UV::new(
            117 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // bottom side
        let b_side = UV::new(
            117 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // right side
        let r_side = UV::new(
            112 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );
        // left side
        let l_side = UV::new(
            124 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );
        // center pice
        let center = UV::new(
            117 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );

        let mut verts = get_dyn_textured_rect(
            self.transform.visual_x as f32,
            self.transform.visual_y as f32,
            self.transform.width as f32,
            self.transform.height as f32,
            &tl_corner,
            &tr_corner,
            &br_corner,
            &bl_corner,
            &t_side,
            &b_side,
            &r_side,
            &l_side,
            &center,
        );

        // handle

        // top left corner
        let tl_corner = UV::new(
            112 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // top right corner
        let tr_corner = UV::new(
            124 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // bottom right corner
        let br_corner = UV::new(
            124 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // bottom left corner
        let bl_corner = UV::new(
            112 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // top side
        let t_side = UV::new(
            117 as f32 / tex_pixel_size,
            80 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            84 as f32 / tex_pixel_size,
        );
        // bottom side
        let b_side = UV::new(
            117 as f32 / tex_pixel_size,
            92 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            96 as f32 / tex_pixel_size,
        );
        // right side
        let r_side = UV::new(
            112 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            116 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );
        // left side
        let l_side = UV::new(
            124 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            128 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );
        // center pice
        let center = UV::new(
            117 as f32 / tex_pixel_size,
            85 as f32 / tex_pixel_size,
            123 as f32 / tex_pixel_size,
            91 as f32 / tex_pixel_size,
        );

        verts.extend_from_slice(&get_dyn_textured_rect(
            self.transform.visual_x as f32,
            self.transform.visual_y as f32,
            50.0,
            self.transform.height as f32,
            &tl_corner,
            &tr_corner,
            &br_corner,
            &bl_corner,
            &t_side,
            &b_side,
            &r_side,
            &l_side,
            &center,
        ));

        self.model.upload(&verts);
    }

    pub fn draw_text(&self) {
        self.text.draw();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.text.realign(&self.transform);
        self.create_model();
    }

    pub fn value_changed(&self) {

    }

    pub fn pressed(&self) {

    }

    pub fn down(&self) {

    }

    pub fn released(&self) {

    }
}
