use mini_ledger::ledger::Ledger;

fn main() {
    let mut ledger = Ledger::new();

    ledger.deposit("Alice", 1000);
    ledger.deposit("Bob", 200);

    ledger.transfer("Alice", "Bob", 400, "Elo chuju 1");
    ledger.transfer("Alice", "Bob", 100, "Elo chuju 2");

    let ok = ledger.transfer("Alice", "Bob", 1000, "Za duzo");
    println!("Udany? {ok}");

    let charlie = ledger.balance_of("Charlie");
    println!("Saldo Charlie: {charlie}");

    ledger.print_balances();
    ledger.print_history();

    let stats = ledger.note_word_stats();
    println!("Slowa: {stats:?}");
    
}
