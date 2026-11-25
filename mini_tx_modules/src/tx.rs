use crate::network::is_amount_valid;
use crate::network::fees::Priority;
use crate::network::fees::calculate_fee;

#[derive(Debug)]
pub struct Transaction{
    pub owner: String,
    tx_amount: u64,
    fee: u64,
}

impl Transaction{
    pub fn new(owner: String, tx_amount: u64, priority: &Priority) -> Option<Transaction>{
        if is_amount_valid(tx_amount) == true {
            let fee = calculate_fee(priority);
            Some(Transaction{owner, tx_amount, fee})
        }else{
            None
        }
    }

    pub fn total_cost(&self) -> u64{
        self.tx_amount + self.fee
    }
}

