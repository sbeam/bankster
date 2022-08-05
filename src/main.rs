// use std::error::Error;
// use rust_decimal_macros::dec;
use rust_decimal::Decimal;

use std::collections::HashMap;

mod account;

pub use crate::account::*;

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
          .or_insert_with(Account::new);

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