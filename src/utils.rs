extern crate serde_json;

use serde_json::Value;
use error::{Result, Error};

fn get_error_desc(v: &Value) -> Option<String> {
    v.get("error")
        .and_then(|e| e.get("description"))
        .and_then(|d| d.as_str())
        .map(str::to_owned)
}

pub fn hue_result(v: Value) -> Result<Value> {
    // Try to grab first spot of the json array or return an error
    let first = v.get(0).ok_or(Error::Other("Hue Bridge sent back bad data"))?;
    first.get("success")
        .map(|v| v.clone())
        .ok_or(Error::Hue(get_error_desc(&first)
            .unwrap_or("Hue Bridge sent back bad data".to_owned())))
}
