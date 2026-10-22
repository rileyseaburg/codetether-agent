#!/usr/bin/env node
'use strict';
// Fixture-backed proposal only. Actual check commands still use installed CodeTether.
const { spawnSync } = require('node:child_process');
if (process.argv[2] !== 'rust-propose') {
  const result = spawnSync(process.env.CODETETHER_REAL_BINARY || 'codetether', process.argv.slice(2), { stdio: 'inherit' });
  process.exit(result.status ?? 1);
}
let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', chunk => input += chunk);
process.stdin.on('end', () => {
  const request = JSON.parse(input);
  if (!request.files.some(file => file.path === 'src/lib.rs' && file.editable)) process.exit(2);
  process.stdout.write(JSON.stringify({ schema_version: 1, summary: 'Fixture repair: replace missing module with a function.',
    edits: [{ path: 'src/lib.rs', old_text: 'pub mod missing;', new_text: 'pub fn repaired() -> u8 { 42 }' }] }));
});