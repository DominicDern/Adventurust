use crate::item::Item;

#[derive(Default, PartialEq, Debug)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl Inventory {
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, name: &str) {
        let mut index = 0;
        for item in &self.items {
            if item.name() == name {
                self.items.remove(index);
                break;
            } else {
                index += 1;
            }
        }
    }

    pub fn weight(&self) -> u32 {
        let mut weight: u32 = 0;
        for item in &self.items {
            weight += *item.weight() as u32;
        }
        weight
    }
}
