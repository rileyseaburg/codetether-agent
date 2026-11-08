// Credentials are supplied at execution time and are never persisted.
import { cancelled } from './lock.mjs';
/** @param {string} value */
export function endpoint(value = 'https://server.codetether.run') {
  const url = new URL(value);
  if (url.username || url.password || url.search || url.hash || url.pathname !== '/') throw new Error('Expected a plain API origin');
  const local = ['127.0.0.1', 'localhost', '[::1]'].includes(url.hostname);
  if (url.protocol !== 'https:' && !(url.protocol === 'http:' && local)) throw new Error('API must use HTTPS or loopback HTTP');
  return url.origin;
}
/** @param {string} origin @param {string} token @param {object} body @param {string} dir @param {number} deadline */
export async function complete(origin, token, body, dir, deadline) {
  if (typeof token !== 'string' || !token || /[\r\n]/.test(token)) throw new Error('CODETETHER_AUTH_TOKEN is required');
  const controller = new AbortController();
  const remaining = Math.min(720000, deadline - Date.now());
  if (remaining <= 0) throw new Error('Job deadline exceeded');
  const timer = setTimeout(() => controller.abort(), remaining);
  const poll = setInterval(() => { cancelled(dir).then(value => { if (value) controller.abort(); }).catch(() => controller.abort()); }, 100);
  const interrupt = () => controller.abort();
  process.once('SIGINT', interrupt); process.once('SIGTERM', interrupt);
  try {
    if (await cancelled(dir)) throw new Error('Job cancelled');
    const response = await fetch(`${endpoint(origin)}/v1/chat/completions`, {
      method: 'POST', redirect: 'error', signal: controller.signal,
      headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' }, body: JSON.stringify(body)
    });
    if (!response.ok) { await response.body?.cancel(); throw new Error(`Chat API HTTP ${response.status}`); }
    const chunks = []; let bytes = 0;
    for await (const chunk of response.body) {
      bytes += chunk.length;
      if (bytes > 512000) { controller.abort(); throw new Error('Chat response too large'); }
      chunks.push(chunk);
    }
    return JSON.parse(Buffer.concat(chunks).toString('utf8'));
  } finally {
    clearTimeout(timer); clearInterval(poll);
    process.removeListener('SIGINT', interrupt); process.removeListener('SIGTERM', interrupt);
  }
}
