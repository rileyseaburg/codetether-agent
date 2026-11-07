import test from 'node:test';
import assert from 'node:assert/strict';
import { mediaRoots } from './media-roots.mjs';

test('general chat media and legacy message images remain readable', () => {
  const roots = mediaRoots('/home/fixture');
  assert.deepEqual(roots, [
    '/home/fixture/.local/share/codetether-mobile/uploads',
    '/home/fixture/.local/share/codetether-chat/.codetether-agent/generated_images',
    '/home/fixture/spotlessbinco/.codetether-agent/generated_images',
  ]);
  assert.ok(!roots.includes('/home/fixture'));
  assert.ok(!roots.includes('/home/fixture/spotlessbinco'));
});