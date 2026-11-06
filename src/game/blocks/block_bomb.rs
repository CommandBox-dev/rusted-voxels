use crate::engine::{block::block::Block, world::world::World};
use crate::game::{blocks::blocks::Blocktype};

pub struct Bomb {}

impl Block for Bomb {

    fn on_place(x: i32, y: i32, z: i32, world: &mut World) {
        //world.fill_sphere(x, y, z, 5, vec![Blocktype::Air, Blocktype::Stone]);
        //world.set_block(x, y + 1, z, Blocktype::MazeTiling);
    }

    fn on_break(x: i32, y: i32, z: i32, world: &mut World) {
        //world.fill_sphere(x, y, z, 5, vec![Blocktype::Air, Blocktype::Stone]);
        //world.set_block(x, y - 1, z, Blocktype::MazeTiling);
    }
}