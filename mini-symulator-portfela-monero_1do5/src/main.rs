use std::io;

#[derive(Debug)]
struct Wallet {
    owner: String,
    balance: u64,
    tx_count: usize,
    last_txs: [u64; 5],
}

#[derive(Debug)]
struct TransferReceipt {
    amount: u64,
    fee: u64,
    success: bool,
}

const PICONERO_PER_XMR: u64 = 1_000_000_000_000;
const TX_FEE: u64 = 10_000;

impl Wallet {
    fn new(owner: String, initial_balance: u64) -> Wallet {
        Wallet {
            owner,
            balance: initial_balance,
            tx_count: 0,
            last_txs: [0; 5],
        }
    }

    fn balance_parts(&self) -> (u64, u64) {
        let whole: u64 = self.balance / PICONERO_PER_XMR;
        let fractional: u64 = self.balance % PICONERO_PER_XMR;
        (whole, fractional)
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn transfer_to(&mut self, recipient: &mut Wallet, amount: u64) -> TransferReceipt {
        if amount > 0 && self.balance >= TX_FEE && amount <= self.balance - TX_FEE {
            self.balance -= amount + TX_FEE;
            self.last_txs[self.tx_count % 5] = amount;
            self.tx_count += 1;
            recipient.balance += amount;
            TransferReceipt {
                amount,
                fee: TX_FEE,
                success: true,
            }
        } else {
            TransferReceipt {
                amount,
                fee: 0,
                success: false,
            }
        }
    }

    fn recent_transfers(&self) -> &[u64] {
        &self.last_txs[0..self.tx_count.min(5)]
    }

    fn burn(self) {
        println!(
            "Wallet of {} burned. Final balance: {} piconero",
            self.owner, self.balance
        );
    }
}

fn split_command(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return (&input[..i], &input[i + 1..]);
        }
    }
    (input, "")
}

fn main() {
    let mut alice = Wallet::new(String::from("Alice"), 10 * PICONERO_PER_XMR);
    let mut bob = Wallet::new(String::from("Bob"), 0);

    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("...");

        let (cmd, arg) = split_command(user_input.trim());

        match cmd {
            "status" => {
                let (a_xmr, a_pico) = alice.balance_parts();
                let (b_xmr, b_pico) = bob.balance_parts();
                println!("Alice: {}.{:012} XMR", a_xmr, a_pico);
                println!("Bob:   {}.{:012} XMR", b_xmr, b_pico);
            }
            "deposit" => match arg.trim().parse::<u64>() {
                Ok(amount) => {
                    alice.deposit(amount);
                }
                Err(_) => println!("Incorrect amount"),
            },
            "send" => match arg.trim().parse::<u64>() {
                Ok(amount) => {
                    let receipt = alice.transfer_to(&mut bob, amount);
                    println!("{:?}", receipt);
                }
                Err(_) => println!("Incorrect amount"),
            },
            "history" => {
                println!("Last transfers: {:?}", alice.recent_transfers());
            }
            "burn" => {
                alice.burn();
                break;
            }
            "quit" => break,
            _ => println!("Unknown command: {}", cmd),
        }
    }
}