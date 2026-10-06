// Vault-owned credentials never enter command arguments or evidence files.
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
const env = { ...process.env, VAULT_TOKEN: process.env.VAULT_TOKEN ||
  readFileSync(join(process.env.HOME, '.config/vault-agent/token'), 'utf8').trim() };
const secret = JSON.parse(execFileSync('vault', ['kv', 'get', '-format=json',
  'kv/spotlessbinco-api/sendgrid'], { encoding: 'utf8', env, stdio: ['ignore', 'pipe', 'pipe'] })).data.data;
const key = secret.SENDGRID_API_KEY;
if (typeof key !== 'string' || !key.startsWith('SG.')) throw new Error('Vault SendGrid key missing');
export const configuredSender = secret.SENDGRID_FROM_EMAIL;
export async function sendgrid(path, body) {
  const response = await fetch(`https://api.sendgrid.com/v3/${path}`, {
    method: body ? 'POST' : 'GET', signal: AbortSignal.timeout(30_000),
    headers: { Authorization: `Bearer ${key}`, 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined });
  return { status: response.status, messageId: response.headers.get('x-message-id'),
    body: await response.text() };
}