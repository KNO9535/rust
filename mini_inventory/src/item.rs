#[derive(Debug)]
pub enum Category{
    Electronics,
    Food,
    Clothing,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: Category,
    pub price: u64
}

impl Item {
    pub fn new(name: &str, category: Category, price: u64) -> Item {
        Item {
            name: name.to_string(),
            category: category,
            price: price,
        }
    }

    pub fn describe(&self) -> String {
        let cat = match &self.category {
            Category::Electronics => "Elektronika",
            Category::Food => "Jedzenie",
            Category::Clothing => "Ubrania",
        };

        format!("{}-{}-{}", self.name, cat, self.price)
    }
}