import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

export async function probe(spec, token, directory) {
  const headers = { 'Content-Type': 'application/json' };
  if (spec.auth) headers.Authorization = `Bearer ${token}`;
  try {
    const response = await fetch(spec.url, {
      method: spec.body ? 'POST' : 'GET', headers, redirect: 'error',
      body: spec.body ? JSON.stringify(spec.body) : undefined,
      signal: AbortSignal.timeout(180000)
    });
    const data = Buffer.from(await response.arrayBuffer());
    const audio = response.headers.get('content-type')?.includes('audio/');
    const text = audio ? '' : data.toString('utf8').replaceAll(token, '[REDACTED]');
    const file = join(directory, `${spec.name}.${audio ? 'wav' : 'json'}`);
    writeFileSync(file, audio ? data : text, { mode: 0o600 });
    let payload;
    try { payload = JSON.parse(text); } catch { payload = null; }
    const extra = spec.audio ? data.subarray(0, 4).toString() === 'RIFF' && data.length > 44 : true;
    const reply = payload?.choices?.[0]?.message?.content;
    const contentOK = spec.reply ? reply?.trim().toLowerCase() === spec.reply : true;
    return { name: spec.name, url: spec.url, model: spec.body?.model, status: response.status,
      expectedStatus: spec.expected, ok: response.status === spec.expected && extra && contentOK,
      bytes: data.length, contentType: response.headers.get('content-type'),
      reply, error: payload?.error, artifact: file };
  } catch (error) {
    return { name: spec.name, url: spec.url, ok: false, error: error.message.replaceAll(token, '[REDACTED]') };
  }
}
