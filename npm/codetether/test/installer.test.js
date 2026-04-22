const test = require('node:test');
const assert = require('node:assert/strict');

const {
  configuredPkgTag,
  defaultCacheDirFor,
  selectAssetCandidates,
} = require('../lib/installer');

test('prefers explicit release tags over npm package versions', () => {
  assert.equal(
    configuredPkgTag({
      version: '1.1.6-alpha-7.2',
      codetetherReleaseTag: 'v4.6.0',
    }),
    'v4.6.0'
  );
});

test('uses the standard macOS cache directory', () => {
  assert.equal(
    defaultCacheDirFor({
      platform: 'darwin',
      env: {},
      homedir: '/Users/tester',
    }),
    '/Users/tester/Library/Caches/codetether-npx'
  );
});

test('prefers published Windows GNU fallback assets when MSVC assets are missing', () => {
  const candidates = selectAssetCandidates({
    tag: 'v4.4.1',
    targetTriple: 'x86_64-pc-windows-msvc',
    isWindowsTarget: true,
    availableAssetNames: [
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
      'SHA256SUMS-v4.4.1.txt',
    ],
  });

  assert.deepEqual(
    candidates.map((candidate) => candidate.name),
    [
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
    ]
  );
});

test('keeps preferred Windows asset order when release assets cannot be enumerated', () => {
  const candidates = selectAssetCandidates({
    tag: 'v4.4.1',
    targetTriple: 'x86_64-pc-windows-msvc',
    isWindowsTarget: true,
    availableAssetNames: [],
  });

  assert.deepEqual(
    candidates.map((candidate) => candidate.name),
    [
      'codetether-v4.4.1-x86_64-pc-windows-msvc.zip',
      'codetether-v4.4.1-x86_64-pc-windows-msvc.exe',
      'codetether-v4.4.1-x86_64-pc-windows-msvc.tar.gz',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.zip',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
    ]
  );
});
