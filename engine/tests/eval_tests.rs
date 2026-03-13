use policygate_engine::{decide, Policy};
use serde_json::json;

#[test]
fn denies_admin_without_mfa() {
    let policy_yaml = r#"
version: 1
rules:
  - id: deny-admin-without-mfa
    effect: DENY
    reason: "Admin actions require MFA"
    when:
      and:
        - in: ["admin", { var: "user.roles" }]
        - not:
            equals: [true, { var: "user.mfa" }]
"#;

    let policy: Policy = serde_yaml::from_str(policy_yaml).unwrap();
    let ctx =
        json!({ "user": { "roles": ["admin"], "mfa": false }, "request": { "action": "read" } });
    let d = decide(&policy, &ctx).unwrap();
    assert_eq!(d.effect, "DENY");
}
