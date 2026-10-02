use crate::engine::{self, block_model::BlockModel, texture::{TextureAtlas, load_texture_atlas}, shader::*};

pub struct ResourceManager {
    pub tile_atlas: TextureAtlas,
    pub block_models: Vec<BlockModel>,

    pub base_shader: u32,
}

impl ResourceManager {

    pub fn new() -> Self {

        let mut manual = Self {
            tile_atlas: load_texture_atlas("./res/textures"),
            base_shader: load_shader("./res/shaders/basic.vert", "./res/shaders/basic.frag"),
            block_models: Vec::new()
        };

        let uv_cobblestone = manual.tile_atlas.get_tile(String::from("stone"));
        let uv_wall = manual.tile_atlas.get_tile(String::from("wall2"));
        let uv_crate = manual.tile_atlas.get_tile(String::from("crate"));
        let uv_grass_top = manual.tile_atlas.get_tile(String::from("grass"));
        let uv_grass_side = manual.tile_atlas.get_tile(String::from("grass_side"));
        let bricks = manual.tile_atlas.get_tile(String::from("stone_bricks"));
        let sand = manual.tile_atlas.get_tile(String::from("sand"));
        let labyrinth_wall = manual.tile_atlas.get_tile(String::from("labyrinth_stone"));
        let tiled_floor = manual.tile_atlas.get_tile(String::from("floor_tiling"));

        manual.block_models = vec![
            BlockModel::new_cube(uv_cobblestone),
            BlockModel::new_cube(uv_wall),
            BlockModel::new_cube(uv_crate),
            BlockModel::new_cube_sides_top_bottom(
                uv_grass_side,
                uv_grass_top,
                uv_cobblestone,
            ),
            BlockModel::new_cube(bricks),
            BlockModel::new_cube(tiled_floor),
            BlockModel::new_cube(labyrinth_wall),
        ];

        manual
    }

    pub fn load_textures_and_atlases() {

    }

    pub fn load_block_models(&self) {

    }
}