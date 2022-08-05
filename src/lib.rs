mod account;
use crate::account::*;
use std::collections::HashMap;
use std::io::{self, BufRead, Result as MyResult};

pub fn read_csv(data: Box<dyn BufRead>) -> HashMap<u16, Account> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        // .flexible(true) // unsure if appropriate
        .from_reader(data);

    let mut accounts = HashMap::new();

    rdr.deserialize()
        .into_iter()
        .for_each(|result: Result<TransactionRecord, csv::Error>| {
            match result {
                Err(_) => {
                    println!("Could not parse line {:?}", result.unwrap_err());
                }
                Ok(record) => {
                    let account = accounts.entry(record.client).or_insert_with(Account::new);

                    account.process(&record);
                }
            };
        });
    accounts
}

pub fn report(accounts: &HashMap<u16, Account>) -> MyResult<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["client", "available", "total", "held", "locked"])
        .unwrap();
    for (id, account) in accounts {
        wtr.write_record(&[
            &id.to_string(),
            &account.available.to_string(),
            &account.total().to_string(),
            &account.held.to_string(),
            &account.locked.to_string(),
        ])
        .unwrap();
    }
    wtr.flush()?;
    Ok(())
}
