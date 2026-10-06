use crate::game::blocks::*;

enum Blocks {
    //GrassBlock(Gra)
}

pub struct Block {
    name: String,
}

pub trait BlockScript {
    fn on_place();
    fn on_break();
    fn on_interaction() -> bool {false} // Returns false if no interaction happened
    fn on_tick(); // If ticked block
    fn on_random_tick();
    fn on_block_update(); // If neighbor block gets updated or in special cases if entity is standing on top of the block and sends update
}