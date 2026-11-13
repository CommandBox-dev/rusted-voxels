use crate::game::blocks::blocks::Blocktype;

pub struct ItemDefinition {
    pub name: String,
    pub description: String,
    pub max_stack: u32,
    pub item_type: ItemType,
    pub category: ItemCategory,
    pub item_data: ItemData,
    //tags: Vec<Tag>,
}

impl ItemDefinition {
    pub fn is_block() {
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Block,
    Tool,
    Weapon,
    Consumeable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    Combat,
    Tools,
    Potions,
    Food,
    ContraptionsAndTraps,
    NatureBlocks,
    BuildingBlocks,
    /// Hidden from item menu
    Hidden,
    /// Completely inaccessable
    NonExistent,
}

// Items should be able to be weapon and tool at the same time...
pub enum ItemData {
    WeaponData, // Damage, Speed...
    ToolData, // Damage, Efficiency, Mining Strenght...
    BlockData(Blocktype),
}

#[derive(Debug, Clone)]
pub struct ItemInstance {
    pub item_id: u32,
    pub quantity: u32,
    pub instance_data: ItemInstanceData,
}

impl ItemInstance {
    pub fn new(
        item_id: u32,
        quantity: u32,
        instance_data: ItemInstanceData
    ) -> Self {
        Self {
            item_id,
            quantity,
            instance_data,
        }
    }
}

/// Used by item stacks and dropped item entities,
/// stores durability for tools, items for containers
/// like shulker chests in minecraft
#[derive(Debug, Clone)]
pub enum ItemInstanceData {
    Placeholder
}

// Scripts should define if a block or armor pice is equippable, usable...