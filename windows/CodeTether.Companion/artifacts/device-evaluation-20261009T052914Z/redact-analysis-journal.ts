import process from 'node:process';

/** Read journal JSON on stdin; emit only allowlisted diagnostics, never raw messages. */
const signals = ['invalid_image', 'unsupported image', 'base64', '401', '403', '404',
  '429', 'quota', 'rate limit', 'usage limit', 'timed out', 'timeout', 'not found',
  'unknown model', 'unsupported model', 'unauthorized', 'authentication',
  'stream error', 'streaming failed', 'failed to stream', 'provider error'];
let input = '';
for await (const chunk of process.stdin) input += String(chunk);
const observations: { timestamp: string; signals: string[]; models: string[] }[] = [];
let records = 0;
for (const line of input.trim().split('\n')) {
  let value: unknown;
  try { value = JSON.parse(line); } catch { continue; }
  if (typeof value !== 'object' || value === null) continue;
  const record = value as Record<string, unknown>;
  records++;
  if (typeof record.MESSAGE !== 'string') continue;
  const message = record.MESSAGE.replace(/\u001b\[[0-9;]*m/g, '');
  const found = signals.filter(signal => message.toLowerCase().includes(signal));
  if (!found.length) continue;
  const models = [...message.matchAll(/(?:requested_model|resolved_model|model_name|model)\s*[=:]\s*"?([a-zA-Z0-9_.:/-]{1,120})/g)].map(match => match[1]);
  const timestamp = typeof record.__REALTIME_TIMESTAMP === 'string'
    && /^\d+$/.test(record.__REALTIME_TIMESTAMP) ? record.__REALTIME_TIMESTAMP : '';
  observations.push({ timestamp, signals: found, models });
}
console.log(JSON.stringify({ records, observations: observations.slice(-60) }, null, 2));