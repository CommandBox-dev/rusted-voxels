use glam::Vec2;

use crate::engine::texture::AtlasTile;

pub struct BlockModel {
    solid_blocksites_mask: u32, // culls touching neigbor's block faces
    cullable_faces: Vec<bool>, // list that stores if a face is cullable for every face of this mesh
    cull_own_blocktype: bool, // when set to true, non-solid sides will count as solid for other blocks with this model
    // shape
    pub uvs: [f32; 24],
}

pub enum Shapes {
    CUBE,
    STAIR,  
}

impl BlockModel {

    pub fn new_cube(
        atlas_tile: &AtlasTile
    ) -> Self {

        let face_uvs = [
            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,

            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,

            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,

            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,

            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,

            atlas_tile.x,
            atlas_tile.y,
            atlas_tile.xm,
            atlas_tile.ym,
        ];

        Self {
            solid_blocksites_mask: 0,
            cullable_faces: Vec::new(),
            cull_own_blocktype: false,
            uvs: face_uvs,
        }
    }

    pub fn new_cube_indiv_tex(
        tile_a: &AtlasTile,
        tile_b: &AtlasTile,
        tile_c: &AtlasTile,
        tile_d: &AtlasTile,
        tile_e: &AtlasTile,
        tile_f: &AtlasTile,
    ) -> Self {

        let face_uvs = [
            tile_a.x,
            tile_a.y,
            tile_a.xm,
            tile_a.ym,

            tile_b.x,
            tile_b.y,
            tile_b.xm,
            tile_b.ym,

            tile_c.x,
            tile_c.y,
            tile_c.xm,
            tile_c.ym,

            tile_d.x,
            tile_d.y,
            tile_d.xm,
            tile_d.ym,

            tile_e.x,
            tile_e.y,
            tile_e.xm,
            tile_e.ym,

            tile_f.x,
            tile_f.y,
            tile_f.xm,
            tile_f.ym,
        ];

        Self {
            solid_blocksites_mask: 0,
            cullable_faces: Vec::new(),
            cull_own_blocktype: false,
            uvs: face_uvs,
        }
    }

    pub fn new_cube_sides_top_bottom(
        side_tile: &AtlasTile,
        top_tile: &AtlasTile,
        bottom_tile: &AtlasTile,
    ) -> Self {

        let face_uvs = [
            side_tile.x,
            side_tile.y,
            side_tile.xm,
            side_tile.ym,

            side_tile.x,
            side_tile.y,
            side_tile.xm,
            side_tile.ym,

            top_tile.x,
            top_tile.y,
            top_tile.xm,
            top_tile.ym,

            bottom_tile.x,
            bottom_tile.y,
            bottom_tile.xm,
            bottom_tile.ym,

            side_tile.x,
            side_tile.y,
            side_tile.xm,
            side_tile.ym,

            side_tile.x,
            side_tile.y,
            side_tile.xm,
            side_tile.ym,
        ];

        Self {
            solid_blocksites_mask: 0,
            cullable_faces: Vec::new(),
            cull_own_blocktype: false,
            uvs: face_uvs,
        }
    }
}