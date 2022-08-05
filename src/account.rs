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
    Chargeback,
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
    pub disputed: bool
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
    fn dispute(&mut self, id: u32) {
        if let Some(tx) = self.transactions.get_mut(&id) {
            if !tx.disputed {
                tx.disputed = true;
                self.available -= tx.amount;
                self.held += tx.amount;
            }
        }

    }

    fn resolve(&mut self, id: u32) { }

    fn chargeback(&mut self, id: u32) { }

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
            TransactionType::Chargeback => {
                self.chargeback(transaction.tx_id);
            }
        }
        self.transactions.insert(transaction.tx_id, *transaction);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rust_decimal_macros::dec;

    #[test]
    fn process_deposits() {
        // copilot, omg!
        let records = vec![
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 1,
                amount: Decimal::new(100, 2),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 2,
                amount: Decimal::new(200, 2),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 3,
                amount: Decimal::new(300, 2),
                disputed: false
            },
        ];
        let mut account = Account::new(1);
        for record in records {
            account.process(&record)
        }
        assert_eq!(account.available, Decimal::new(600, 2));
    }

    #[test]
    fn process_withdrawals() {
        // copilot, omg!
        let records = vec![
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 1,
                amount: Decimal::new(200, 2),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Withdrawal,
                tx_id: 2,
                amount: Decimal::new(150, 2),
                disputed: false
            },
        ];
        let mut account = Account::new(1);
        for record in records {
            account.process(&record)
        }
        assert_eq!(account.available, Decimal::new(50, 2));
    }

    #[test]
    fn process_prevents_overdraw() {
        // copilot, omg!
        let records = vec![
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 1,
                amount: Decimal::new(200, 2),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Withdrawal,
                tx_id: 2,
                amount: Decimal::new(300, 2),
                disputed: false
            },
        ];
        let mut account = Account::new(1);
        for record in records {
            account.process(&record)
        }
        assert_eq!(account.available, Decimal::new(200, 2));
    }

    #[test]
    fn process_disputed() {
        // copilot, omg!
        let records = vec![
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 1,
                amount: dec!(201.00),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Deposit,
                tx_id: 2,
                amount: dec!(140.92),
                disputed: false
            },
            TransactionRecord {
                client_id: 1,
                tx_type: TransactionType::Dispute,
                tx_id: 1,
                amount: dec!(0),
                disputed: false
            },
        ];
        let mut account = Account::new(1);
        for record in records {
            account.process(&record)
        }
        assert_eq!(account.available, dec!(140.92));
        assert_eq!(account.held, dec!(201.00));
    }

    #[test]
    fn process_resolve() {

    }
}