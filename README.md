# 1) start everything

docker compose up --build

# 2) test API

curl -s http://localhost:8080/v1/decide \
 -H 'Content-Type: application/json' \
 -d '{"policy":"version: 1\nrules:\n - id: default-deny\n effect: DENY\n reason: \"Default deny\"\n when:\n equals: [true, true]\n","context":{"user":{"roles":["admin"],"mfa":false},"request":{"action":"read"}}}' | jq
