// Native fallback for TetherScript JS-engine compatibility; run from repo root.
const fs = require("node:fs");
const vm = require("node:vm");
const source = fs.readFileSync("docs/provenance-audit/source.layout.txt", "utf8");
const document = fs.readFileSync("docs/draft-seaburg-codetether-agent-provenance-00.md", "utf8");
const script = fs.readFileSync("docs/provenance-audit/compare.js", "utf8");
const result = vm.runInNewContext(script, { source, document }, { timeout: 5000 });
console.log(result);
// Preserve the report for inspection, including unsuccessful comparisons.
fs.writeFileSync("docs/provenance-audit/validation.json", result + "\n");
process.exitCode = JSON.parse(result).ok ? 0 : 1;