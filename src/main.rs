fn main() {
    let data = "amount,client_id,tx,type\n100.01,99,1,deposit\n2.9,34,2,withdrawal\nxxx,xx\n9.99,99,3,deposit\n-80,34,5,withdrawal";
    let accounts = bankster::read_csv(data);

    println!("Processed {:?} accounts", accounts.len());
}