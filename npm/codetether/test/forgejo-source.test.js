// Mocked-local metadata checks against the shipping installer functions.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { createRequire } = require('node:module');
test('uses Forgejo prereleases, explicit tags, and Forgejo repository default', async () => {
  const file = path.resolve(__dirname, '../lib/installer.js');
  const localRequire = createRequire(file);
  const calls = [];
  const context = { module: { exports: {} }, __dirname: path.dirname(file), process: { env: {} },
    require(name) {
      if (name !== './http') return localRequire(name);
      return { ...localRequire(name), async requestJson(url) {
        calls.push(url);
        return url.includes('?') ? [{ tag_name: 'v4.7.6-dev.5', prerelease: true }]
          : { assets: [{ name: 'windows-gnu.zip' }] };
      } };
    },
  };
  vm.runInNewContext(fs.readFileSync(file, 'utf8') +
    '\nmodule.exports = { repoFromEnv, getLatestReleaseTag, getReleaseAssetNames };', context);
  const api = context.module.exports;
  assert.equal(api.repoFromEnv(), 'riley/codetether-agent');
  assert.equal(await api.getLatestReleaseTag(api.repoFromEnv()), 'v4.7.6-dev.5');
  assert.equal((await api.getReleaseAssetNames(api.repoFromEnv(), 'v4.7.6-dev.5')).join(','), 'windows-gnu.zip');
  assert.deepEqual(calls, [
    'https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent/releases?draft=false&limit=1',
    'https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent/releases/tags/v4.7.6-dev.5',
  ]);
});