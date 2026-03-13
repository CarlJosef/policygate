use crate::model::{Expr, Policy, Value, VarRef};
use regex::Regex;
use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("invalid variable path: {0}")]
    InvalidVarPath(String),
    #[error("type mismatch: {0}")]
    TypeMismatch(String),
    #[error("regex error: {0}")]
    Regex(String),
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub rule_id: String,
    pub effect: String, // "ALLOW" / "DENY"
    pub reason: String,
    pub matched: bool,
}

pub fn decide(policy: &Policy, ctx: &JsonValue) -> Result<Decision, EvalError> {
    for rule in &policy.rules {
        let matched = eval_expr(&rule.when, ctx)?;
        if matched {
            return Ok(Decision {
                rule_id: rule.id.clone(),
                effect: match rule.effect {
                    crate::model::Effect::Allow => "ALLOW".to_string(),
                    crate::model::Effect::Deny => "DENY".to_string(),
                },
                reason: rule.reason.clone(),
                matched: true,
            });
        }
    }

    Ok(Decision {
        rule_id: "no_rule".to_string(),
        effect: "DENY".to_string(),
        reason: "No rule matched".to_string(),
        matched: false,
    })
}

fn eval_expr(expr: &Expr, ctx: &JsonValue) -> Result<bool, EvalError> {
    match expr {
        Expr::Equals { equals } => {
            let a = resolve_value(&equals[0], ctx)?;
            let b = resolve_value(&equals[1], ctx)?;
            Ok(a == b)
        }
        Expr::In { r#in } => {
            let needle = resolve_value(&r#in[0], ctx)?;
            let hay = resolve_value(&r#in[1], ctx)?;
            match hay {
                JsonValue::Array(arr) => Ok(arr.iter().any(|v| v == &needle)),
                _ => Err(EvalError::TypeMismatch(
                    "in expects array as second operand".into(),
                )),
            }
        }
        Expr::Matches { matches } => {
            let text = resolve_value(&matches[0], ctx)?;
            let pat = resolve_value(&matches[1], ctx)?;
            let text = text
                .as_str()
                .ok_or_else(|| EvalError::TypeMismatch("matches expects string text".into()))?;
            let pat = pat
                .as_str()
                .ok_or_else(|| EvalError::TypeMismatch("matches expects string pattern".into()))?;
            let re = Regex::new(pat).map_err(|e| EvalError::Regex(e.to_string()))?;
            Ok(re.is_match(text))
        }
        Expr::And { and } => {
            for e in and {
                if !eval_expr(e, ctx)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Expr::Or { or } => {
            for e in or {
                if eval_expr(e, ctx)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Expr::Not { not } => Ok(!eval_expr(not, ctx)?),
    }
}

fn resolve_value(v: &Value, ctx: &JsonValue) -> Result<JsonValue, EvalError> {
    match v {
        Value::Bool(b) => Ok(JsonValue::Bool(*b)),
        Value::Number(n) => Ok(JsonValue::Number(
            serde_json::Number::from_f64(*n)
                .ok_or_else(|| EvalError::TypeMismatch("invalid number".into()))?,
        )),
        Value::String(s) => Ok(JsonValue::String(s.clone())),
        Value::Array(a) => Ok(JsonValue::Array(
            a.iter()
                .map(|x| resolve_value(x, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Value::VarRef(VarRef { var }) => get_var(ctx, var),
        Value::Json(j) => Ok(j.clone()),
    }
}

fn get_var(ctx: &JsonValue, path: &str) -> Result<JsonValue, EvalError> {
    // Supports dot paths like "user.roles" or "request.action"
    let mut cur = ctx;
    for seg in path.split('.') {
        cur = cur
            .get(seg)
            .ok_or_else(|| EvalError::InvalidVarPath(path.into()))?;
    }
    Ok(cur.clone())
}
