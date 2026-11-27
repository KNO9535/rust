use std::collections::HashMap;
use crate::item::Item;

pub struct Warehouse {
    stock: HashMap<String, u32>,
    catalog: Vec<Item>,
}


impl Warehouse {
    pub fn new() -> Warehouse {
        Warehouse {
            stock: HashMap::new(),
            catalog: Vec::new(),
        }
    }

    pub fn restock(&mut self, name: &str, quantity: u32){
        let count = self.stock.entry(name.to_string()).or_insert(0);
        *count += quantity;
    }

    pub fn add_item(&mut self, item: Item, quantity: u32){
        self.restock(&item.name, quantity);
        self.catalog.push(item);
    }

    pub fn quantity_of(&self, name: &str) -> u32 {
        self.stock.get(name).copied().unwrap_or(0)
    }

    pub fn sell(&mut self, name: &str, amount: u32) -> bool {
        if amount == 0 || self.quantity_of(name) < amount {
            false
        }else {
            let count = self.stock.entry(name.to_string()).or_insert(0);
            *count -= amount;
            true
        }
    }

    pub fn print_catalog(&self){
        for i in &self.catalog { 
            println!("{}: {}", i.describe(), self.quantity_of(&i.name));
        }
    }

    pub fn total_stock(&self) -> u32 {
        let mut total = 0;
        for (_, value) in &self.stock {
            total += *value
        }

        total
    }
}