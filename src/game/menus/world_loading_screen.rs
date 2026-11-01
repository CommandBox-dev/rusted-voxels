use rand::distr::slice::Empty;

use crate::engine::{gui::{basic_ui::{UIAnchor::{self, AnchorCentered, AnchorTopLeft}, UIElement, UITransformC}, elements::{label::UILabel, tex_rect::UITexturedRect}}, resources::{resource_manager::ResourceManager, texture::UV}, world::{self, chunk, chunk_manager::ChunkManager, chunk_state::{ChunkState::{Available, InWork}, GenerationStep}}};

pub struct WorldLoadingScreenMenu {
    background: UITexturedRect,
    label: UILabel,
    loading_chunk_canvas: Vec<UITexturedRect>,
}

impl WorldLoadingScreenMenu {
    pub fn new(window_transform: &UITransformC) -> Self {
        let height_aspect = window_transform.width as f32 / window_transform.height as f32;
        let rect_size = 30;

        let mut loading_chunk_canvas: Vec<UITexturedRect> = Vec::new();

        let chunk_canvas_range = 16;

        for x in 0..chunk_canvas_range {
            for z in 0..chunk_canvas_range {

                loading_chunk_canvas.push(
                    UITexturedRect::new(
                        -((chunk_canvas_range / 2) * rect_size) + x * rect_size,
                        -10 - z * rect_size,
                        rect_size as u32,
                        rect_size as u32,
                        UV::new(0.0, 0.0, 1.0, 1.0),
                        UIAnchor::AnchorBottom,
                        window_transform
                    )
                );
            }
        }

        let mut s = Self {
            background: UITexturedRect::new(
                0,
                0,
                window_transform.width as u32,
                (window_transform.height as f32 * height_aspect) as u32,
                UV::new(0.0, 0.0, 10.0, 10.0),
                AnchorTopLeft,
                window_transform,
            ),
            label: UILabel::new(
                0,
                0,
                String::from("label.loading_progress"),
                AnchorCentered,
                window_transform,
            ),
            loading_chunk_canvas,
        };
        s
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        resource_manager.background_tex.bind(0);
        self.background.draw();
        
        resource_manager.font_tex.bind(0);
        self.label.draw();

        resource_manager.ui_tex.bind(0);
        for rect in &self.loading_chunk_canvas {
            rect.draw();
        }
    }

    pub fn update(&mut self, chunk_count: u32, chunk_manager: &ChunkManager) {
        self.label.set_text(
            format!(
                "Loaded Chunks: {}/{}",
                chunk_count,
                world::world::RENDER_AREA - 100
            ),
        );

        let chunk_canvas_range = 16;
        let half_canvas = chunk_canvas_range / 2;
        let rect_size = 50;

        for x in -half_canvas..half_canvas {
            for z in -half_canvas..half_canvas {

                let lx = x + half_canvas;
                let lz = z + half_canvas;

                let mut state = 0;

                if let Some(chunk_state) = chunk_manager.get_chunk_ref(x, z) {
                    match chunk_state {
                        Available(chunk) => {
                            match chunk.generation_step() {
                                GenerationStep::Empty => state = 2,
                                GenerationStep::Terrain => state = 3,
                                GenerationStep::TerrainFeatures => state = 4,
                                GenerationStep::DirectLight => state = 5,
                                GenerationStep::IndirectLight => state = 6,
                                GenerationStep::Complete => state = 7,
                                _ => state = 2,
                            }
                        }

                        InWork(step) => {
                            //state = 1
                            match step {
                                GenerationStep::Empty => state = 2,
                                GenerationStep::Terrain => state = 3,
                                GenerationStep::TerrainFeatures => state = 4,
                                GenerationStep::DirectLight => state = 5,
                                GenerationStep::IndirectLight => state = 6,
                                GenerationStep::Complete => state = 7,
                                _ => state = 2,
                            }
                        }
                    }
                }

                let tile_uv_size = 16;

                let u = 0;
                let mut v = 48;
                let u_max = u + tile_uv_size;

                match state {
                    // None
                    0 => {}
                    // Worked on
                    1 => v += 2 * tile_uv_size,
                    // Empty
                    2 => v += 1 * tile_uv_size,
                    // Terrain
                    3 => v += 3 * tile_uv_size,
                    // Features
                    4 => v += 4 * tile_uv_size,
                    // Direct Light
                    5 => v += 5 * tile_uv_size,
                    // Indirect Light
                    6 => v += 6 * tile_uv_size,
                    // Complete
                    7 => v += 7 * tile_uv_size,

                    _ => {}
                }

                let v_max = v + tile_uv_size;

                let u = u as f32 / 256.0;
                let v = v as f32 / 256.0;
                let u_max = u_max as f32 / 256.0;
                let v_max = v_max as f32 / 256.0;

                // TODO: Should only destroy and reupload rects whoose chunk's state changed
                let rect = &mut self.loading_chunk_canvas[(lx * chunk_canvas_range + lz % chunk_canvas_range) as usize];
                rect.uv = UV::new(u, v, u_max, v_max);
                rect.model.destroy();
                rect.rebuild_model();
            }
        }
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.background.realign(parent_transform);
        self.label.realign(parent_transform);
        for rect in &mut self.loading_chunk_canvas {
            rect.realign(parent_transform);
        }
    }
}