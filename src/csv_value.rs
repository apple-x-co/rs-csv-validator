use serde::Serialize;

#[derive(Debug)]
pub enum CsvValue {
    String(String),
    Null,
    Integer(i64),
    Number(f64),
    Bool(bool),
}

impl CsvValue {
    pub fn from_str(s: &str) -> Self {
        if let Ok(int) = s.parse::<i64>() {
            return CsvValue::Integer(int);
        }

        if let Ok(num) = s.parse::<f64>() {
            return CsvValue::Number(num);
        }

        match s.to_lowercase().as_str() {
            "true" => return CsvValue::Bool(true),
            "false" => return CsvValue::Bool(false),
            "" => return CsvValue::Null,
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
            CsvValue::Null => serializer.serialize_none(),
            CsvValue::Integer(i) => serializer.serialize_i64(*i),
            CsvValue::Number(n) => serializer.serialize_f64(*n),
            CsvValue::Bool(b) => serializer.serialize_bool(*b),
        }
    }
}