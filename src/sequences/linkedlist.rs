use rand::Rng;
use std::time::SystemTime;

pub type Transactions = Vec<Transaction>;

#[derive(Debug)]
pub struct Owner {
    id: String,
    username: String,
}

#[derive(Debug)]
pub struct Transaction {
    id: String,
    owner: Owner,
    mov_type: String,
    amount: f64,
    ts: SystemTime,
}

#[derive(Debug)]
pub struct Ledger {
    id: String,
    transactions: Transactions,
}

pub fn gen_random_id(length: i64) -> String {
    let mut res = String::from("");
    let mut rng = rand::thread_rng();
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let tot = letters.len();

    let coll: Vec<char> = letters.chars().collect();

    let mut cur = 0;
    while cur < length {
        let cur_rand = rng.gen_range(0..tot);

        let cur_char = coll[cur_rand];
        res.push(cur_char);

        cur += 1;
    }

    res
}

impl Owner {
    pub fn new(username: &str) -> Self {
        let username_str = String::from(username);

        let id = gen_random_id(10);

        Owner {
            id,
            username: username_str,
        }
    }
}

impl Transaction {
    pub fn new(owner: Owner, mov_type: &str, amount: f64) -> Self {
        let id = gen_random_id(10);
        let mov_type_str = String::from(mov_type);

        Transaction {
            id,
            owner,
            mov_type: mov_type_str,
            amount,
            ts: SystemTime::now(),
        }
    }
}

impl Ledger {
    pub fn new() -> Self {
        let id = gen_random_id(10);
        Ledger {
            id,
            transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, t: Transaction) -> () {
        self.transactions.push(t);
    }
}
