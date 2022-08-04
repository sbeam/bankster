use serde::Deserialize;
// use std::error::Error;
use rust_decimal_macros::dec;
use rust_decimal::Decimal;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TransactionRecord {
    client_id: u16,
    #[serde(rename = "type")]
    tx_type: String,
    #[serde(rename = "tx")]
    tx_id: u32,
    #[serde(with = "rust_decimal::serde::float")]
    amount: Decimal,
}

#[derive(Debug, Clone)]
struct InsufficientFundsError;

struct Account {
    id: u16,
    available: Decimal,
    held: Decimal,
    locked: bool,
    transactions: HashMap<u32, TransactionRecord>
}

impl Account {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: Decimal::new(0, 2),
            held: Decimal::new(0, 2),
            locked: false,
            transactions: HashMap::new()
        }
    }

    // lock an Account
    fn lock(&mut self) {
        self.locked = true;
    }

    // unlock an Account
    fn unlock(&mut self) {
        self.locked = false;
    }

    // deposit money into an Account
    fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    // withdraw money from an Account
    fn withdraw(&mut self, amount: Decimal) -> Result<(), InsufficientFundsError> {
        if self.available < amount {
            return Err(InsufficientFundsError);
        }
        self.available -= amount;
        Ok(())
    }

    // dispute a transaction
    fn dispute(&mut self, id: u32) { }

}


fn main() {
    let data = "amount,client_id,tx,type\n100.01,99,1,deposit\n2.9,34,2,withdrawal\nxxx,xx\n9.99,99,3,deposit\n-80,34,5,withdrawal";
    let lines = read_csv(data);

    println!("Processed {:?} transactions", lines.len());

    let mut accounts = HashMap::new();

    let mut total_deposits = Decimal::new(0, 2);
    for line in lines {
        total_deposits += line.amount;

        println!("{}: ${:?}, {}", line.client_id, line.amount, line.tx_type);

        let account = accounts.entry(line.client_id)
          .or_insert_with(|| Account::new(line.client_id));

        match line.tx_type.as_str() {
            "deposit" => {
                account.deposit(line.amount);
            },
            "withdrawal" => {
                if let Err(InsufficientFundsError) = account.withdraw(line.amount) {
                    println!("Insufficient funds");
                }
            },
            "dispute" => {
                account.dispute(line.tx_id);
            }
            _ => {
                println!("Unknown transaction type: {}", line.tx_type);
            }
        }
        account.transactions.insert(line.tx_id, line);
    };
    println!("total deposits: ${:?}", total_deposits);
}

fn read_csv(data: &str) -> Vec<TransactionRecord> {
    let mut records = Vec::new();
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    rdr.deserialize().into_iter().for_each(|result| {
        match result {
            Err(_) => {
                println!("Could not parse line {:?}", result.unwrap_err());
            },
            Ok(record) => { records.push(record); }
        };
    });
    records
}