use glam::IVec3;

use crate::{engine::{block::block::Block, world::world::World}, game::blocks::blocks};

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blocktype {
    Air,
    Void,
    Grass,
    Stone,
    Sand,
    OakLog,
    Leaves,
    MazeStone,
    MazeTiling,
    DungeonBrick,
    CompressedStone,
    Rock,
    Bomb,
}

pub fn on_place(block: Blocktype, pos: IVec3, world: &mut World) {
    match block {
        Blocktype::Stone => {crate::game::blocks::block_bomb::Bomb::on_place(pos.x, pos.y, pos.z, world)}
        _ => {}
    }
}

pub fn on_break(block: Blocktype, pos: IVec3, world: &mut World) {
    match block {
        Blocktype::Stone => {crate::game::blocks::block_bomb::Bomb::on_break(pos.x, pos.y, pos.z, world)}
        _ => {}
    }
}


