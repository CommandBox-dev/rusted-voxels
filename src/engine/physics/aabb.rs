use glam::{Vec3};
use crate::engine::world::World;

const EPSILON: f32 = 0.0001;

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub const CUBE: AABB = AABB {
        min: Vec3::ZERO,
        max: Vec3::ONE,
    };
    
    pub const HUMANOID: AABB = AABB {
        min: Vec3::new(-0.3, -1.60, -0.3),
        max: Vec3::new(0.3, 0.20, 0.3),
    };
}

pub struct CollisionResult {
    pub is_on_floor: bool,
    // collided
    // slide collisions
}

pub fn move_and_collide(world: &World, position: &Vec3, velocity: &mut Vec3, hit_box: &AABB) -> CollisionResult {

    let mut result = CollisionResult {
        is_on_floor: false,
    };

    // TODO: calculate axis with the heighest velocity first
    velocity.x = move_axis_x(world, position, velocity.x, hit_box, &mut result);
    velocity.y = move_axis_y(world, position, velocity.y, hit_box, &mut result);
    velocity.z = move_axis_z(world, position, velocity.z, hit_box, &mut result);

    result
}

fn move_axis_x(world: &World, position: &Vec3, v: f32, hit_box: &AABB, result: &mut CollisionResult) -> f32 {
    
    if v == 0.0 {return 0.0;}

    let min_y = position.y + hit_box.min.y;
    let max_y = position.y + hit_box.max.y;
    let min_z = position.z + hit_box.min.z;
    let max_z = position.z + hit_box.max.z;

    let min_block_y = min_y.floor() as i32;
    let max_block_y = (max_y - EPSILON).floor() as i32;
    let min_block_z = min_z.floor() as i32;
    let max_block_z = (max_z - EPSILON).floor() as i32;

    if v > 0.0 {
        let max_x = position.x + hit_box.max.x;
        let new_max_x = max_x + v;
        let start = max_x.ceil() as i32;
        let end = new_max_x.ceil() as i32 - 1;

        for bx in start..=end {
            for y in min_block_y..=max_block_y {
                for z in min_block_z..=max_block_z {
                    if world.get_block(bx, y, z) > 0 {
                        return (bx as f32 - max_x).max(0.0);
                    }
                }
            }
        }
    } else {
        let min_x = position.x + hit_box.min.x;
        let new_min_x = min_x + v;
        let start = min_x.ceil() as i32 - 1;
        let end = new_min_x.ceil() as i32 - 1;

        for bx in (end..=start).rev() {
            for y in min_block_y..=max_block_y {
                for z in min_block_z..=max_block_z {
                    if world.get_block(bx, y, z) > 0 {
                        return ((bx + 1) as f32 - min_x).min(0.0);
                    }
                }
            }
        }
    }
    v
}

fn move_axis_y(world: &World, position: &Vec3, v: f32, hit_box: &AABB, result: &mut CollisionResult) -> f32 {

    if v == 0.0 {return 0.0;}

    let min_x = position.x + hit_box.min.x;
    let max_x = position.x + hit_box.max.x;
    let min_z = position.z + hit_box.min.z;
    let max_z = position.z + hit_box.max.z;

    let min_block_x = min_x.floor() as i32;
    let max_block_x = (max_x - EPSILON).floor() as i32;
    let min_block_z = min_z.floor() as i32;
    let max_block_z = (max_z - EPSILON).floor() as i32;

    if v > 0.0 {
        let max_y = position.y + hit_box.max.y;
        let new_max_y = max_y + v;
        let start = max_y.ceil() as i32;
        let end = new_max_y.ceil() as i32 - 1;

        for x in min_block_x..=max_block_x {
            for by in start..=end {
                for z in min_block_z..=max_block_z {
                    if world.get_block(x, by, z) > 0 {
                        return (by as f32 - max_y).max(0.0);
                    }
                }
            }
        }
    } else {
        let min_y = position.y + hit_box.min.y;
        let new_min_y = min_y + v;
        let start = min_y.ceil() as i32 - 1;
        let end = new_min_y.ceil() as i32 - 1;

        for x in min_block_x..=max_block_x {
            for by in (end..=start).rev() {
                for z in min_block_z..=max_block_z {
                    if world.get_block(x, by, z) > 0 {
                        result.is_on_floor = true;
                        return ((by + 1) as f32 - min_y).min(0.0);
                    }
                }
            }
        }
    }
    v
}

fn move_axis_z(world: &World, position: &Vec3, v: f32, hit_box: &AABB, result: &mut CollisionResult) -> f32 {
    
    if v == 0.0 {return 0.0;}

    let min_x = position.x + hit_box.min.x;
    let max_x = position.x + hit_box.max.x;
    let min_y = position.y + hit_box.min.y;
    let max_y = position.y + hit_box.max.y;

    let min_block_x = min_x.floor() as i32;
    let max_block_x = (max_x - EPSILON).floor() as i32;
    let min_block_y = min_y.floor() as i32;
    let max_block_y = (max_y - EPSILON).floor() as i32;

    if v > 0.0 {
        let max_z = position.z + hit_box.max.z;
        let new_max_z = max_z + v;
        let start = max_z.ceil() as i32;
        let end = new_max_z.ceil() as i32 - 1;

        for x in min_block_x..=max_block_x {
            for y in min_block_y..=max_block_y {
                for bz in start..=end {
                    if world.get_block(x, y, bz) > 0 {
                        return (bz as f32 - max_z).max(0.0);
                    }
                }
            }
        }
    } else {
        let min_z = position.z + hit_box.min.z;
        let new_min_z = min_z + v;
        let start = min_z.ceil() as i32 - 1;
        let end = new_min_z.ceil() as i32 - 1;

        for x in min_block_x..=max_block_x {
            for y in min_block_y..=max_block_y {
                for bz in (end..=start).rev() {
                    if world.get_block(x, y, bz) > 0 {
                        return ((bz + 1) as f32 - min_z).min(0.0);
                    }
                }
            }
        }
    }
    v
}