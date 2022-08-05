use bankster;
use rust_decimal_macros::dec;
use std::io::{BufReader};

fn bufferize(s: &str) -> Box<BufReader<&[u8]>> {
    Box::new(BufReader::new(s.as_bytes()))
}

#[test]
fn basic_data() {
    let data = bufferize("amount,client,tx,type
    100.01,  99, 1,deposit
    2.9095,  34,2, deposit  
    xxx,     xx
    9.99,    99,  3,deposit
    47.1,  34,7, deposit  
    20,      34,5,withdrawal");
    let records = bankster::read_csv(data);

    assert_eq!(records.len(), 2);
    assert_eq!(records.get(&99).unwrap().available, dec!(110));
    assert_eq!(records.get(&34).unwrap().available, dec!(30.0095));
}

#[test]
fn disputed_and_resolved_data() {
    let data = bufferize("type,client,tx,amount
    deposit,33,3,9.99
    deposit,33,4,47.1
    dispute,33,3,0,
    deposit,99,1,100.01
    deposit,99,2,2.9
    dispute,99,1,0
    resolve,33,3,0");
    let records = bankster::read_csv(data);

    assert_eq!(records.get(&99).unwrap().available, dec!(2.90));
    assert_eq!(records.get(&99).unwrap().held, dec!(100.01));
    assert_eq!(records.get(&33).unwrap().available, dec!(57.09));
    assert_eq!(records.get(&33).unwrap().held, dec!(0));
}

#[test]
fn chargeback_data() {
    let data = bufferize("type,client,tx,amount
    deposit,33,3,9.99
    deposit,33,4,47.1
    dispute,33,3,
    deposit,99,1,100.01
    deposit,99,2,2.9
    chargeback,99,1,
    chargeback,33,3,");
    let records = bankster::read_csv(data);

    // chargeback on 99 invalid because never disputed
    assert_eq!(records.get(&99).unwrap().available, dec!(102.91));
    assert_eq!(records.get(&99).unwrap().held, dec!(0));

    assert_eq!(records.get(&33).unwrap().available, dec!(47.10));
    assert_eq!(records.get(&33).unwrap().held, dec!(0));
}

#[test]
fn invalid_amounts_are_skipped() {
    let data = bufferize("type,client,tx,amount
    deposit,33,3,9.99
    deposit,33,4,BZZT");
    
    let records = bankster::read_csv(data);

    assert_eq!(records.get(&33).unwrap().available, dec!(9.99));
}