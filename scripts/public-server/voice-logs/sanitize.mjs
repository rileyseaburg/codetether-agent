import { createHash } from 'node:crypto';
const sources = { 'codetether-audio.service': 'bridge', 'kokoro-tts.service': 'kokoro',
  'codetether-spotlessbinco-thinker.service': 'agent' };
const values = {
  event: ['voice_bridge', 'tts_request'], status: ['started', 'progress', 'completed', 'failed', 'error', 'client_disconnected'],
  stage: ['route', 'authorization', 'body', 'synthesis', 'upstream_response'],
  route: ['/tts/speak', '/tts/voices', '/tts/health', '/mobile/attachments', '/mobile/image', 'unknown'],
  method: ['GET', 'POST', 'other'], error_type: ['AbortError', 'TimeoutError', 'TypeError', 'HTTPError', 'Error']
};
/** Explicit allowlist: raw journal messages, text, headers and exception strings never reach NAS. */
export function sanitize(entry) {
  const source = sources[entry._SYSTEMD_USER_UNIT || entry.USER_UNIT];
  const time = Number(entry.__REALTIME_TIMESTAMP) / 1000;
  const message = typeof entry.MESSAGE === 'string' ? entry.MESSAGE :
    Array.isArray(entry.MESSAGE) && entry.MESSAGE.length <= 4 * 1024 * 1024 &&
    entry.MESSAGE.every(byte => Number.isInteger(byte) && byte >= 0 && byte <= 255) ? Buffer.from(entry.MESSAGE).toString('utf8') : null;
  if (!source || !Number.isFinite(time) || time <= 0 || message === null) return null;
  const record = { timestamp: new Date(time).toISOString(), source,
    journal_id: createHash('sha256').update(String(entry.__CURSOR)).digest('hex') };
  if (source !== 'agent') {
    let data; try { data = JSON.parse(message); } catch { return null; }
    if (!data || !values.event.includes(data.event)) return null;
    for (const [key, allowed] of Object.entries(values)) if (allowed.includes(data[key])) record[key] = data[key];
    for (const key of ['elapsed_ms', 'characters', 'audio_bytes', 'http_status', 'upstream_status']) {
      if (typeof data[key] === 'number' && Number.isFinite(data[key]) && data[key] >= 0) record[key] = data[key];
    }
    if (/^[a-f0-9-]{36}$/i.test(data.request_id || '')) record.request_id = data.request_id;
    return record;
  }
  const text = message.replace(/\x1b\[[0-9;]*m/g, '');
  const level = text.match(/\b(ERROR|WARN|INFO)\b/)?.[1];
  const api = text.match(/action=(GET|POST|DELETE|PATCH) (\/api\/(?:realtime\/)?session)(?:\/([a-f0-9-]{36}))?/i);
  if (!api && !['ERROR', 'WARN'].includes(level)) return null;
  Object.assign(record, { event: api ? 'agent_api' : 'agent_diagnostic', level: level || 'INFO' });
  if (api) { record.method = api[1]; record.route = api[2]; if (api[3]) record.session_id = api[3]; }
  const outcome = text.match(/outcome=(Success|Failure|Denied)/)?.[1];
  if (outcome) record.outcome = outcome;
  record.category = /timeout|timed out/i.test(text) ? 'timeout' : /429|rate.limit/i.test(text) ? 'rate_limit' :
    /401|403|unauthorized|authentication/i.test(text) ? 'authentication' : /disconnect|connection|socket/i.test(text) ? 'transport' : 'other';
  const module = text.match(/\b(codetether_[a-z_]+(?:::[a-z_]+)+)\b/)?.[1];
  if (module) record.module = module;
  return record;
}