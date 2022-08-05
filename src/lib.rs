
mod account;
use crate::account::*;
use std::collections::HashMap;

pub fn read_csv(data: &str) -> HashMap<u16, Account> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut accounts = HashMap::new();

    rdr.deserialize().into_iter().for_each(|result:Result<TransactionRecord, csv::Error>| {
        match result {
            Err(_) => {
                println!("Could not parse line {:?}", result.unwrap_err());
            }
            Ok(record) => {
                let account = accounts
                    .entry(record.client_id)
                    .or_insert_with(Account::new);

                account.process(&record);
            }
        };
    });
    accounts
}