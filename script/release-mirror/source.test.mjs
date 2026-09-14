// Mocked-local source completeness and immutable asset-conflict checks.
import test from 'node:test';
import assert from 'node:assert/strict';
import { checksums, digest } from './source.mjs';
import { mirrorAssets } from './asset.mjs';
test('rejects partial source uploads and duplicate checksum names', async () => {
  const original = globalThis.fetch;
  const sha = '0'.repeat(64);
  const release = { tag_name: 'v1.0.0', assets: [{ name: 'SHA256SUMS-v1.0.0.txt' }, { name: 'app.exe' }] };
  try {
    globalThis.fetch = async () => new Response(`${sha}  app.exe\n`);
    const sums = await checksums(release);
    assert.equal(sums.get('app.exe'), sha);
    assert.equal(sums.get('SHA256SUMS-v1.0.0.txt'), digest(Buffer.from(`${sha}  app.exe\n`)));
    await assert.rejects(checksums({ ...release, assets: release.assets.slice(0, 1) }), /incomplete/);
    globalThis.fetch = async () => new Response(`${sha}  app.exe\n${sha}  app.exe\n`);
    await assert.rejects(checksums(release), /Duplicate/);
  } finally { globalThis.fetch = original; }
});
test('does not overwrite same-name GitHub assets with a different digest', async () => {
  const sha = '0'.repeat(64);
  const source = { tag_name: 'v1.0.0', assets: [{ name: 'app.exe', size: 1 }] };
  const target = { assets: [{ name: 'app.exe', size: 1, digest: `sha256:${'1'.repeat(64)}` }] };
  await assert.rejects(mirrorAssets(source, target, new Map([['app.exe', sha]]), '/unused'), /Refusing conflicting/);
  target.assets[0].digest = `sha256:${sha}`;
  assert.equal(await mirrorAssets(source, target, new Map([['app.exe', sha]]), '/unused'), target);
});
