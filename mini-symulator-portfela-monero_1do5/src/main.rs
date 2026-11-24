use std::io; //do wczytywania tekstu z klawiatury

#[derive(Debug)] //atrybut nakazujacy kompilatorowi generowania sposobu wypisywania struktury
struct Wallet { //struktura reprezentujaca portfel
    owner: String,
    balance: u64,
    tx_count: usize, //laczna liczba wykonywanych przelewow
    last_txs: [u64; 5], //tablica o stalym rozmiarze 5 elementow, trzymajaca kwoty ostatnich 5 przelewow
}

#[derive(Debug)]
struct TransferReceipt { //struktura na paragon zwracany po kazdej probie przelewu
    amount: u64, //kwota przelewu
    fee: u64, //pobrana prowizja
    success: bool, //czy przelew sie powiodl
}

const PICONERO_PER_XMR: u64 = 1_000_000_000_000; //ile piconero miesci 1XMR
const TX_FEE: u64 = 10_000; //stala prowizja

impl Wallet { //generuje funkcje i metody powiazane z portfelem
    fn new(owner: String, initial_balance: u64) -> Wallet { //funkcja przyjmujaca nazwe wlasiciciela i poczatkowy stan konta, zwracajaca nowy portfel
        Wallet {
            owner,
            balance: initial_balance,
            tx_count: 0,
            last_txs: [0; 5],
        }
    }

    fn balance_parts(&self) -> (u64, u64) { //funkcja pozycza portfel niemutowalnie (do odczytu) i nie moze niczego zmienic. zwraca krotke dwoch liczb
        let whole: u64 = self.balance / PICONERO_PER_XMR;
        let fractional: u64 = self.balance % PICONERO_PER_XMR;
        (whole, fractional)
    }

    fn deposit(&mut self, amount: u64) { //pozycza portfel mutowalnie (do zmiany) i dodaje do niego kwote
        self.balance += amount;
    }

    //przelewa kwote z portfela do portfela, zwraca paragon
    fn transfer_to(&mut self, recipient: &mut Wallet, amount: u64) -> TransferReceipt {
        //sprawdza czy kwota jest dodatnia, czy portfel ma wystarczajaco srodkow na przelew i prowizje, oraz czy kwota nie przekracza srodkow portfela po odjeciu prowizji
        if amount > 0 && self.balance >= TX_FEE && amount <= self.balance - TX_FEE {
            self.balance -= amount + TX_FEE; //odejmujesz kwote i prowizje od nadawcy
            self.last_txs[self.tx_count % 5] = amount; //zapisujesz kwote do bufora kolowego. reszta z dzielenia daje idneksy 0, 1, 2, 3, 4 a przy 6 wraca i nadpisuje najstarszy
            self.tx_count += 1; //zwieksza licznik przelewow
            recipient.balance += amount; //dodaje kwote do portfela odbiorcy
            TransferReceipt { //zwrocenie paragonu z informacji o przelewie
                amount,
                fee: TX_FEE,
                success: true,
            }
        } else { //jesli warunki nie sa spelnione
            TransferReceipt {
                amount,
                fee: 0,
                success: false,
            }
        }
    }

    fn recent_transfers(&self) -> &[u64] { //do odczytu, zwraca wycinek tablicy ostatnich przelewow
        &self.last_txs[0..self.tx_count.min(5)]
    }

    fn burn(self) { //przejmuje portfel na wlasnosc (brak &) i niszczy portfel
        println!(
            "Wallet of {} burned. Final balance: {} piconero",
            self.owner, self.balance
        );
    }
}

//przyjmuje wycinek napisu i zweraca pare dwoch wycinkow wskazujacych na fragmenty tego samego napisu (komenda, argument)
fn split_command(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes(); //poglda napisu jako tablicy bajtow, co pozwala przechodzic po nim numerami idenksow

    for (i, &byte) in bytes.iter().enumerate() { //iteruje po kazdym bajcie, podaje kolejno referencje do bajtow oraz dokleja do kazdeg elementu jego pozycje od zera tworzac pary (indeks, &bajt)
        if byte == b' ' { //szuka spacji
            return (&input[..i], &input[i + 1..]); //po znalezieniu przerywa i zwraca dwa wycinki napisu 
        }
    }
    (input, "") //wykonuje sie wtedy gdy petla przejrzala napis i nie znalazla spacji
}

fn main() {
    //tworzy portfele mutowalne z saldami poczatkowymi
    let mut alice = Wallet::new(String::from("Alice"), 10 * PICONERO_PER_XMR);
    let mut bob = Wallet::new(String::from("Bob"), 0);

    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("...");

        let (cmd, arg) = split_command(user_input.trim()); //rozpakowuje zwrocona krotke do dwoch osobnych zmiennych &str

        match cmd {
            "status" => { //wypisuje stan portfela w formacie XMR.piconero
                let (a_xmr, a_pico) = alice.balance_parts();
                let (b_xmr, b_pico) = bob.balance_parts();
                println!("Alice: {}.{:012} XMR", a_xmr, a_pico);
                println!("Bob:   {}.{:012} XMR", b_xmr, b_pico);
            }
            "deposit" => match arg.trim().parse::<u64>() {//czysci spacje, zamienia tekst na liczbe calkowita i sprawdza czy sie udalo 
                Ok(amount) => {
                    alice.deposit(amount);
                }
                Err(_) => println!("Incorrect amount"),
            },
            "send" => match arg.trim().parse::<u64>() {//parsuje arg do u64
                Ok(amount) => {
                    let receipt = alice.transfer_to(&mut bob, amount); //mutowalna pozyczka portfela boba do metody na alice 
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