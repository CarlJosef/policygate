use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
pub struct DecideRequest {
    pub policy: String, // YAML text
    pub context: JsonValue,
}

#[derive(Debug, Serialize)]
pub struct DecideResponse {
    pub decision: String,
    pub rule_id: String,
    pub reason: String,
    pub matched: bool,
}
