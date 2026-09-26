use glam::{Vec3, IVec3};

use crate::engine::{chunk::{self, Chunk}, world::World};

pub fn cast_ray(origin: Vec3, dir: Vec3, length: f32, world: &mut World) -> RayResult {
    // direction needs to be normalized
    // chunk is passed as an parameter for now
    let mut result: RayResult = RayResult::new();

    let dx = dir.x;
    let dy = dir.y;
    let dz = dir.z;

    let mut x: i32 = origin.x.floor() as i32;
    let mut y: i32 = origin.y.floor() as i32;
    let mut z: i32 = origin.z.floor() as i32;

    let step_x: i32 = if dx > 0.0 {1} else {-1};
    let step_y: i32 = if dy > 0.0 {1} else {-1};
    let step_z: i32 = if dz > 0.0 {1} else {-1};

    let t_delta_x: f32 = if dx == 0.0 {f32::MAX} else {(1.0 / dx).abs()};
    let t_delta_y: f32 = if dy == 0.0 {f32::MAX} else {(1.0 / dy).abs()};
    let t_delta_z: f32 = if dz == 0.0 {f32::MAX} else {(1.0 / dz).abs()};

    let next_boundary_x: f32 = if step_x > 0 {x as f32 + 1.0} else {x as f32};
    let next_boundary_y: f32 = if step_y > 0 {y as f32 + 1.0} else {y as f32};
    let next_boundary_z: f32 = if step_z > 0 {z as f32 + 1.0} else {z as f32};
    
    let mut t_max_x: f32 = if dx == 0.0 {f32::MAX} else {(next_boundary_x - origin.x) / dx};
    let mut t_max_y: f32 = if dy == 0.0 {f32::MAX} else {(next_boundary_y - origin.y) / dy};
    let mut t_max_z: f32 = if dz == 0.0 {f32::MAX} else {(next_boundary_z - origin.z) / dz};

    if (t_max_x < 0.0) {t_max_x = 0.0;}
    if (t_max_y < 0.0) {t_max_y = 0.0;}
    if (t_max_z < 0.0) {t_max_z = 0.0;}

    let mut distance: f32 = 0.0;
    let mut last_step_axis: i32 = -1;

    while (distance <= length) {

        let block: u32 = world.get_block(x, y, z);

        if (block > 0) {

            result.hit = true;
            result.hit_position.x = x;
            result.hit_position.y = y;
            result.hit_position.z = z;
            result.travel_distance = distance;
            result.hit_block = block;

            // determine hit normal
            if (last_step_axis == 0) {
                result.hit_normal.x = -step_x;
            } else if (last_step_axis == 1) {
                result.hit_normal.y = -step_y;
            } else if (last_step_axis == 2) {
                result.hit_normal.z = -step_z;
            }
            return result;
        }
        // step to next voxel
        if (t_max_x < t_max_y && t_max_x < t_max_z) {
            x += step_x;
            distance = t_max_x;
            t_max_x += t_delta_x;
            last_step_axis = 0;
        }
        else if (t_max_y < t_max_z) {
            y += step_y;
            distance = t_max_y;
            t_max_y += t_delta_y;
            last_step_axis = 1;
        }
        else {
            z += step_z;
            distance = t_max_z;
            t_max_z += t_delta_z;
            last_step_axis = 2;
        }
    }
    // ray hit nothing
    result
}

pub struct RayResult {
    pub hit: bool,
    pub hit_position: IVec3,
    pub hit_normal: IVec3,
    pub hit_block: u32,
    pub travel_distance: f32
}

impl RayResult {
    fn new() -> RayResult {
        Self {
            hit: false,
            hit_position: IVec3::new(0, 0, 0),
            hit_normal: IVec3::new(0, 0, 0),
            hit_block: 0,
            travel_distance: 0.0
        }
    }
}
