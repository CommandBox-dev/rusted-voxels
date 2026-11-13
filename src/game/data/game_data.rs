use crate::{engine::item::item::{ItemCategory, ItemData, ItemDefinition, ItemType}, game::blocks::blocks::Blocktype};

pub struct GameData {
    pub items: ItemRegistry,
    // blocks: ItemRegistry,
    // entities: ItemRegistry,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            items: ItemRegistry::new(),
        }
    }
}

pub struct ItemRegistry {
    pub entries: Vec<ItemDefinition>
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn register_block(&mut self, blocktype: Blocktype) {
        self.entries.push(
            ItemDefinition {
                name: String::from("block name"),
                description: String::from("..."),
                max_stack: 99,
                item_type: ItemType::Block,
                category:  ItemCategory::BuildingBlocks,
                item_data: ItemData::BlockData(blocktype),
            }
        );
    }
}