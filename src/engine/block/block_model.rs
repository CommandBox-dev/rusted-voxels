use glam::{IVec3, Vec2, Vec3};

use crate::engine::{physic::aabb::AABB, rendering::chunk_mesher::{FACE_BACK, FACE_BOTTOM, FACE_FRONT, FACE_LEFT, FACE_RIGHT, FACE_TOP}, resources::texture::{AtlasTile, UV}};

#[derive(Clone)]
pub struct BlockModel {
    pub shape: BlockShape,

    // Only used by CubeMeshes
    pub occlusion: BlockOcclusion,
    pub cullable_faces: Vec<bool>,
    // Will give transparent blocks the ability
    // to cull (only) their own blocktype
    pub cull_own_blocktype: bool,
    
    pub aabbs: Vec<AABB>,
    // Indicates the number of cubes a CubeModel is
    // made of and the number of faces for a Custom model
    pub parts: u32,
    // Will later be ignored if shape is None or Voxel
    pub uvs: Vec<f32>,
    pub verts: Vec<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockShape {
    Empty,
    Voxel,
    /// A mesh made of one or multiple cubes with
    /// custom size and position, doesn't support rotated faces.
    CubeMesh,
    /// Supports rotation, and separate faces.
    /// doesn't support culling or ambient occlusion.
    Custom,
}

// the chunk mesher will just add every triangle
// and uv to the chunk mesh until there is nothing left
// when the block shape is set to Custom
// faces with any transparent pixel won't count on the occlusion mesh
// if the model has transparent parts, it wont cull any blocks
// unless its own kind when cull own blocktype is active
// Add option if cube has inverted backfaces (just different triangle order)
// Faces inside blocks like the faces of the two cubes a stair
// is made up should have an option to be hidden


impl BlockModel {
    pub fn new_block(atlas_tile: &AtlasTile) -> Self {

        let face_uvs = vec![
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
            atlas_tile.x, atlas_tile.y, atlas_tile.xm, atlas_tile.ym,
        ];

        Self {
            shape: BlockShape::Voxel,
            occlusion: BlockOcclusion::default(),
            cullable_faces: Vec::new(),
            cull_own_blocktype: true,
            aabbs: vec![AABB::new(Vec3::splat(0.0), Vec3::splat(16.0))],
            parts: 0,
            verts: Vec::new(),
            uvs: face_uvs,
        }
    }

    pub fn new_block_indiv_tex(
        tile_a: &AtlasTile,
        tile_b: &AtlasTile,
        tile_c: &AtlasTile,
        tile_d: &AtlasTile,
        tile_e: &AtlasTile,
        tile_f: &AtlasTile,
    ) -> Self {
        let face_uvs = vec![
            tile_a.x, tile_a.y, tile_a.xm, tile_a.ym,
            tile_b.x, tile_b.y, tile_b.xm, tile_b.ym,
            tile_c.x, tile_c.y, tile_c.xm, tile_c.ym,
            tile_d.x, tile_d.y, tile_d.xm, tile_d.ym,
            tile_e.x, tile_e.y, tile_e.xm, tile_e.ym,
            tile_f.x, tile_f.y, tile_f.xm, tile_f.ym,
        ];

        Self {
            shape: BlockShape::Voxel,
            occlusion: BlockOcclusion::default(),
            cullable_faces: Vec::new(),
            cull_own_blocktype: true,
            aabbs: vec![AABB::new(Vec3::splat(0.0), Vec3::splat(16.0))],
            parts: 0,
            verts: Vec::new(),
            uvs: face_uvs,
        }
    }

    pub fn new_block_sides_top_bottom(
        side_tile: &AtlasTile,
        top_tile: &AtlasTile,
        bottom_tile: &AtlasTile,
    ) -> Self {

        let face_uvs = vec![
            side_tile.x, side_tile.y, side_tile.xm, side_tile.ym,
            side_tile.x, side_tile.y, side_tile.xm, side_tile.ym,
            top_tile.x, top_tile.y, top_tile.xm, top_tile.ym,
            bottom_tile.x, bottom_tile.y, bottom_tile.xm, bottom_tile.ym,
            side_tile.x, side_tile.y, side_tile.xm, side_tile.ym,
            side_tile.x, side_tile.y, side_tile.xm, side_tile.ym,
        ];

        Self {
            shape: BlockShape::Voxel,
            occlusion: BlockOcclusion::default(),
            cullable_faces: Vec::new(),
            cull_own_blocktype: true,
            aabbs: vec![AABB::new(Vec3::splat(0.0), Vec3::splat(16.0))],
            parts: 0,
            verts: Vec::new(),
            uvs: face_uvs,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            shape: BlockShape::Empty,
            occlusion: BlockOcclusion::default(),
            cullable_faces: Vec::new(),
            cull_own_blocktype: true,
            aabbs: Vec::new(),
            parts: 0,
            verts: Vec::new(),
            uvs: Vec::new(),
        }
    }

