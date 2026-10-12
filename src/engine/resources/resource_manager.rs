use crate::engine::{self, blocks::block_model::BlockModel, resources::{shader::*, texture::*}};

pub struct ResourceManager {
    pub tile_atlas: TextureAtlas,
    pub font_tex: Texture,
    pub ui_tex: Texture,
    
    pub block_models: Vec<BlockModel>,

    pub base_shader: Shader,
    pub ui_shader: Shader,
}

impl ResourceManager {

    pub fn new() -> Self {

        let mut manual = Self {
            tile_atlas: TextureAtlas::create_from_tile_pool_dir("./res/textures/blocks"),
            font_tex: Texture::load_from_file("./res/textures/ui/font.png"),
            ui_tex: Texture::load_from_file("./res/textures/ui/ui.png"),

            block_models: Vec::new(),

            base_shader: Shader::load_from_file("./res/shaders/basic.vert", "./res/shaders/basic.frag"),
            ui_shader: Shader::load_from_file("./res/shaders/basic_ui.vert", "./res/shaders/basic_ui.frag"),
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
        let harsh_dirt = manual.tile_atlas.get_tile(String::from("harsh_dirt"));
        let base = manual.tile_atlas.get_tile(String::from("base"));
        let ore = manual.tile_atlas.get_tile(String::from("ore"));
        let bomb_side = manual.tile_atlas.get_tile(String::from("bomb"));
        let bomb_top = manual.tile_atlas.get_tile(String::from("bomb_top"));
        let face = manual.tile_atlas.get_tile(String::from("face"));
        let pillar = manual.tile_atlas.get_tile(String::from("pillar"));

        manual.block_models = vec![
            BlockModel::new_cube(cobblestone),
            BlockModel::new_cube(wall),
            BlockModel::new_cube(sand),
            BlockModel::new_cube_sides_top_bottom(
                grass_side,
                grass_top,
                cobblestone,
            ),
            //BlockModel::new_cube(crate_tile),
            BlockModel::new_cube_sides_top_bottom(bomb_side, bomb_top, bomb_side),
            BlockModel::new_cube(floor_tiling),
            BlockModel::new_cube(labyrinth_stone),
            BlockModel::new_cube(stone_bricks),
            BlockModel::new_cube(stone_tilee),
            BlockModel::new_cube(stone),
            BlockModel::new_cube(wall),
            BlockModel::new_cube(wall2),
            BlockModel::new_cube(bricks),
            BlockModel::new_cube(ore),
            BlockModel::new_cube(base),
            BlockModel::new_cube_sides_top_bottom(pillar, labyrinth_stone, labyrinth_stone),
            BlockModel::new_cube_sides_top_bottom(face, labyrinth_stone, labyrinth_stone),
        ];

        manual
    }

    pub fn load_textures_and_atlases(&mut self) {
        todo!();
    }

    pub fn load_block_models(&self) {
        todo!();
    }
}

pub enum BlockModelRotations {
    NONE,
    ONE,
    TWO,
    THREE,
}

pub enum BlockModelMirrors {
    NONE,
    MIRRORED,
}

pub fn mirror_face_x() {todo!()}