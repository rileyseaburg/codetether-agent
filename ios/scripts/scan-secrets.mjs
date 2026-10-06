// Exact-value audit: report paths only, never secret contents.
import { execFileSync } from 'node:child_process';
import { auditFiles } from './audit-files.mjs';
/** @param {string} path @returns {Record<string, unknown>} */
function load(path) {
  try {
    return JSON.parse(execFileSync('vault', ['kv', 'get', '-format=json', path],
      { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] })).data.data;
  } catch {
    // Do not expose Vault output or captured credential material on failure.
    throw new Error(`Required Vault audit input unavailable: ${path}`);
  }
}
const apple = load('secret/codetether/ios-api-key');
const secrets = [load('secret/codetether/endpoints/public-server').token,
  load('secret/codetether/ios-signing-development').private_key_pem,
  load('secret/codetether/ios-signing/distribution-6JF3S3B8MP').private_key_pem,
  load('kv/cloudflare/api-token').token,
  ...Object.entries(apple).filter(([key]) => key.endsWith('.p8')).map(([, value]) => value)]
  .filter(value => typeof value === 'string' && value.length > 16)
  .map(value => Buffer.from(value.trim()));
if (secrets.length < 3) throw new Error('Required Vault audit inputs missing');
const { count, archives, matches } = auditFiles(process.argv.slice(2), secrets);
console.log(JSON.stringify({ validation: 'static/local', files: count, archives,
  auditedValues: secrets.length, matches, scope: 'Exact values, including decompressed IPA/ZIP contents' },
  null, 2));
if (matches.length) { console.error('Credential leak in:', matches); process.exitCode = 1; }