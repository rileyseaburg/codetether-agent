import { Readable } from 'node:stream';
import { pipeline } from 'node:stream/promises';
import { authorize, HTTPError } from './auth.mjs';
import { speechBody } from './body.mjs';
import { serveMedia } from './media.mjs';
import { requestTrace } from './trace.mjs';

const routes = new Map([['POST /tts/speak', '/tts/speak'], ['GET /tts/voices', '/voices'], ['GET /tts/health', '/health']]);
export function createHandler(authOrigin, speechOrigin, emit) {
  return async (request, response) => {
    const trace = requestTrace(request, response, emit);
    const controller = new AbortController();
    response.on('close', () => { if (!response.writableEnded) controller.abort(); });
    try {
      const path = new URL(request.url, 'http://localhost').pathname;
      const upstream = routes.get(`${request.method} ${path}`);
      const media = (path === '/mobile/attachments' && request.method === 'POST') || (path === '/mobile/image' && request.method === 'GET');
      if (!upstream && !media) throw new HTTPError(404, 'Unknown route');
      trace.mark('authorization');
      await authorize(request.headers.authorization, authOrigin,
        AbortSignal.any([controller.signal, AbortSignal.timeout(5000)]));
      trace.mark('body');
      if (media) { await serveMedia(request, response); return; }
      const body = request.method === 'POST' ? await speechBody(request) : undefined;
      trace.mark('synthesis', { characters: body ? JSON.parse(body).script.length : 0 });
      const result = await fetch(`${speechOrigin}${upstream}`, {
        method: request.method, body, headers: { 'Content-Type': 'application/json' },
        redirect: 'error', signal: AbortSignal.any([controller.signal, AbortSignal.timeout(180000)])
      });
      trace.mark('upstream_response', { upstream_status: result.status });
      if (!result.ok) {
        await result.body?.cancel();
        throw new HTTPError(result.status >= 500 ? 502 : result.status, 'Kokoro rejected the speech request');
      }
      const type = result.headers.get('content-type') || '';
      if (body && !type.startsWith('audio/wav')) {
        await result.body?.cancel();
        throw new HTTPError(502, 'Kokoro returned unexpected audio');
      }
      response.writeHead(200, { 'Content-Type': type, 'Cache-Control': 'no-store', 'X-Content-Type-Options': 'nosniff' });
      await pipeline(Readable.fromWeb(result.body), response);
    } catch (error) {
      trace.fail(error);
      if (response.destroyed) return;
      if (response.headersSent) { response.destroy(); return; }
      response.writeHead(error instanceof HTTPError ? error.status : 502,
        { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' });
      response.end(JSON.stringify({ error: error instanceof HTTPError ? error.message : 'Audio service unavailable' }));
    }
  };
}