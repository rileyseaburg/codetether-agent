import { randomUUID } from 'node:crypto';
const paths = new Set(['/tts/speak', '/tts/voices', '/tts/health', '/mobile/attachments', '/mobile/image']);
const errors = new Set(['AbortError', 'TimeoutError', 'TypeError', 'HTTPError']);
/** Metadata only: never serialize requests, headers, bodies, or error messages. */
export function requestTrace(request, response, emit = record => console.log(JSON.stringify(record))) {
  const started = performance.now();
  let path;
  try { path = new URL(request.url, 'http://localhost').pathname; } catch { path = 'unknown'; }
  const state = { request_id: randomUUID(), route: paths.has(path) ? path : 'unknown',
    method: ['GET', 'POST'].includes(request.method) ? request.method : 'other', stage: 'route' };
  let ended = false;
  response.setHeader('X-Request-ID', state.request_id);
  function record(status) {
    try { emit({ event: 'voice_bridge', timestamp: new Date().toISOString(), ...state,
      status, elapsed_ms: Math.round(performance.now() - started) }); } catch { /* logging must not break audio */ }
  }
  function finish(status) {
    if (ended) return;
    ended = true; state.http_status = response.statusCode; record(status);
  }
  response.once('finish', () => finish(response.statusCode >= 400 ? 'failed' : 'completed'));
  response.once('close', () => { if (!response.writableFinished) finish('client_disconnected'); });
  record('started');
  return {
    mark(stage, details = {}) {
      state.stage = stage;
      for (const key of ['characters', 'upstream_status']) if (Number.isFinite(details[key])) state[key] = details[key];
      record('progress');
    },
    fail(error) {
      state.error_type = errors.has(error?.name) ? error.name : 'Error'; record('error');
    }
  };
}