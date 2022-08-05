
mod account;
use crate::account::*;
use clap::{arg, command};
use std::collections::HashMap;
use std::io::{BufRead};

pub fn get_filename() -> String {
    let matches = command!()
      .arg(arg!([FILENAME]))
      .get_matches();
    let filename = matches.value_of("FILENAME").unwrap_or("-");
    println!("{:?}", filename);
    filename.to_string()
}

pub fn read_csv(data: Box<dyn BufRead>) -> HashMap<u16, Account> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(data);

    let mut accounts = HashMap::new();

    rdr.deserialize().into_iter().for_each(|result:Result<TransactionRecord, csv::Error>| {
        match result {
            Err(_) => {
                println!("Could not parse line {:?}", result.unwrap_err());
            }
            Ok(record) => {
                let account = accounts
                    .entry(record.client)
                    .or_insert_with(Account::new);

                account.process(&record);
            }
        };
    });
    accounts
}