extern crate serde_json;

use serde_json::Value;
use error::{Result, Error};

pub fn hue_result(v: Value) -> Result<Value> {
    if let Some(val) = v.get(0).and_then(|v| v.get("success")) {
        return Ok(val.clone());
    }
    if let Some(err) = v.get(0).and_then(|v| v.get("error")) {
        let desc = err["description"].as_str().unwrap_or("Hue error")
            .to_owned();
        return Err(Error::Hue(desc));
    }
    Err(Error::Other("Hue Bridge sent back bad data"))
}
