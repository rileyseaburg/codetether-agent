const test = require('node:test');
const assert = require('node:assert/strict');

const { selectAssetCandidates } = require('../lib/installer');

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