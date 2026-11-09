// Ref identity is verified independently of release metadata.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { githubRepo, sourceRepo } from './source.mjs';
/** @param {string} repo @param {string} tag @returns {string|undefined} */
function resolve(repo, tag) {
  const rows = execFileSync('git', ['ls-remote', repo, `refs/tags/${tag}`, `refs/tags/${tag}^{}`], { encoding: 'utf8' })
    .trim().split('\n').filter(Boolean).map(line => line.split(/\s+/));
  return (rows.find(row => row[1].endsWith('^{}')) || rows[0])?.[0];
}
/** @param {string} tag @returns {string} */
export function matchingCommit(tag) {
  const source = resolve(`${sourceRepo}.git`, tag); assert(source, 'Forgejo tag is missing');
  const github = resolve(`https://github.com/${githubRepo}.git`, tag);
  assert.equal(github, source, 'Code/tag mirror must catch up before release publication');
  return source;
}