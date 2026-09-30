use glam::Vec2;

pub struct BlockModel {
    solid_blocksites_mask: u32, // culls touching neigbor's block faces
    cullable_faces: Vec<bool>, // list that stores if a face is cullable for every face of this mesh
    cull_own_blocktype: bool, // when set to true, non-solid sides will count as solid for other blocks with this model

    // shape

    pub uvs: [f32; 24],
}

impl BlockModel {

    pub fn new_cube(
        uv_min_x: f32,
        uv_min_y: f32,
        uv_max_x: f32,
        uv_max_y: f32,
    ) -> Self {

        let uvs = [
            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,

            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,

            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,

            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,

            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,

            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,
        ];

        Self {
            solid_blocksites_mask: 0,
            cullable_faces: Vec::new(),
            cull_own_blocktype: false,
            uvs,
        }
    }

    pub fn new_cube_indiv_face_uvs(face_uvs: [f32; 24]) -> Self {

        Self {
            solid_blocksites_mask: 0,
            cullable_faces: Vec::new(),
            cull_own_blocktype: false,
            uvs: face_uvs,
        }
        
    }
}