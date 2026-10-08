import { readFile } from 'node:fs/promises';
import type { ServerResponse } from 'node:http';
import { HTTPError } from './types.ts';

const files = new Set(['index.html', 'style.css', 'app.js', 'manifest.webmanifest', 'icon.svg', 'sw.js',
  'session.js', 'flow.js', 'pump.js', 'api.js', 'capture.js', 'halt.js', 'models.js', 'pairing.js', 'sharing.js', 'render.js']);
const types: Record<string, string> = { html: 'text/html; charset=utf-8', js: 'text/javascript', css: 'text/css',
  webmanifest: 'application/manifest+json', svg: 'image/svg+xml' };
/** Serve a fixed shell allowlist, never source, state, tokens, or a request-selected path. */
export async function asset(path: string, response: ServerResponse): Promise<boolean> {
  const file = path === '/companion/' ? 'index.html' : path.slice('/companion/'.length);
  if (!path.startsWith('/companion/') || !files.has(file)) return false;
  const bytes = await readFile(new URL(`./web/${file}`, import.meta.url)).catch(() => {
    throw new HTTPError(404, 'Asset not found');
  });
  response.writeHead(200, {
    'Content-Type': types[file.split('.').pop() ?? ''] ?? 'application/octet-stream',
    'Cache-Control': 'no-cache', 'X-Content-Type-Options': 'nosniff', 'Referrer-Policy': 'no-referrer',
    'Cross-Origin-Resource-Policy': 'same-origin', 'Permissions-Policy': 'camera=(), microphone=(), display-capture=(self)',
    'Content-Security-Policy': "default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; worker-src 'self'; manifest-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
    'Service-Worker-Allowed': '/companion/'
  });
  response.end(bytes); return true;
}