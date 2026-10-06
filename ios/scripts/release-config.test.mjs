import test from 'node:test';
import assert from 'node:assert/strict';
import { releaseConfig } from './release-config.mjs';
test('release coordinates keep installed and camera releases distinct', () => {
  const hash = 'a'.repeat(64);
  const previous = releaseConfig('1.4.0', '8', hash);
  const camera = releaseConfig('1.5.0', '9', hash);
  assert.equal(previous.base, 'https://ios.codetether.run/releases/1.4.0-8');
  assert.equal(camera.base, 'https://ios.codetether.run/releases/1.5.0-9');
  assert.equal(camera.sha256, hash);
});
test('invalid coordinates and missing trusted hashes fail closed', () => {
  for (const inputs of [['1.5', '9', 'a'.repeat(64)], ['1.5.0', '0', 'a'.repeat(64)],
    ['../1.5.0', '9', 'a'.repeat(64)], ['1.5.0', '9', undefined],
    ['1.5.0', '9', 'not-a-hash']])
    assert.throws(() => releaseConfig(...inputs));
});