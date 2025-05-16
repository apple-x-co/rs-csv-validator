use std::any::Any;
use std::collections::HashMap;
use std::fs;
use clap::Parser;
use anyhow::Result;
use serde_json::{json, Value};

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

        // FIXME: 値が全て文字型として登録されてしまう...
        let mut data = HashMap::new();
        // let mut data: HashMap<&String, Box<dyn Any>> = HashMap::new();
        for i in 0..=number_of_headers-1 {
            data.insert(&headers[i], &record[i]);
            // data.insert(&headers[i], Box::new(&record[i]));
        }
        println!("data: {:?}", data);

        let json: Value = json!(data);
        println!("json: {:?}", json);

        let validate_result = validator.validate(&json);

        if !validate_result.is_ok() {
            println!("record: {:?}", record);
            println!("error: {:?}", validate_result.err().unwrap());
        }
    }

    Ok(())
}