    pub fn new_cube_mesh(atlas_tile: &AtlasTile, cubes: &[Cube]) -> Self {

        let mut model = Self {
            shape: BlockShape::CubeMesh,
            occlusion: BlockOcclusion::default(),
            cullable_faces: Vec::new(),
            cull_own_blocktype: true,
            aabbs: Vec::new(),
            parts: 0,
            verts: Vec::new(),
            uvs: Vec::new(),
        };

        // Self::add_cube(&mut model, atlas_tile, &IVec3::new(6, 0, 6), &IVec3::new(10, 14, 10));
        // Self::add_cube(&mut model, atlas_tile, &IVec3::new(2, 7, 6), &IVec3::new(14, 11, 10));

        //Self::add_cube(&mut model, atlas_tile, &IVec3::new(1, 0, 6), &IVec3::new(15, 14, 10));
        //Self::add_cube(&mut model, atlas_tile, &IVec3::new(4, 12, 6), &IVec3::new(12, 16, 10));

        for cube in cubes {
            Self::add_cube(
                &mut model,
                atlas_tile,
                &cube.min_corner,
                &cube.max_corner,
            );
            model.aabbs.push(
                AABB::new(
                    cube.min_corner.clone().as_vec3(),
                    cube.max_corner.clone().as_vec3(),
                )
            );
        }

        model
    }
    
