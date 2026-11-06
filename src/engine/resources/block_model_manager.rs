use crate::engine::block::block_model::BlockModel;

pub struct BlockModelManager {
    pub models: Vec<BlockModel>,
    pub cull_data: Vec<bool>,
}