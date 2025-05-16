use crate::csv_value::CsvValue;
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;

pub fn validate(schema_path: &String, csv_path: &String) -> Result<()> {
    let schema_reader = fs::File::open(schema_path)?;
    let schema = serde_json::from_reader(schema_reader)?;
    let validator = jsonschema::validator_for(&schema)?;

    let mut reader = csv::Reader::from_path(csv_path)?;
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    let number_of_headers = headers.len();

    for (i, result) in reader.records().enumerate() {
        let record = result?;

        let mut data: HashMap<&String, CsvValue> = HashMap::new();
        for i in 0..=number_of_headers - 1 {
            data.insert(&headers[i], CsvValue::from_str(&record[i]));
        }
        let json: Value = json!(data);

        // TODO: エラー JSON にする

        let result = validator.validate(&json);
        if let Some(err) = result.err() {
            eprintln!(
                "line:{}, colum:{}, error:{}",
                i + 1,
                &err.instance_path.to_string()[1..],
                err.to_string()
            );
        }
    }

    Ok(())
}
