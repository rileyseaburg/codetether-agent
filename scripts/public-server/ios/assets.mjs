// Explicit allowlist: no directory listing, traversal, app credentials, or API proxy.
import { createReadStream } from 'node:fs';
import { stat } from 'node:fs/promises';
import { join } from 'node:path';
const types = { 'index.html': 'text/html; charset=utf-8', 'manifest.plist': 'application/xml',
  'CodeTether.ipa': 'application/octet-stream', 'release.json': 'application/json',
  'SHA256SUMS': 'text/plain; charset=utf-8' };
export function assetHandler(root, latest) {
  return async (req, res) => {
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('Cache-Control', 'no-store');
    res.setHeader('Content-Security-Policy', "default-src 'none'; base-uri 'none'; frame-ancestors 'none'");
    if (!['GET', 'HEAD'].includes(req.method)) {
      res.setHeader('Allow', 'GET, HEAD'); res.writeHead(405); res.end(); return;
    }
    const match = req.url === '/' ? [null, latest, 'index.html'] :
      req.url?.match(/^\/releases\/(\d+\.\d+\.\d+-[1-9]\d*)\/([^/?]+)$/);
    const [, release, file] = match ?? [];
    if (!Object.hasOwn(types, file ?? '')) { res.writeHead(404); res.end(); return; }
    try {
      const path = join(root, release, file), info = await stat(path);
      res.writeHead(200, { 'Content-Type': types[file], 'Content-Length': info.size });
      if (req.method === 'HEAD') { res.end(); return; }
      const stream = createReadStream(path);
      stream.on('error', () => res.destroy()); res.on('close', () => stream.destroy());
      stream.pipe(res);
    } catch { res.writeHead(404); res.end(); }
  };
}