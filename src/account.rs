use rust_decimal::Decimal;
use serde::Deserialize;
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
    pub client: u16,
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub amount: Option<Decimal>,
}

#[derive(Debug, Copy, Clone)]
pub struct DepositRecord {
    pub disputed: bool,
    pub amount: Decimal,
}

pub struct Account {
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
    deposits: HashMap<u32, DepositRecord>,
}

#[derive(Debug, Clone)]
struct InsufficientFundsError;

#[derive(Debug, Clone)]
struct InvalidChargebackError;

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

impl Account {
    pub fn new() -> Self {
        Self {
            available: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            locked: false,
            deposits: HashMap::new(),
        }
    }

    pub fn total(&self) -> Decimal {
        self.available + self.held
    }

    fn lock(&mut self) {
        self.locked = true;
    }

    fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    fn withdraw(&mut self, amount: Decimal) -> Result<(), InsufficientFundsError> {
        if self.available < amount {
            return Err(InsufficientFundsError);
        }
        self.available -= amount;
        Ok(())
    }

    fn dispute(&mut self, id: u32) {
        if let Some(deposit) = self.deposits.get_mut(&id) {
            if !deposit.disputed {
                deposit.disputed = true;
                self.available -= deposit.amount;
                self.held += deposit.amount;
            }
        }
    }

    fn resolve(&mut self, id: u32) {
        if let Some(deposit) = self.deposits.get_mut(&id) {
            if deposit.disputed {
                deposit.disputed = false;
                self.available += deposit.amount;
                self.held -= deposit.amount;
            }
        }
    }

    fn chargeback(&mut self, id: u32) -> Result<(), InvalidChargebackError> {
        if let Some(deposit) = self.deposits.get_mut(&id) {
            if !deposit.disputed {
                return Err(InvalidChargebackError);
            } else {
                self.held -= deposit.amount;
                self.lock()
            }
        }
        Ok(())
    }

    pub fn process(&mut self, transaction: &TransactionRecord) {
        match transaction.tx_type {
            TransactionType::Deposit => {
                match transaction.amount {
                    Some(amount) => {
                        self.deposits.insert(transaction.tx_id, DepositRecord {
                            disputed: false,
                            amount,
                        });
                        self.deposit(amount);
                    }
                    None => {
                        println!("Invalid amount specified for deposit for transaction {}", transaction.tx_id);
                    }
                }
            }
            TransactionType::Withdrawal => {
                if let Err(InsufficientFundsError) = self.withdraw(transaction.amount.unwrap()) {
                    // probably should do something else here
                    println!(
                        "Insufficient funds for transaction id {:?}",
                        transaction.tx_id
                    );
                }
            }
            TransactionType::Dispute => {
                self.dispute(transaction.tx_id);
            }
            TransactionType::Resolve => {
                self.resolve(transaction.tx_id);
            }
            TransactionType::Chargeback => {
                if let Err(InvalidChargebackError) = self.chargeback(transaction.tx_id) {
                    // probably should do something else here
                    println!(
                        "Invalid chargeback for transaction id {:?}",
                        transaction.tx_id
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rust_decimal_macros::dec;

    #[test]
    fn process_deposits_and_withdrawals() {
        let account = seed_transactions();
        assert_eq!(account.available, dec!(107.10));
    }

    #[test]
    fn process_prevents_overdraw() {
        let mut account = seed_transactions();
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Withdrawal,
            tx_id: 8,
            amount: Some(dec!(300)),
        });
        assert_eq!(account.available, dec!(107.10));
    }

    #[test]
    fn process_disputed() {
        let mut account = seed_transactions();
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Dispute,
            tx_id: 1,
            amount: None
        });
        // exact same id to make sure we don't double-dispute
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Dispute,
            tx_id: 1,
            amount: None
        });
        assert_eq!(account.available, dec!(7.09));
        assert_eq!(account.held, dec!(100.01));
    }

    #[test]
    fn process_resolve() {
        let mut account = seed_transactions();
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Dispute,
            tx_id: 1,
            amount: None
        });
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Resolve,
            tx_id: 1,
            amount: None
        });
        assert_eq!(account.available, dec!(107.10));
        assert_eq!(account.held, dec!(0));
    }

    #[test]
    fn process_chargeback() {
        let mut account = seed_transactions();
        // must be disputed first
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Dispute,
            tx_id: 1,
            amount: None,
        });
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Chargeback,
            tx_id: 1,
            amount: None,
        });
        assert!(account.locked);
        assert_eq!(account.available, dec!(7.09));
        assert_eq!(account.held, dec!(0));
    }

    #[test]
    fn process_sub_pennies() {
        let mut account = seed_transactions();
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Withdrawal,
            tx_id: 1,
            amount: Some(dec!(0.0001)),
        });
        assert_eq!(account.available, dec!(107.0999));
    }

    // test some undefinied behavior we may want to handle later
    #[test]
    fn process_dispute_over_balance() {
        let mut account = seed_transactions();
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Withdrawal,
            tx_id: 1,
            amount: Some(dec!(100.00)),
        });
        // balance is now below the dispute amount
        account.process(&TransactionRecord {
            client: 99,
            tx_type: TransactionType::Dispute,
            tx_id: 1,
            amount: Some(dec!(0)),
        });
        // available is now negative
        assert_eq!(account.available, dec!(-92.91));
        assert_eq!(account.held, dec!(100.01));
    }

    fn seed_transactions() -> Account {
        let records = vec![
            TransactionRecord {
                client: 99,
                tx_type: TransactionType::Deposit,
                tx_id: 1,
                amount: Some(dec!(100.01)),
            },
            TransactionRecord {
                client: 99,
                tx_type: TransactionType::Withdrawal,
                tx_id: 2,
                amount: Some(dec!(2.9)),
            },
            TransactionRecord {
                client: 99,
                tx_type: TransactionType::Deposit,
                tx_id: 3,
                amount: Some(dec!(9.99)),
            },
        ];
        let mut account = Account::new();
        for record in records {
            account.process(&record)
        }
        account
    }
}