    fn add_cube(
        model: &mut BlockModel,
        atlas_tile: &AtlasTile,
        min_corner: &IVec3,
        max_corner: &IVec3,
    ) {
        let uvs = &mut model.uvs;
        let verts = &mut model.verts;
        let occlusion = &mut model.occlusion;

        let x = min_corner.x as f32 / 16.0;
        let y = min_corner.y as f32 / 16.0;
        let z = min_corner.z as f32 / 16.0;

        let xm = max_corner.x as f32 / 16.0;
        let ym = max_corner.y as f32 / 16.0;
        let zm = max_corner.z as f32 / 16.0;

        let tiles_per_row = 6.0;

        let ux = x / tiles_per_row;
        let uy = y / tiles_per_row;
        let uz = z / tiles_per_row;

        let uxm = xm / tiles_per_row;
        let uym = ym / tiles_per_row;
        let uzm = zm / tiles_per_row;

        let cube = [
            // right
            xm, y, zm,
            xm, y, z,
            xm, ym, z,

            xm, y, zm,
            xm, ym, z,
            xm, ym, zm,

            // left
            x, y, z,
            x, y, zm,
            x, ym, zm,

            x, y, z,
            x, ym, zm,
            x, ym, z,

            // top
            x, ym, zm,
            xm, ym, zm,
            xm, ym, z,

            x, ym, zm,
            xm, ym, z,
            x, ym, z,

            // bottom
            x, y, z,
            xm, y, z,
            xm, y, zm,

            x, y, z,
            xm, y, zm,
            x, y, zm,

            // front
            x, y, zm,
            xm, y, zm,
            xm, ym, zm,

            x, y, zm,
            xm, ym, zm,
            x, ym, zm,

            // back
            xm, y, z,
            x, y, z,
            x, ym, z,

            xm, y, z,
            x, ym, z,
            xm, ym, z
        ];
        
        let tex = [
            atlas_tile.x + uz, atlas_tile.y + uy, atlas_tile.x + uzm, atlas_tile.y + uym,
            atlas_tile.x + uz, atlas_tile.y + uy, atlas_tile.x + uzm, atlas_tile.y + uym,
            atlas_tile.x + ux, atlas_tile.y + uz, atlas_tile.x + uxm, atlas_tile.y + uzm,
            atlas_tile.x + ux, atlas_tile.y + uz, atlas_tile.x + uxm, atlas_tile.y + uzm,
            atlas_tile.x + ux, atlas_tile.y + uy, atlas_tile.x + uxm, atlas_tile.y + uym,
            atlas_tile.x + ux, atlas_tile.y + uy, atlas_tile.x + uxm, atlas_tile.y + uym,
        ];

        uvs.extend_from_slice(&tex);
        verts.extend_from_slice(&cube);

        // skip if model is transparent
        // right face
        if max_corner.x == 16 {
            let mut face = &mut occlusion.faces[FACE_RIGHT];

            face.set_rect(
                min_corner.z as u32,
                min_corner.y as u32,
                (max_corner.z - min_corner.z) as u32,
                (max_corner.y - min_corner.y) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }
        // left face
        if min_corner.x == 0 {
            let mut face = &mut occlusion.faces[FACE_LEFT];

            face.set_rect(
                min_corner.z as u32,
                min_corner.y as u32,
                (max_corner.z - min_corner.z) as u32,
                (max_corner.y - min_corner.y) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }
        // top face
        if max_corner.y == 16 {
            let mut face = &mut occlusion.faces[FACE_TOP];

            face.set_rect(
                min_corner.x as u32,
                min_corner.z as u32,
                (max_corner.x - min_corner.x) as u32,
                (max_corner.z - min_corner.z) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }
        // bottom face
        if min_corner.y == 0 {
            let mut face = &mut occlusion.faces[FACE_BOTTOM];

            face.set_rect(
                min_corner.x as u32,
                min_corner.z as u32,
                (max_corner.x - min_corner.x) as u32,
                (max_corner.z - min_corner.z) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }
        // front face
        if max_corner.z == 16 {
            let mut face = &mut occlusion.faces[FACE_FRONT];

            face.set_rect(
                min_corner.x as u32,
                min_corner.y as u32,
                (max_corner.x - min_corner.x) as u32,
                (max_corner.y - min_corner.y) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }
        // back face
        if min_corner.z == 0 {
            let mut face = &mut occlusion.faces[FACE_BACK];

            face.set_rect(
                min_corner.x as u32,
                min_corner.y as u32,
                (max_corner.x - min_corner.x) as u32,
                (max_corner.y - min_corner.y) as u32
            );

            model.cullable_faces.push(true);
        } else {
            model.cullable_faces.push(false);
        }

        model.parts += 1;
    }

}

// only culls the whole face or nothing,
// if only parts of neighbor are culled, nothing is.

// model modifying block state
// uv modyfing block state

// random model / texture per block

// orientate: top down: (y 0, 90, 180, 270) / log: (0, 90x, 90z)
// mirror y

/*
    uv_right: &UV,
    uv_left: &UV,
    uv_top: &UV,
    uv_bottom: &UV,
    uv_front: &UV,
    uv_back: &UV, 
*/

#[derive(Clone, Copy, Default)]
pub struct BlockOcclusion {
    pub faces: [FaceMask; 6],
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct FaceMask {
    pub bits: [u64; 4],
}

impl FaceMask {
    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32) {
        let index = y * 16 + x;
        self.bits[(index / 64) as usize] |= 1_u64 << (index % 64);
    }

    #[inline(always)]
    pub fn get(&self, x: u32, y: u32) -> bool {
        let index = y * 16 + x;
        (self.bits[(index / 64) as usize] & (1_u64 << (index % 64))) != 0
    }

    #[inline(always)]
    pub fn is_fully_occluded(&self, neighbor: &Self) -> bool {
        (self.bits[0] & !neighbor.bits[0]) == 0
        && (self.bits[1] & !neighbor.bits[1]) == 0
        && (self.bits[2] & !neighbor.bits[2]) == 0
        && (self.bits[3] & !neighbor.bits[3]) == 0
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bits == [0; 4]
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.bits == [u64::MAX; 4]
    }

    #[inline(always)]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            bits: [
                self.bits[0] | other.bits[0],
                self.bits[1] | other.bits[1],
                self.bits[2] | other.bits[2],
                self.bits[3] | other.bits[3],
            ],
        }
    }

    #[inline]
    pub fn set_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        
        assert!(x + width <= 16);
        assert!(y + height <= 16);
        
        for row in y..(y + height) {
            let index = row * 16 + x;
            let word = (index / 64) as usize;
            let offset = index % 64;
            
            let mask = ((1u64 << width) - 1) << offset;
            
            self.bits[word] |= mask;
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Cube {
    min_corner: IVec3,
    max_corner: IVec3,
}

impl Cube {
    pub fn new(min_corner: IVec3, max_corner: IVec3) -> Self {
        Self {
            min_corner,
            max_corner,
        }
    }
}