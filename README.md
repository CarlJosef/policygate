![CI](https://github.com/CarlJosef/policygate/actions/workflows/ci.yml/badge.svg)

## 2) Test API

### PowerShell (Windows)

```powershell
$policy = @"
version: 1
rules:
  - id: default-deny
    effect: DENY
    reason: Default deny
    when:
      equals: [true, true]
"@

$body = @{
  policy  = $policy
  context = @{
    user    = @{ roles = @("admin"); mfa = $false }
    request = @{ action = "read" }
  }
} | ConvertTo-Json -Depth 10

Invoke-RestMethod -Method Post -Uri "http://localhost:8080/v1/decide" -ContentType "application/json" -Body $body
```
