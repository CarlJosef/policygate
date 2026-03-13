use axum::{Json, http::StatusCode};
use crate::types::{DecideRequest, DecideResponse};
use policygate_engine::{decide, Policy};

pub async fn decide_route(Json(req): Json<DecideRequest>) -> Result<Json<DecideResponse>, (StatusCode, String)> {
    let policy: Policy = serde_yaml::from_str(&req.policy)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid policy YAML: {e}")))?;

    let d = decide(&policy, &req.context)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Evaluation error: {e}")))?;

    Ok(Json(DecideResponse {
        decision: d.effect,
        rule_id: d.rule_id,
        reason: d.reason,
        matched: d.matched,
    }))
}
