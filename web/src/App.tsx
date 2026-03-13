import React, { useMemo, useState } from "react";
import { decide } from "./api";

const defaultPolicy = `version: 1
rules:
  - id: deny-admin-without-mfa
    effect: DENY
    reason: "Admin actions require MFA"
    when:
      and:
        - in: ["admin", { var: "user.roles" }]
        - not:
            equals: [true, { var: "user.mfa" }]

  - id: allow-read-public
    effect: ALLOW
    reason: "Public read is allowed"
    when:
      equals: ["read", { var: "request.action" }]

  - id: default-deny
    effect: DENY
    reason: "Default deny"
    when:
      equals: [true, true]
`;

const defaultContext = `{
  "user": { "id": "u123", "roles": ["admin"], "mfa": false },
  "request": { "resource": "invoice:42", "action": "read" }
}`;

export default function App() {
  const apiBase = useMemo(
    () => import.meta.env.VITE_API_BASE ?? "http://localhost:8080",
    [],
  );

  const [policy, setPolicy] = useState(defaultPolicy);
  const [contextText, setContextText] = useState(defaultContext);
  const [result, setResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  async function onDecide() {
    setError(null);
    setResult(null);

    try {
      const ctx = JSON.parse(contextText);
      const out = await decide(apiBase, policy, ctx);
      setResult(out);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  return (
    <div
      style={{
        maxWidth: 1100,
        margin: "0 auto",
        padding: 16,
        fontFamily: "system-ui, sans-serif",
      }}
    >
      <h1>PolicyGate</h1>
      <p>
        Policy-as-Code demo: paste a policy and context, get ALLOW/DENY with
        reasons.
      </p>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12 }}>
        <div>
          <h3>Policy (YAML)</h3>
          <textarea
            value={policy}
            onChange={(e) => setPolicy(e.target.value)}
            style={{ width: "100%", height: 360 }}
          />
        </div>

        <div>
          <h3>Context (JSON)</h3>
          <textarea
            value={contextText}
            onChange={(e) => setContextText(e.target.value)}
            style={{ width: "100%", height: 360 }}
          />
        </div>
      </div>

      <div
        style={{
          marginTop: 12,
          display: "flex",
          gap: 12,
          alignItems: "center",
        }}
      >
        <button onClick={onDecide} style={{ padding: "10px 14px" }}>
          Decide
        </button>
        <span style={{ opacity: 0.7 }}>API: {apiBase}</span>
      </div>

      {error && (
        <pre
          style={{
            marginTop: 12,
            padding: 12,
            background: "#fff3f3",
            border: "1px solid #ffd1d1",
          }}
        >
          {error}
        </pre>
      )}

      {result && (
        <pre
          style={{
            marginTop: 12,
            padding: 12,
            background: "#f6f6f6",
            border: "1px solid #ddd",
          }}
        >
          {JSON.stringify(result, null, 2)}
        </pre>
      )}
    </div>
  );
}
