pub mod fees;

const MAX_TX_AMOUNT: u64 = 1_000_000;

pub fn is_amount_valid(amount: u64) -> bool {
    if amount > 0 && amount <= MAX_TX_AMOUNT { 
        return true;
    }
    return false;
}