use std::error::Error;
use std::io;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    #[serde(rename(serialize = "Date", deserialize = "Transaction date"))]
    date: String,
    #[serde(rename(serialize = "Payee", deserialize = "Description"))]
    payee: String,
    #[serde(rename(serialize = "Amount", deserialize = "Amount"))]
    amount: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(io::stdin());

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(io::stdout());

    for result in reader.deserialize() {
        let record: Transaction = result?;

        writer.serialize(record)?;
    }

    writer.flush().expect("writing to stdout succeeds");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error: {}", err);
        process::exit(1);
    }
}
