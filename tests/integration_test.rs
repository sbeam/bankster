use bankster;

#[test]
fn test_basic_data() {
    let data = "amount,client_id,tx,type\n100.01,99,1,deposit\n2.9,34,2,withdrawal\nxxx,xx\n9.99,99,3,deposit\n-80,34,5,withdrawal";
    let records = bankster::read_csv(data);

    assert_eq!(records.len(), 2);
}