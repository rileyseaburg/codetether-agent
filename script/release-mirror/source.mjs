// Public Forgejo release metadata and checksum verification. No credentials used.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
export const sourceRepo = 'https://forgejo.quantum-forge.io/riley/codetether-agent';
/** @typedef {{id:number,name:string,size:number,browser_download_url:string}} SourceAsset */
/** @typedef {{id:number,tag_name:string,name:string,body:string,draft:boolean,prerelease:boolean,assets:SourceAsset[]}} SourceRelease */
export const githubRepo = 'rileyseaburg/codetether-agent';
/** @param {string} url @returns {Promise<Buffer>} */
export async function download(url) {
  const response = await fetch(url, { signal: AbortSignal.timeout(300000) });
  assert.equal(response.status, 200, `Download failed: ${url}`);
  return Buffer.from(await response.arrayBuffer());
}
/** @param {Buffer} bytes @returns {string} */
export function digest(bytes) { return createHash('sha256').update(bytes).digest('hex'); }
/** @param {string | undefined} tag @returns {Promise<SourceRelease>} */
export async function sourceRelease(tag) {
  const api = 'https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent/releases';
  const endpoint = tag ? `/tags/${encodeURIComponent(tag)}` : '?draft=false&limit=1';
  const result = JSON.parse((await download(api + endpoint)).toString());
  const release = tag ? result : result[0];
  assert(release && !release.draft && /^v?[0-9A-Za-z._-]+$/.test(release.tag_name));
  return release;
}
/** @param {SourceRelease} release @returns {Promise<Map<string, string>>} */
export async function checksums(release) {
  const name = `SHA256SUMS-${release.tag_name}.txt`;
  const bytes = await download(`${sourceRepo}/releases/download/${release.tag_name}/${name}`);
  const result = new Map([[name, digest(bytes)]]);
  for (const line of bytes.toString().trim().split(/\r?\n/)) {
    const match = line.match(/^([a-f0-9]{64}) [ *]([A-Za-z0-9._-]+)$/i);
    assert(match, 'Invalid checksum manifest');
    assert(!result.has(match[2]), 'Duplicate checksum entry');
    result.set(match[2], match[1].toLowerCase());
  }
  assert.equal(release.assets.length, result.size, 'Source release asset upload is incomplete');
  for (const asset of release.assets) assert(result.has(asset.name), `Unmanifested source asset: ${asset.name}`);
  return result;
}