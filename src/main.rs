use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    let filename = bankster::get_filename();
    match open(&filename) {
        Ok(file) => {
            let accounts = bankster::read_csv(file);
            println!("Processed {:?} accounts", accounts.len());
        }
        Err(err) => {
            eprintln!("{}: {:?}", filename, err);
        }
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}