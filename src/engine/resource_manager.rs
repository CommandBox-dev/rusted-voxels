use crate::engine::{self, block_model::BlockModel, texture::{TextureAtlas, load_texture_atlas}, shader::*};

pub struct ResourceManager {
    pub tile_atlas: TextureAtlas,
    pub block_models: Vec<BlockModel>,

    pub base_shader: u32,
    pub ui_shader: u32,
}

impl ResourceManager {

    pub fn new() -> Self {

        let mut manual = Self {
            tile_atlas: load_texture_atlas("./res/textures/blocks"),
            block_models: Vec::new(),

            base_shader: load_shader("./res/shaders/basic.vert", "./res/shaders/basic.frag"),
            ui_shader: load_shader("./res/shaders/basic_ui.vert", "./res/shaders/basic_ui.frag"),
        };

        let cobblestone = manual.tile_atlas.get_tile(String::from("stone"));
        let crate_tile = manual.tile_atlas.get_tile(String::from("crate"));
        let floor_tiling = manual.tile_atlas.get_tile(String::from("floor_tiling"));
        let grass_side = manual.tile_atlas.get_tile(String::from("grass_side"));
        let grass_top = manual.tile_atlas.get_tile(String::from("grass_top"));
        let labyrinth_stone = manual.tile_atlas.get_tile(String::from("labyrinth_stone"));
        let sand = manual.tile_atlas.get_tile(String::from("sand"));
        let stone_bricks = manual.tile_atlas.get_tile(String::from("stone_bricks"));
        let stone_tilee = manual.tile_atlas.get_tile(String::from("stone_tilee"));
        let stone = manual.tile_atlas.get_tile(String::from("stone"));
        let wall = manual.tile_atlas.get_tile(String::from("wall"));
        let wall2 = manual.tile_atlas.get_tile(String::from("wall2"));
        let bricks = manual.tile_atlas.get_tile(String::from("bricks"));

        manual.block_models = vec![
            BlockModel::new_cube(cobblestone),
            BlockModel::new_cube(wall),
            BlockModel::new_cube(sand),
            BlockModel::new_cube_sides_top_bottom(
                grass_side,
                grass_top,
                cobblestone,
            ),
            BlockModel::new_cube(crate_tile),
            BlockModel::new_cube(floor_tiling),
            BlockModel::new_cube(labyrinth_stone),
            BlockModel::new_cube(stone_bricks),
            BlockModel::new_cube(stone_tilee),
            BlockModel::new_cube(stone),
            BlockModel::new_cube(wall),
            BlockModel::new_cube(wall2),
            BlockModel::new_cube(bricks),
        ];

        println!("base_shader: {}, ui_shader: {}", manual.base_shader, manual.ui_shader);

        manual
    }

    pub fn load_textures_and_atlases() {

    }

    pub fn load_block_models(&self) {

    }
}