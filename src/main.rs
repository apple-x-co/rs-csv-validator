mod csv_value;

use crate::csv_value::CsvValue;
use anyhow::Result;
use clap::Parser;
use serde_json::{Value, json};
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

    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();

    let number_of_headers = headers.len();

    let mut i = 0;
    for result in reader.records() {
        i += 1;

        let record = result?;

        let mut data: HashMap<&String, CsvValue> = HashMap::new();
        for i in 0..=number_of_headers - 1 {
            data.insert(&headers[i], CsvValue::from_str(&record[i]));
        }
        let json: Value = json!(data);

        let result = validator.validate(&json);
        if let Some(err) = result.err() {
            eprintln!(
                "line:{}, colum:{}, error:{}",
                i,
                &err.instance_path.to_string()[1..],
                err.to_string()
            );
        }
    }

    Ok(())
}
