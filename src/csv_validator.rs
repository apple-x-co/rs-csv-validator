use crate::csv_value::CsvValue;
use anyhow::Result;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize)]
struct CsvError {
    line: usize,
    column: String,
    error: String,
}

pub fn validate(schema_path: &String, csv_path: &String) -> Result<()> {
    let schema_reader = fs::File::open(schema_path)?;
    let schema = serde_json::from_reader(schema_reader)?;
    let validator = jsonschema::validator_for(&schema)?;

    let mut reader = csv::Reader::from_path(csv_path)?;
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    let number_of_headers = headers.len();

    // let mut errors: Vec<HashMap<&str, String>> = Vec::new();
    let mut errors: Vec<CsvError> = Vec::new();

    for (i, result) in reader.records().enumerate() {
        let record = result?;

        let mut data: HashMap<&String, CsvValue> = HashMap::new();
        for i in 0..=number_of_headers - 1 {
            data.insert(&headers[i], CsvValue::from_str(&record[i]));
        }
        let json: Value = json!(data);

        let result = validator.validate(&json);
        if let Some(err) = result.err() {
            errors.push(CsvError {
                line: i + 1,
                column: err.instance_path.to_string()[1..].to_string(),
                error: err.to_string(),
            });
        }
    }

    if errors.len() > 0 {
        let errors_json = json!(errors);
        eprintln!("{}", errors_json.to_string());
    }

    Ok(())
}
