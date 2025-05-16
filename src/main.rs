mod csv_value;

use crate::csv_value::CsvValue;
use anyhow::Result;
use clap::Parser;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    csv: String,

    #[arg(short, long)]
    schema: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    //////////
    let schema_reader = fs::File::open(args.schema)?;
    let schema = serde_json::from_reader(schema_reader)?;
    let validator = jsonschema::validator_for(&schema)?;

    //////////
    let mut reader = csv::Reader::from_path(args.csv)?;

    let headers: Vec<String> = reader.headers()?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let number_of_headers = headers.len();
    
    for result in reader.records() {
        let record = result?;

        let mut data: HashMap<&String, CsvValue> = HashMap::new();
        for i in 0..=number_of_headers-1 {
            data.insert(&headers[i], CsvValue::from_str(&record[i]));
        }
        let json: Value = json!(data);

        let validate_result = validator.validate(&json);
        if !validate_result.is_ok() {
            println!("record: {:?}", record);
            println!("error: {:?}", validate_result.err().unwrap());
        }
    }

    Ok(())
}