import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { auditFiles } from './audit-files.mjs';
import { zipFixture } from './zip-fixture.mjs';

const secret = Buffer.from('fixture-only-private-value-0123456789');
const directory = mkdtempSync(join(tmpdir(), 'codetether-ipa-audit-'));
const entry = join(directory, 'payload.txt');
const archive = join(directory, 'fixture.ipa');
writeFileSync(entry, Buffer.concat([secret, Buffer.alloc(4000, 65)]));
writeFileSync(archive, zipFixture(readFileSync(entry)));

test('compressed IPA credentials cannot evade exact-value auditing', () => {
  assert.equal(readFileSync(archive).includes(secret), false);
  assert.deepEqual(auditFiles([archive], [secret]), {
    count: 1, archives: 1, matches: [archive],
  });
});

test('raw values and directory contents are scanned', () => {
  assert.deepEqual(auditFiles([directory], [secret]), {
    count: 2, archives: 1, matches: [archive, entry],
  });
});

test('clean archives and files produce no matches', () => {
  const absent = Buffer.from('different-fixture-private-value');
  assert.deepEqual(auditFiles([archive, entry], [absent]), {
    count: 2, archives: 1, matches: [],
  });
});

test('missing inputs and invalid archives fail closed without leaking content', () => {
  assert.throws(() => auditFiles([], [secret]), /No audit files/);
  assert.throws(() => auditFiles([entry], []), /audit values missing/);
  assert.throws(() => auditFiles([entry], [Buffer.alloc(0)]), /audit values missing/);
  assert.throws(() => auditFiles([join(directory, 'missing')], [secret]), /ENOENT/);
  const invalid = join(directory, 'invalid.ipa');
  writeFileSync(invalid, secret);
  assert.throws(() => auditFiles([invalid], [secret]), error =>
    error.message.includes('Cannot safely audit archive:') && !error.message.includes(secret.toString()));
});