use serde::Deserialize;
// use std::error::Error;
// use rust_decimal_macros::dec;
use rust_decimal::Decimal;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
}

#[derive(Debug, Copy, Clone, Deserialize)]
struct TransactionRecord {
    client_id: u16,
    #[serde(rename = "type")]
    tx_type: TransactionType,
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

    fn resolve(&mut self, id: u32) { }

    fn process(&mut self, transaction: &TransactionRecord) {
        match transaction.tx_type {
            TransactionType::Deposit => {
                self.deposit(transaction.amount);
            },
            TransactionType::Withdrawal => {
                if let Err(InsufficientFundsError) = self.withdraw(transaction.amount) {
                    println!("Insufficient funds");
                }
            },
            TransactionType::Dispute => {
                self.dispute(transaction.tx_id);
            }
            TransactionType::Resolve => {
                self.resolve(transaction.tx_id);
            }
        }
        self.transactions.insert(transaction.tx_id, *transaction);
    }
}


fn main() {
    let data = "amount,client_id,tx,type\n100.01,99,1,deposit\n2.9,34,2,withdrawal\nxxx,xx\n9.99,99,3,deposit\n-80,34,5,withdrawal";
    let records = read_csv(data);

    println!("Processed {:?} transactions", records.len());

    let mut accounts = HashMap::new();

    let mut total_deposits = Decimal::new(0, 2);
    
    for record in records {
        total_deposits += record.amount;

        println!("{}: ${:?}, {:?}", record.client_id, record.amount, record.tx_type);

        let account = accounts.entry(record.client_id)
          .or_insert_with(|| Account::new(record.client_id));

        account.process(&record);
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