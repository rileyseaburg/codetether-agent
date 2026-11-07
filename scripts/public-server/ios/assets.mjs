// Explicit allowlist: no directory listing, traversal, app credentials, or API proxy.
import { createReadStream } from 'node:fs';
import { stat } from 'node:fs/promises';
import { join } from 'node:path';
import { byteRange } from './byte-range.mjs';
const types = { 'index.html': 'text/html; charset=utf-8', 'manifest.plist': 'application/xml',
  'CodeTether.ipa': 'application/octet-stream', 'release.json': 'application/json',
  'SHA256SUMS': 'text/plain; charset=utf-8' };
/**
 * Serves only allowlisted release assets, with GET byte ranges and HEAD metadata.
 * @param {string} root Directory holding versioned releases.
 * @param {string} latest Release selected only for the homepage.
 * @returns {import('node:http').RequestListener} Streaming HTTP request handler.
 * @example
 * import { createServer } from 'node:http';
 * import { assetHandler } from './assets.mjs';
 * createServer(assetHandler('/srv/releases', '1.5.1-25')).listen(4098);
 */
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
      const headers = { 'Content-Type': types[file], 'Accept-Ranges': 'bytes',
        'Content-Length': info.size };
      let range;
      try {
        range = byteRange(req.method === 'GET' && !req.headers['if-range']
          ? req.headers.range : undefined, info.size);
      } catch {
        res.writeHead(416, { 'Content-Range': `bytes */${info.size}` });
        res.end(); return;
      }
      if (range) {
        headers['Content-Range'] = `bytes ${range.start}-${range.end}/${info.size}`;
        headers['Content-Length'] = range.end - range.start + 1;
      }
      res.writeHead(range ? 206 : 200, headers);
      if (req.method === 'HEAD') { res.end(); return; }
      const stream = createReadStream(path, range ?? undefined);
      stream.on('error', () => res.destroy()); res.on('close', () => stream.destroy());
      stream.pipe(res);
    } catch { res.writeHead(404); res.end(); }
  };
}
