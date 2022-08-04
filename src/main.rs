use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TransactionRecord {
    #[serde(rename = "type")]
    tx_type: String,
    #[serde(rename = "tx")]
    tx_id: u32,
    client_id: u16,
    amount: i32,
}

fn main() {
    let data = "amount,client_id,tx,type\n100,99,1,deposit\n2,34,2,withdrawal\nxxx,xx\n10,99,3,deposit\n-80,34,5,withdrawal";
    let lines = read_csv(data);

    println!("Processed {:?} transactions", lines.len());
    for line in lines {
        println!("client: {}: ${:?}", line.client_id, line.amount);
    }
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