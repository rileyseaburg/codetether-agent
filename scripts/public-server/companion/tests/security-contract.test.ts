import assert from 'node:assert/strict';
import { test } from 'node:test';
import { digest, requireOrigin, requireToken } from '../auth.ts';
import { pairCode } from '../session-input.ts';
import { HTTPError } from '../types.ts';
import { request, security } from './security-fixture.ts';

function status(operation: () => void): number {
  try { operation(); return 200; }
  catch (error: unknown) {
    assert.ok(error instanceof HTTPError);
    return error.status;
  }
}
test('owner authentication agrees with Rust security fixtures', () => {
  for (const item of security.authorization) {
    assert.equal(status(() => requireToken(request('authorization', item.header),
      digest(security.owner))), item.status);
  }
});
test('native and browser origins agree with Rust security fixtures', () => {
  for (const item of security.origins) {
    assert.equal(status(() => requireOrigin(request('origin', item.origin),
      security.allowed_origin)), item.status);
  }
});
test('pairing code normalization agrees with Rust security fixtures', () => {
  for (const item of security.normalized_codes) {
    assert.equal(pairCode({ code: item.input }), item.output);
  }
});