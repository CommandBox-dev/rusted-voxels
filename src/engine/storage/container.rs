use crate::engine::item::item::ItemInstance;


#[derive(Debug, Clone)]
pub struct Container {
    pub slots: Vec<Option<ItemInstance>>,
}

impl Container {
    pub fn new(capacity: u32) -> Self {
        Self {
            slots: vec![None; capacity as usize],
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn clear_item(&mut self) {
        todo!();
    }

    #[inline(always)]
    pub fn set_item(&mut self, slot: u32, item: Option<ItemInstance>) {
        self.slots[slot as usize] = item;
    }

    pub fn add_item(&mut self, item: &ItemInstance) -> bool {

        let mut first_empty: Option<usize> = None;

        for i in 0..self.slots.len() {
            let mut slot = &mut self.slots[i];

            if let Some(stack) = slot {
                if stack.item_id != item.item_id {
                    continue;
                }
                stack.quantity += item.quantity;
                return true;
            } else if first_empty.is_none() {
                first_empty = Some(i);
            }
        }

        if let Some(index) = first_empty {
            self.slots[index] = Some(item.clone());
        }

        false
    }

    pub fn remove_item(&mut self) {
        todo!();
    }

    pub fn count_item(&mut self) {
        todo!();
    }

    pub fn take_item(&mut self, slot: u32) -> Option<ItemInstance> {
        self.slots[slot as usize].take()
    }
}