import { createServer } from 'node:http';
import type { Server } from 'node:http';
import { asset } from './assets.ts';
import { digest, requireOrigin } from './auth.ts';
import { stop } from './events.ts';
import { json } from './json.ts';
import { Registry } from './registry.ts';
import { route } from './routes.ts';
import { HTTPError } from './types.ts';
import type { Analyze } from './types.ts';

/** Bind explicitly to loopback in the entry point; expose only the companion prefix. */
export function companionServer(token: string, analyze: Analyze, origin = 'https://server.codetether.run'): Server {
  if (token.length < 32) throw new Error('A persistent API credential is required');
  const registry = new Registry();
  const services = { registry, analyze, ownerHash: digest(token) };
  const server = createServer(async (request, response): Promise<void> => {
    try {
      requireOrigin(request, origin);
      const url = new URL(request.url ?? '/', origin);
      if (request.method === 'GET' && url.pathname === '/companion') {
        response.writeHead(308, { Location: '/companion/' }); response.end(); return;
      }
      if (request.method === 'GET' && await asset(url.pathname, response)) return;
      await route(request, response, url.pathname, services);
    } catch (error) {
      if (response.headersSent) { response.destroy(); return; }
      json(response, error instanceof HTTPError ? error.status : 500,
        { error: error instanceof HTTPError ? error.message : 'Screen service unavailable' });
    }
  });
  const sweep = setInterval(() => registry.sweep(), 10000).unref();
  server.on('close', () => { clearInterval(sweep); for (const session of registry.sessions.values()) stop(session); });
  server.requestTimeout = 15000; server.headersTimeout = 10000; return server;
}