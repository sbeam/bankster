
mod account;
use crate::account::*;
use clap::{App, Arg};
use std::collections::HashMap;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
    }


pub fn read_csv(data: &str) -> HashMap<u16, Account> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(data.as_bytes());

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