pub enum Dimension {
    Overworld,
    Flat(Layer),
}
pub struct LayerOptions {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub blocks: Vec<u32>,
    pub height: u32,
}