#[derive(Debug)]
pub struct Tx{
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub note: String,
}

impl Tx{
    pub fn new(from: &str, to: &str, amount: u64, note: &str) -> Tx {
        Tx {from: from.to_string(), to: to.to_string(), amount, note: note.to_string()}
    }

    pub fn summary(&self) -> String {
        let chars = self.note.chars().count();
        format!("{}-{}-{}-{}-{chars}", self.from, self.to, self.amount, self.note)
    }
}