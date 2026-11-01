use glam::Vec3;

use crate::engine::world::world::World;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Blocktype {
    Air,
    Grass,
    Stone,
    Sand,
    OakLog,
    Leaves,
}

pub fn on_place(block: Blocktype, position: Vec3, world: &mut World) {
    match block {
        _ => {}
    }
}

pub fn on_break(block: Blocktype, position: Vec3, world: &mut World) {
    match block {
        _ => {}
    }
}

pub fn on_interaction(block: Blocktype) { //, Entity
    match block {
        _ => {}
    }
}

pub fn on_action(block: Blocktype) {
    match block {
        _ => {}
    }
}

pub fn on_tick(block: Blocktype) {
    match block {
        _ => {}
    }
}

pub fn on_random_tick(block: Blocktype) {
    match block {
        _ => {}
    }
}

pub fn on_update(block: Blocktype) {
    match block {
        _ => {}
    }
}


