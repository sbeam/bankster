use serde::Deserialize;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct TransactionRecord {
    pub client_id: u16,
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
}

pub struct Account {
    id: u16,
    available: Decimal,
    held: Decimal,
    locked: bool,
    transactions: HashMap<u32, TransactionRecord>
}

#[derive(Debug, Clone)]
struct InsufficientFundsError;

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

    pub fn process(&mut self, transaction: &TransactionRecord) {
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