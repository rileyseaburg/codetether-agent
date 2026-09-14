// GitHub operations use pre-provisioned gh authentication, never copied credentials.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { githubRepo } from './source.mjs';
/** @typedef {{id:number,name:string,size:number,digest:string,browser_download_url:string}} GitHubAsset */
/** @typedef {{id:number,tag_name:string,name:string,body:string,draft:boolean,prerelease:boolean,html_url:string,assets:GitHubAsset[]}} GitHubRelease */
/** @param {string[]} args @param {string|undefined} [input] @returns {string} */
function gh(args, input) { return execFileSync('gh', args, { encoding: 'utf8', input, maxBuffer: 32 * 1024 * 1024 }); }
/** @param {string} tag @returns {GitHubRelease|undefined} */
export function findRelease(tag) {
  const pages = JSON.parse(gh(['api', `repos/${githubRepo}/releases`, '--paginate', '--slurp']));
  return pages.flat().find(release => release.tag_name === tag);
}
/** @param {number} id @returns {GitHubRelease} */
export function getRelease(id) { return JSON.parse(gh(['api', `repos/${githubRepo}/releases/${id}`])); }
/** @param {import('./source.mjs').SourceRelease} release @param {string} sha @returns {GitHubRelease} */
export function createDraft(release, sha) {
  const payload = { tag_name: release.tag_name, target_commitish: sha, name: release.name || release.tag_name,
    body: release.body || '', draft: true, prerelease: release.prerelease };
  return JSON.parse(gh(['api', '--method', 'POST', `repos/${githubRepo}/releases`, '--input', '-'], JSON.stringify(payload)));
}
/** @param {string} tag @param {string} file @returns {void} */
export function upload(tag, file) { gh(['release', 'upload', tag, file, '--repo', githubRepo]); }
/** @param {GitHubRelease} target @param {import('./source.mjs').SourceRelease} source @returns {GitHubRelease} */
export function publish(target, source) {
  assert(target.assets.length >= source.assets.length, 'Not all source assets uploaded');
  const data = { name: source.name || source.tag_name, body: source.body || '', draft: false, prerelease: source.prerelease };
  if (!target.draft && target.name === data.name && target.body === data.body && target.prerelease === data.prerelease) return target;
  return JSON.parse(gh(['api', '--method', 'PATCH', `repos/${githubRepo}/releases/${target.id}`, '--input', '-'], JSON.stringify(data)));
}
