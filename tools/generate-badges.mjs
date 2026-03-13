import { makeBadge } from "badge-maker";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const outDir = join(process.cwd(), "docs", "badges");
mkdirSync(outDir, { recursive: true });

const badges = [
  {
    file: "stack.svg",
    label: "stack",
    message: "rust + axum + react",
    color: "blue",
  },
  {
    file: "runtime.svg",
    label: "runtime",
    message: "docker compose",
    color: "informational",
  },
  { file: "policy.svg", label: "policy", message: "yaml v1", color: "success" },
];

for (const b of badges) {
  const svg = makeBadge({
    label: b.label,
    message: b.message,
    color: b.color,
  });
  writeFileSync(join(outDir, b.file), svg, "utf8");
}

console.log(`[OK] wrote ${badges.length} badges to ${outDir}`);
