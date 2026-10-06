import test from 'node:test';
import assert from 'node:assert/strict';
import http from 'node:http';
import { mkdtemp, mkdir, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { assetHandler } from './assets.mjs';
test('mocked local: immutable releases coexist and unlisted routes fail closed', async () => {
  const root = await mkdtemp(join(tmpdir(), 'codetether-ios-assets-'));
  for (const release of ['1.4.0-8', '1.5.0-9']) {
    await mkdir(join(root, release));
    await writeFile(join(root, release, 'index.html'), release);
    await writeFile(join(root, release, 'CodeTether.ipa'), release);
    await writeFile(join(root, release, 'credentials.json'), 'not-public');
  }
  const server = http.createServer(assetHandler(root, '1.5.0-9'));
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  console.log(`Mocked local fixture artifact: ${root}`);
  try {
    for (const release of ['1.4.0-8', '1.5.0-9']) {
      const response = await fetch(`${base}/releases/${release}/CodeTether.ipa`);
      assert.equal(response.status, 200);
      assert.equal(await response.text(), release);
    }
    assert.equal(await (await fetch(base)).text(), '1.5.0-9');
    const head = await fetch(`${base}/releases/1.5.0-9/CodeTether.ipa`, { method: 'HEAD' });
    assert.equal(head.status, 200);
    assert.equal(await head.text(), '');
    for (const path of ['/releases/1.5.0-9/credentials.json', '/releases/1.5.0-9/',
      '/releases/1.5.0-9/../credentials.json', '/v1/agent/chat']) {
      const response = await fetch(base + path);
      assert.equal(response.status, 404);
    }
    const post = await fetch(`${base}/releases/1.5.0-9/CodeTether.ipa`, { method: 'POST' });
    assert.equal(post.status, 405);
    assert.equal(post.headers.get('allow'), 'GET, HEAD');
  } finally {
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
});

