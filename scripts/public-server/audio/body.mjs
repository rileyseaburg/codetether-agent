import { HTTPError } from './auth.mjs';

export async function speechBody(request) {
  let size = 0;
  const chunks = [];
  for await (const chunk of request) {
    size += chunk.length;
    if (size > 4096) throw new HTTPError(413, 'Speech request is too large');
    chunks.push(chunk);
  }
  let body;
  try { body = JSON.parse(Buffer.concat(chunks).toString('utf8')); }
  catch { throw new HTTPError(400, 'Invalid JSON'); }
  if (!body || typeof body.script !== 'string' || !body.script.trim() || body.script.length > 400) {
    throw new HTTPError(400, 'Speech text must be 1–400 characters');
  }
  if (body.voice_id !== undefined && (typeof body.voice_id !== 'string' || !/^[A-Za-z0-9_-]{1,64}$/.test(body.voice_id))) {
    throw new HTTPError(400, 'Invalid voice identifier');
  }
  return JSON.stringify({ script: body.script, voice_id: body.voice_id });
}
