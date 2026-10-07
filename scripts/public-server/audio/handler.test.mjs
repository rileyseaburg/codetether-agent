import test from 'node:test';
import assert from 'node:assert/strict';
import { unlink } from 'node:fs/promises';
import { fixture } from './test-support.mjs';

test('missing and invalid bearer never reach speech service', async t => {
  const { url, calls } = await fixture(t);
  for (const headers of [{}, { Authorization: 'Bearer invalid' }]) {
    assert.equal((await fetch(`${url}/tts/voices`, { headers })).status, 401);
  }
  assert.equal(calls(), 0);
});
test('media rejects unauthenticated uploads, invalid images and arbitrary file reads', async t => {
  const { url } = await fixture(t);
  assert.equal((await fetch(`${url}/mobile/attachments`, { method: 'POST', body: '{}' })).status, 401);
  assert.equal((await fetch(`${url}/mobile/attachments`, {
    method: 'POST', headers: { Authorization: 'Bearer fixture' },
    body: JSON.stringify({ data: Buffer.from('not an image').toString('base64') })
  })).status, 400);
  assert.equal((await fetch(`${url}/mobile/image?path=${encodeURIComponent('/etc/passwd')}`, {
    headers: { Authorization: 'Bearer fixture' }
  })).status, 404);
});
test('media accepts PDFs under 12 MiB and rejects larger ones', async t => {
  const { url } = await fixture(t);
  const headers = { Authorization: 'Bearer fixture' };
  const small = await fetch(`${url}/mobile/attachments`, { method: 'POST', headers,
    body: JSON.stringify({ data: Buffer.from('%PDF-1.4 small').toString('base64') }) });
  assert.equal(small.status, 200);
  const { path } = await small.json();
  assert.match(path, /\.pdf$/);
  t.after(() => unlink(path).catch(() => {}));
  const big = Buffer.concat([Buffer.from('%PDF-1.4'), Buffer.alloc(13 * 1024 * 1024)]);
  const oversized = await fetch(`${url}/mobile/attachments`, { method: 'POST', headers,
    body: JSON.stringify({ data: big.toString('base64') }) });
  assert.equal(oversized.status, 400);
});
test('authenticated WAV streams without forwarding credentials to Kokoro', async t => {
  const { url } = await fixture(t);
  const response = await fetch(`${url}/tts/speak`, { method: 'POST',
    headers: { Authorization: 'Bearer fixture' }, body: JSON.stringify({ script: 'Hello', voice_id: 'af_heart' }) });
  assert.equal(response.status, 200);
  assert.equal(response.headers.get('content-type'), 'audio/wav');
  assert.equal(response.headers.get('cache-control'), 'no-store');
  assert.equal(await response.text(), 'RIFFfixture-wave');
});
test('invalid text and oversized JSON fail before synthesis', async t => {
  const { url, calls } = await fixture(t);
  for (const [body, status] of [['{}', 400], [JSON.stringify({ script: 'a'.repeat(401) }), 400], ['a'.repeat(5000), 413]]) {
    const response = await fetch(`${url}/tts/speak`, { method: 'POST', headers: { Authorization: 'Bearer fixture' }, body });
    assert.equal(response.status, status);
  }
  assert.equal(calls(), 0);
});
test('unknown paths cannot select arbitrary upstreams', async t => {
  const { url, calls } = await fixture(t);
  assert.equal((await fetch(`${url}/other`, { headers: { Authorization: 'Bearer fixture' } })).status, 404);
  assert.equal(calls(), 0);
});
