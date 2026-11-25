mod network;
mod tx;
use crate::tx::Transaction;
use crate::network::fees::Priority;

fn main() {
    let transaction_valid = Transaction::new(String::from("Bob"), 50_000, &Priority::High);
    let transaction_invalid = Transaction::new(String::from("Alice"), 5_000_000, &Priority::High);

    match transaction_valid {
        Some(tx) => {
            println!("Odbiorca: {}", tx.owner);
            println!("Podglad: {:?}", tx);
            println!("Calkowity koszt: {}", tx.total_cost());
        }
        None => {
            println!("Bledna kwota transakcji!");
        }
    }

    match transaction_invalid {
        Some(tx) => {
            println!("Odbiorca: {}", tx.owner);
            println!("Podglad: {:?}", tx);
            println!("Calkowity koszt: {}", tx.total_cost());
        }
        None => {
            println!("Bledna kwota transakcji!");
        }
    }
}
