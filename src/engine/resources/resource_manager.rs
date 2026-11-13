use glam::IVec3;
use kira::sound::static_sound::StaticSoundData;

use crate::engine::{
    self, block::block_model::{BlockModel, Cube}, rendering::{mesh::BasicMesh, voxel_mesher::voxelize_atlas}, resources::{shader::*, texture::*},
};

pub struct ResourceManager {
    pub tile_atlas: TextureAtlas,
    pub item_atlas: TextureAtlas,
    pub font_tex: Texture,
    pub ui_tex: Texture,
    pub background_tex: Texture,
    pub background_bricks_tex: Texture,
    pub crosshair_tex: Texture,

    pub block_models: Vec<BlockModel>,
    pub block_meshes: Vec<BasicMesh>,
    pub item_meshes: Vec<BasicMesh>,

    pub base_shader: Shader,
    pub ui_shader: Shader,
    pub chunk_shader: Shader,
    pub wireframe_shader: Shader,

    pub place_sound: StaticSoundData,
    pub break_sound: StaticSoundData,
    pub white_grass_music: Option<StaticSoundData>,
    //pub forest: StaticSoundData,
    //pub wind: StaticSoundData,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut manual = Self {
            tile_atlas: TextureAtlas::from_tile_pool_dir("./res/textures/blocks", false),
            item_atlas: TextureAtlas::from_tile_pool_dir("./res/textures/items", true),
            font_tex: Texture::from_file("./res/textures/ui/font_shaded.png"),
            ui_tex: Texture::from_file("./res/textures/ui/ui.png"),
            background_tex: Texture::from_file("./res/textures/blocks/labyrinth_stone.png"),
            background_bricks_tex: Texture::from_file("./res/textures/blocks/bricks.png"),
            crosshair_tex: Texture::from_file("./res/textures/blocks/crosshair.png"),

            block_models: Vec::new(),
            block_meshes: Vec::new(),
            item_meshes: Vec::new(),

            base_shader: Shader::load_from_file(
                "./res/shaders/basic.vert",
                "./res/shaders/basic.frag",
            ),
            ui_shader: Shader::load_from_file(
                "./res/shaders/basic_ui.vert",
                "./res/shaders/basic_ui.frag",
            ),
            chunk_shader: Shader::load_from_file(
                "./res/shaders/chunk.vert",
                "./res/shaders/chunk.frag",
            ),

            wireframe_shader: Shader::load_from_file(
                "./res/shaders/wireframe.vert",
                "./res/shaders/wireframe.frag",
            ),

            place_sound: StaticSoundData::from_file("./res/audio/sfx/block/place/sand.mp3")
                .expect("Failed to load sound from file"),
            break_sound: StaticSoundData::from_file("./res/audio/sfx/block/place/sand.mp3")
                .expect("Failed to load sound from file"),
            white_grass_music: None, //StaticSoundData::from_file("./res/audio/voxel_white_grass.mp3")
                //.expect("Failed to load sound from file"),
            //forest: StaticSoundData::from_file("./res/audio/ambient/forest.mp3")
            //    .expect("Failed to load sound from file"),
            //wind: StaticSoundData::from_file("./res/audio/ambient/wind.mp3")
            //    .expect("Failed to load sound from file"),
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
        let leaves = manual.tile_atlas.get_tile(String::from("leaves"));
        let log = manual.tile_atlas.get_tile(String::from("bark"));
        let snow = manual.tile_atlas.get_tile(String::from("snow"));
        let forest_grass = manual.tile_atlas.get_tile(String::from("forest_grass"));
        let bush = manual.tile_atlas.get_tile(String::from("bush"));
        let apple_leaves = manual.tile_atlas.get_tile(String::from("leaves_apple"));
        let rock = manual.tile_atlas.get_tile(String::from("cracked_stone_floor"));
        let compressed = manual.tile_atlas.get_tile(String::from("compressed_stone"));
        let top = manual.tile_atlas.get_tile(String::from("grid"));
        let planks = manual.tile_atlas.get_tile(String::from("planks"));

        manual.block_models = vec![
            BlockModel::new_empty(),
            BlockModel::new_block(planks),
            BlockModel::new_block_sides_top_bottom(grass_side, grass_top, cobblestone),
            BlockModel::new_block(cobblestone),
            BlockModel::new_block(cobblestone),
            BlockModel::new_cube_mesh(
                cobblestone,
                &[
                    Cube::new(IVec3::new(0, 0, 0), IVec3::new(16, 8, 16)),
                    Cube::new(IVec3::new(0, 8, 8), IVec3::new(16, 16, 16)),
                ]
            ),
            BlockModel::new_cube_mesh(
                log,
                &[
                    Cube::new(IVec3::new(0, 0, 0), IVec3::new(2, 14, 2)),
                    Cube::new(IVec3::new(14, 0, 0), IVec3::new(16, 14, 2)),
                    Cube::new(IVec3::new(0, 0, 14), IVec3::new(2, 14, 16)),
                    Cube::new(IVec3::new(14, 0, 14), IVec3::new(16, 14, 16)),

                    Cube::new(IVec3::new(0, 14, 0), IVec3::new(16, 16, 16)),
                ]
            ),
            BlockModel::new_block(sand),
            BlockModel::new_block(log),
            BlockModel::new_block(leaves),
            BlockModel::new_block(labyrinth_stone),
            BlockModel::new_block(floor_tiling),
            BlockModel::new_block(stone_bricks),
            BlockModel::new_block(compressed),
            BlockModel::new_block(rock),
            
            BlockModel::new_block(wall),
            //BlockModel::new_block(crate_tile),
            BlockModel::new_block_sides_top_bottom(bomb_side, bomb_top, bomb_side),
            BlockModel::new_block(stone_tilee),
            BlockModel::new_block(stone),
            BlockModel::new_block(wall),
            BlockModel::new_block(wall2),
            BlockModel::new_block(bricks),
            BlockModel::new_block(base),
            //BlockModel::new_block_sides_top_bottom(pillar, labyrinth_stone, labyrinth_stone),
            //BlockModel::new_block_sides_top_bottom(face, labyrinth_stone, labyrinth_stone),
            BlockModel::new_block(snow),
            BlockModel::new_block(forest_grass),
            BlockModel::new_block(bush),
            BlockModel::new_block(apple_leaves),
        ];

        for model in &manual.block_models {
            manual.block_meshes.push(BasicMesh::from_block_model(model));
        }

        manual.item_meshes = voxelize_atlas(&manual.item_atlas);

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

pub fn mirror_face_x() {
    todo!()
}
