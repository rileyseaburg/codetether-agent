// Read a Vault KV-v2 record from stdin; never log credentials or error bodies.
import { readFileSync } from 'node:fs';
const endpoint = 'https://api.groq.com/openai/v1/models';
try {
  const record = JSON.parse(readFileSync(0, 'utf8'));
  const key = record?.data?.data?.api_key;
  if (typeof key !== 'string' || !key.trim()) throw new Error();
  const response = await fetch(endpoint, {
    headers: { Authorization: `Bearer ${key}` },
    redirect: 'error',
    signal: AbortSignal.timeout(20000),
  });
  if (!response.ok) {
    console.log(JSON.stringify({ endpoint, http_status: response.status }));
    process.exitCode = 1;
  } else {
    const body = await response.json();
    const models = body.data.filter(model => model.active !== false).map(model => model.id);
    console.log(JSON.stringify({ endpoint, http_status: response.status, active_models: models }, null, 2));
  }
} catch {
  console.error('Groq catalog check failed; credentials and response bodies were not logged.');
  process.exitCode = 1;
}