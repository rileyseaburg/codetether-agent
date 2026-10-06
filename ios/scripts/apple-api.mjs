// App Store Connect API helper. Vault credentials remain in process memory.
import { execFileSync } from 'node:child_process';
import { sign } from 'node:crypto';
import { readFileSync } from 'node:fs';

const secret = JSON.parse(execFileSync('vault', ['kv', 'get', '-format=json',
  'secret/codetether/ios-api-key'], { encoding: 'utf8' })).data.data;
const field = Object.keys(secret).find(name => /^AuthKey_[A-Z0-9]+\.p8$/.test(name));
if (!field) throw new Error('Apple API key missing');
const keyID = field.slice(8, -3);
const encode = value => Buffer.from(JSON.stringify(value)).toString('base64url');
const now = Math.floor(Date.now() / 1000);
const unsigned = `${encode({ alg: 'ES256', kid: keyID, typ: 'JWT' })}.${encode({
  iss: secret.issuer_id, iat: now, exp: now + 300, aud: 'appstoreconnect-v1'
})}`;
const signature = sign('sha256', Buffer.from(unsigned), {
  key: secret[field], dsaEncoding: 'ieee-p1363'
}).toString('base64url');
const [path, method = 'GET', bodyFile] = process.argv.slice(2);
if (!path?.startsWith('/v1/')) throw new Error('Expected /v1/ API path');
const response = await fetch(`https://api.appstoreconnect.apple.com${path}`, {
  method, redirect: 'error',
  headers: { Authorization: `Bearer ${unsigned}.${signature}`, 'Content-Type': 'application/json' },
  body: bodyFile ? readFileSync(bodyFile, 'utf8') : undefined
});
const result = await response.json();
console.log(JSON.stringify(result, null, 2));
if (!response.ok) process.exitCode = 1;
