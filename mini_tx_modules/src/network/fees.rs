pub enum Priority{
    Low,
    Normal,
    High,
}

fn base_fee() -> u64 {
    1_000
}

pub fn calculate_fee(priority: &Priority) -> u64{
    match priority {
        Priority::Low => base_fee(),
        Priority::Normal => base_fee() * 2,
        Priority::High => base_fee() * 5
    }
}