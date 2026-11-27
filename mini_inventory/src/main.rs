use mini_inventory::item::Category;
use mini_inventory::item::Item;
use mini_inventory::warehouse::Warehouse;

fn main() {
    let mut new_warehouse = Warehouse::new();

    new_warehouse.add_item(Item::new("T-shirt", Category::Clothing, 50), 20);
    new_warehouse.add_item(Item::new("Banana", Category::Food, 10), 3);
    new_warehouse.add_item(Item::new("PC", Category::Electronics, 10), 1);

    new_warehouse.restock("Banana", 10);

    println!("sprzedaz pc: {}", new_warehouse.sell("PC", 1));
    println!("sprzedaz tshirtow: {}",new_warehouse.sell("T-shirt", 67));

    println!("nieistniejacy: {}", new_warehouse.quantity_of("Rower"));

    new_warehouse.print_catalog();
    println!("{}", new_warehouse.total_stock());
}

