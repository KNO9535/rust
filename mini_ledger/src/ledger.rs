use std::collections::HashMap;
use crate::tx::Tx;

pub struct Ledger{
    balances: HashMap<String, u64>,
    history: Vec<Tx>,
}

impl Ledger{
    pub fn new() -> Ledger {
        Ledger{
            balances: HashMap::new(), 
            history: Vec::new(),
        }
    }

    pub fn deposit(&mut self, user: &str, amount: u64){
        let saldo = self.balances.entry(user.to_string()).or_insert(0);
        *saldo += amount;
    }

    pub fn balance_of(&self, user: &str) -> u64 {
        self.balances.get(user).copied().unwrap_or(0)
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64, note: &str) -> bool {
        if amount == 0 || self.balance_of(from) < amount {
            false
        }else{
            let sender_balance = self.balances.entry(from.to_string()).or_insert(0);
            *sender_balance -= amount;

            self.deposit(to, amount);

            let tx = Tx::new(from, to, amount, note);
            self.history.push(tx);
            true
        }
    }

    pub fn print_balances(&self){
        for (key, value) in &self.balances {
            println!("{key}: {value}");
        }
    }

    pub fn print_history(&self){
        for tx in &self.history {
            println!("{}", tx.summary());
        }
    }

    pub fn note_word_stats(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();

        for tx in &self.history{
          for word in tx.note.split_whitespace(){
              let count = map.entry(word.to_string()).or_insert(0);
              *count += 1;
          }
      }
      map
    }
}