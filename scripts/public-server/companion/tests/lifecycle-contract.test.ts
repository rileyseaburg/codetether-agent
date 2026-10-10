import { test } from 'node:test';
import assert from 'node:assert/strict';
import { Registry } from '../registry.ts';
import { lifecycle as f, status } from './lifecycle-fixture.ts';

test('static/local: shared inclusive pairing and session expiry boundaries', () => {
  for (const boundary of f.pairing) {
    const registry = new Registry(), receipt = registry.create(f.input, f.now);
    assert.equal(status(() => registry.pair(receipt.code, f.now + boundary.age)), boundary.status);
  }
  for (const boundary of f.session) {
    const registry = new Registry(), receipt = registry.create(f.input, f.now);
    assert.equal(status(() => registry.get(receipt.id, f.now + boundary.age)), boundary.status);
  }
});

test('static/local: shared capacity and pairing-window boundaries', () => {
  const registry = new Registry();
  for (let i = 0; i < f.capacity; i++) registry.create(f.input, f.now);
  assert.equal(status(() => registry.create(f.input, f.now)), 429);
  const expires = f.now + f.session[1].age;
  assert.equal(status(() => registry.create(f.input, expires)), 200);
  for (let i = 0; i < f.attempts; i++) {
    assert.equal(status(() => registry.pair('000000000000', expires)), 401);
  }
  assert.equal(status(() => registry.pair('000000000000', expires + f.window_ms - 1)), 429);
  assert.equal(status(() => registry.pair('000000000000', expires + f.window_ms)), 401);
});

test('static/local: successful pairing consumes the shared attempt budget', () => {
  const registry = new Registry(), receipt = registry.create(f.input, f.now);
  registry.pair(receipt.code, f.now);
  for (let i = 1; i < f.attempts; i++) {
    assert.equal(status(() => registry.pair('000000000000', f.now)), 401);
  }
  assert.equal(status(() => registry.pair('000000000000', f.now)), 429);
});