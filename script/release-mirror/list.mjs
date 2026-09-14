// Enumerate published source releases so fast consecutive releases are not skipped.
import assert from 'node:assert/strict';
import { download } from './source.mjs';
/** @returns {Promise<import('./source.mjs').SourceRelease[]>} */
export async function publishedReleases() {
  /** @type {import('./source.mjs').SourceRelease[]} */
  const releases = [];
  for (let page = 1; ; page++) {
    const api = `https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent/releases?draft=false&limit=50&page=${page}`;
    const entries = JSON.parse((await download(api)).toString());
    assert(Array.isArray(entries), 'Invalid release listing');
    for (const entry of entries) {
      assert(!entry.draft && /^v?[0-9A-Za-z._-]+$/.test(entry.tag_name), 'Invalid published release');
      releases.push(entry);
    }
    if (entries.length < 50) return releases;
  }
}