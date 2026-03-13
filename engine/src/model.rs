use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub version: u32,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub effect: Effect,
    pub reason: String,
    pub when: Expr,
}

/// Values used inside expressions.
/// Note: This is designed to deserialize cleanly from YAML/JSON without YAML tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    // Keep VarRef first to avoid ambiguity with generic JSON objects.
    VarRef(VarRef),

    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),

    // Fallback for any other JSON/YAML value we don't explicitly model.
    Json(JsonValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarRef {
    pub var: String,
}

/// Expression AST.
/// This is untagged so policies can be written as normal YAML maps:
/// - equals: [...]
/// - and: [...]
/// - not: { equals: [...] }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expr {
    Equals {
        equals: [Value; 2],
    },

    In {
        #[serde(rename = "in")]
        r#in: [Value; 2],
    },

    Matches {
        matches: [Value; 2],
    },

    And {
        and: Vec<Expr>,
    },
    Or {
        or: Vec<Expr>,
    },
    Not {
        not: Box<Expr>,
    },
}
