use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use clap::{arg, command};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    let filename = get_filename();
    match open(&filename) {
        Ok(file) => {
            let accounts = bankster::read_csv(file);
            println!("Processed {:?} accounts", accounts.len());
            bankster::report(&accounts);
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

fn get_filename() -> String {
    let matches = command!()
      .arg(arg!([FILENAME]))
      .get_matches();
    let filename = matches.value_of("FILENAME").unwrap_or("-");
    filename.to_string()
}
