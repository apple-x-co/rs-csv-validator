use serde::Serialize;

#[derive(Debug)]
pub enum CsvValue {
    String(String),
    Integer(i64),
    Bool(bool),
}

impl CsvValue {
    pub fn from_str(s: &str) -> Self {
        if let Ok(num) = s.parse::<i64>() {
            return CsvValue::Integer(num);
        }

        match s.to_lowercase().as_str() {
            "true" => return CsvValue::Bool(true),
            "false" => return CsvValue::Bool(false),
            _ => {}
        }
        
        CsvValue::String(s.to_string())
    }
}

impl Serialize for CsvValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CsvValue::String(s) => serializer.serialize_str(s),
            CsvValue::Integer(i) => serializer.serialize_i64(*i),
            CsvValue::Bool(b) => serializer.serialize_bool(*b),
        }
    }
}