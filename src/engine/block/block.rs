use crate::engine::world::world::World;

pub trait Block {

    fn on_place(x: i32, y: i32, z: i32, world: &mut World) {} // , Option<Entity>

    fn on_break(x: i32, y: i32, z: i32, world: &mut World) {}

    fn on_interaction() {}

    fn on_action() {}

    fn on_step() {}
    
    fn on_tick() {} 

    fn on_random_tick() {}

    fn on_block_update(x: i32, y: i32, z: i32, world: &mut World) {}

    fn is_ticked() -> bool { false }

    fn is_fluid() -> bool { false }

    fn is_solid() -> bool { true }

    fn is_interactable() -> bool { false }

    fn is_container() -> bool { false }
    
    fn is_climbable() -> bool { false }

    fn get_hardness() -> i32 { 10 }

    fn get_brightness() -> u32 { 0 }

    fn get_friction() -> f32 { 1.0 }

    fn get_bounciness() -> f32 { 0.0 }
}

pub struct BlockDefinition {
    //pub name: String,
    //pub 
    //pub illuminace: u32,
}
