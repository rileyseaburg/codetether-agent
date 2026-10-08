import type { IncomingMessage, ServerResponse } from 'node:http';
import { requireToken } from './auth.ts';
import { captureInput } from './capture-input.ts';
import { capture, pause } from './capture.ts';
import { commandSnapshot, requestCapture } from './commands.ts';
import { stop, subscribe } from './events.ts';
import { json, readJSON } from './json.ts';
import { Registry } from './registry.ts';
import { pairCode, sessionInput } from './session-input.ts';
import { HTTPError } from './types.ts';
import type { Analyze } from './types.ts';

export interface Services { registry: Registry; ownerHash: string; analyze: Analyze }
/** Route only the companion API; device capabilities cannot access owner operations. */
export async function route(request: IncomingMessage, response: ServerResponse, path: string, services: Services): Promise<void> {
  const { registry, ownerHash, analyze } = services;
  if (path === '/companion/sessions' && request.method === 'POST') {
    requireToken(request, ownerHash);
    json(response, 200, registry.create(sessionInput(await readJSON(request)))); return;
  }
  if (path === '/companion/pair' && request.method === 'POST') {
    json(response, 200, registry.pair(pairCode(await readJSON(request)))); return;
  }
  const match = /^\/companion\/sessions\/([a-f0-9-]{36})(?:\/(events|frames|pause|commands|request))?$/.exec(path);
  if (!match) throw new HTTPError(404, 'Route not found');
  const [, id, action] = match;
  const deviceRoute = (request.method === 'POST' && (action === 'frames' || action === 'pause'))
    || (request.method === 'GET' && action === 'commands');
  if (!deviceRoute) requireToken(request, ownerHash);
  const session = registry.get(id);
  if (deviceRoute) requireToken(request, session.deviceHash ?? '');
  if (request.method === 'GET' && action === 'events') { subscribe(session, response); return; }
  if (request.method === 'GET' && action === 'commands') { json(response, 200, commandSnapshot(session)); return; }
  if (request.method === 'POST' && action === 'request') {
    json(response, 202, requestCapture(session, await readJSON(request))); return;
  }
  if (request.method === 'DELETE' && !action) { stop(session); json(response, 200, { stopped: true }); return; }
  if (deviceRoute && action === 'pause') { pause(session); json(response, 200, { paused: true }); return; }
  if (deviceRoute && action === 'frames') {
    const epoch = session.seq;
    const frame = captureInput(await readJSON(request, 710000));
    if (session.seq !== epoch) throw new HTTPError(409, 'Session changed while uploading');
    capture(session, frame, analyze);
    json(response, 202, { accepted: true }); return;
  }
  throw new HTTPError(404, 'Route not found');
}