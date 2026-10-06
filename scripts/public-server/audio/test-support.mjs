import { createServer } from 'node:http';
import { once } from 'node:events';
import { createHandler } from './handler.mjs';
import assert from 'node:assert/strict';

async function listen(handler) {
  const server = createServer(handler);
  server.listen(0, '127.0.0.1');
  await once(server, 'listening');
  return { server, url: `http://127.0.0.1:${server.address().port}` };
}
export async function fixture(context) {
  const auth = await listen((req, res) => { res.writeHead(req.headers.authorization === 'Bearer fixture' ? 200 : 401); res.end(); });
  let calls = 0;
  const speech = await listen((req, res) => {
    assert.equal(req.headers.authorization, undefined);
    calls++;
    res.writeHead(200, { 'Content-Type': req.method === 'POST' ? 'audio/wav' : 'application/json' });
    res.end(req.method === 'POST' ? Buffer.from('RIFFfixture-wave') : '{"voices":["af_heart"]}');
  });
  const bridge = await listen(createHandler(auth.url, speech.url));
  context.after(() => { for (const { server } of [auth, speech, bridge]) { server.closeAllConnections(); server.close(); } });
  return { url: bridge.url, calls: () => calls };
}
