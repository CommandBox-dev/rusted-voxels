use crate::engine::{gui::basic_ui::{UI_SCALE, UI_TEX_PIXEL_SIZE, UIAnchor, UIElement, UIModelC, UITransformC}, resources::texture::UV};

pub struct UIPanel {
    transform: UITransformC,
    model: UIModelC
}

impl UIElement for UIPanel {
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

impl UIPanel {
    pub fn new(x: i32, y: i32, w: u32, h: u32, anchor: UIAnchor, parent_transform: &UITransformC) -> Self {

        let mut transform = UITransformC::new(x, y, w, h, anchor);
        transform.update_transform(parent_transform);

        let mut panel = Self {
            transform,
            model: UIModelC::new(),
        };
        panel.build_model();
        panel
    }

    fn build_model(&mut self) {
        let tex_pixel_size = UI_TEX_PIXEL_SIZE;

        // the panel is split into nine parts in
        // order to keep pixel scale consistent

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

        let verts = get_dyn_textured_rect(
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

        self.model.upload(&verts);
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.transform.update_transform(parent_transform);
        self.build_model();
    }
}

/// A rect that is created from
/// multiple pices to keep the pixel
/// scale consistent instead of
/// distorted when scaled

// TODO: Make the rect more dynamic,
// expand the texture instead of streching,
// could be archived by expanding the texture by
// adding extra sub rects that keep the
// size instead of scaling the full rect.
pub fn get_dyn_textured_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tl_corner: &UV,
    tr_corner: &UV,
    br_corner: &UV,
    bl_corner: &UV,
    t_side: &UV,
    b_side: &UV,
    r_side: &UV,
    l_side: &UV,
    center: &UV,
) -> Vec<f32> {
    // The rect is split into nine parts in
    // order to keep pixel scale consistent.
    // Does not support detailed textures yet.
    
    let mut verts = Vec::new();

    let pixel_scale = UI_SCALE / 16.0;
    let part_w = 4.0 * pixel_scale;
    let part_w_x2 = part_w * 2.0;

    // top left corner
    let w = part_w;
    let h = part_w;
    let px = x;
    let py = y;

    add_rect(&mut verts, px, py, w, h, tl_corner.x, tl_corner.y, tl_corner.x_max, tl_corner.y_max);

    // top right corner
    let w = part_w;
    let h = part_w;
    let px = x + width - w;
    let py = y;

    add_rect(&mut verts, px, py, w, h, tr_corner.x, tr_corner.y, tr_corner.x_max, tr_corner.y_max);

    // bottom right corner
    let w = part_w;
    let h = part_w;
    let px = x + width - w;
    let py = y + height - h;

    add_rect(&mut verts, px, py, w, h, br_corner.x, br_corner.y, br_corner.x_max, br_corner.y_max);

    // bottom left corner
    let w = part_w;
    let h = part_w;
    let px = x;
    let py = y + height - h;

    add_rect(&mut verts, px, py, w, h, bl_corner.x, bl_corner.y, bl_corner.x_max, bl_corner.y_max);

    // top side
    let w = width - part_w_x2;
    let h = part_w;
    let px = x + part_w;
    let py = y;

    add_rect(&mut verts, px, py, w, h, t_side.x, t_side.y, t_side.x_max, t_side.y_max);

    // bottom side
    let w = width - part_w_x2;
    let h = part_w;
    let px = x + part_w;
    let py = y + height - part_w;

    add_rect(&mut verts, px, py, w, h, b_side.x, b_side.y, b_side.x_max, b_side.y_max);

    // right side
    let w = part_w;
    let h = height - part_w_x2;
    let px = x;
    let py = y + part_w;

    add_rect(&mut verts, px, py, w, h, r_side.x, r_side.y, r_side.x_max, r_side.y_max);

    // left side
    let w = part_w;
    let h = height - part_w_x2;
    let px = x + width - part_w;
    let py = y + part_w;

    add_rect(&mut verts, px, py, w, h, l_side.x, l_side.y, l_side.x_max, l_side.y_max);

    // center pice
    let w = width - part_w_x2;
    let h = height - part_w_x2;
    let px = x + part_w;
    let py = y + part_w;

    add_rect(&mut verts, px, py, w, h, center.x, center.y, center.x_max, center.y_max);

    verts
}

pub fn add_rect(
    verts: &mut Vec<f32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    u: f32,
    v: f32,
    uw: f32,
    vh: f32,
) {
    verts.extend_from_slice(
        &UIModelC::get_rect_verts(x, y, w, h, u, v, uw, vh)
    );
}