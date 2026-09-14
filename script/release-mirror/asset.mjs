// Upload missing assets only. A same-name digest conflict is never overwritten.
import assert from 'node:assert/strict';
import { writeFileSync } from 'node:fs';
import path from 'node:path';
import { download, digest, sourceRepo } from './source.mjs';
import { upload, getRelease } from './github.mjs';
/** @param {import('./source.mjs').SourceRelease} source @param {import('./github.mjs').GitHubRelease} target @param {Map<string,string>} sums @param {string} evidence @returns {Promise<import('./github.mjs').GitHubRelease>} */
export async function mirrorAssets(source, target, sums, evidence) {
  for (const asset of source.assets) {
    assert(/^[A-Za-z0-9._-]+$/.test(asset.name), 'Unsafe asset filename');
    const expected = sums.get(asset.name); assert(expected, `Missing checksum: ${asset.name}`);
    let remote = target.assets.find(item => item.name === asset.name);
    if (!remote) {
      const url = `${sourceRepo}/releases/download/${source.tag_name}/${asset.name}`;
      assert.equal(asset.browser_download_url, url, 'Unexpected asset origin');
      const bytes = await download(url);
      assert.equal(digest(bytes), expected, `Source checksum mismatch: ${asset.name}`);
      const file = path.join(evidence, asset.name); writeFileSync(file, bytes);
      upload(source.tag_name, file);
      target = getRelease(target.id); remote = target.assets.find(item => item.name === asset.name);
    }
    assert(remote, `Upload missing: ${asset.name}`);
    assert.equal(remote.size, asset.size, `Size mismatch: ${asset.name}`);
    assert.equal(remote.digest, `sha256:${expected}`, `Refusing conflicting GitHub asset: ${asset.name}`);
  }
  return target;
}