import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, readdir, readFile, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { Readable } from 'node:stream';
import { archive, loadCursor } from './archive.mjs';
import { lines } from './lines.mjs';
test('NAS failure preserves cursor; recovery archives before acknowledgement', async () => {
  const root = await mkdtemp(join(tmpdir(), 'voice-archive-test-'));
  const state = join(root, 'state'), nas = join(root, 'nas');
  const record = { timestamp: '2026-10-28T12:00:00.000Z', source: 'bridge', event: 'voice_bridge' };
  await archive(record, 'first', nas, state, async () => {});
  await assert.rejects(archive(record, 'second', nas, state, async () => { throw new Error('offline'); }));
  assert.equal(await loadCursor(state), 'first');
  await archive(record, 'second', nas, state, async () => {});
  assert.equal(await loadCursor(state), 'second');
  const files = await readdir(nas);
  assert.deepEqual(files, ['bridge-2026-10-28-0.jsonl']);
  const records = (await readFile(join(nas, files[0]), 'utf8')).trim().split('\n').map(JSON.parse);
  assert.deepEqual(records, [record, record]);
  console.log(`Retained mocked archive evidence: ${root}`);
});
test('failed NAS write does not acknowledge or create state', async () => {
  const root = await mkdtemp(join(tmpdir(), 'voice-archive-fail-'));
  const state = join(root, 'state');
  await writeFile(join(root, 'not-a-directory'), 'fixture');
  await assert.rejects(archive({ source: 'bridge', timestamp: '2026-10-28' }, 'cursor',
    join(root, 'not-a-directory', 'logs'), state, async () => {}));
  assert.equal(await loadCursor(state), '');
});
test('line reader handles split records and discards oversized records', async () => {
  const stream = Readable.from(['{"a":', '1}\n{"b":2}\n']);
  const collected = [];
  for await (const line of lines(stream)) collected.push(JSON.parse(line));
  assert.deepEqual(collected, [{ a: 1 }, { b: 2 }]);
  const oversized = Readable.from(['x'.repeat(4 * 1024 * 1024 + 1), 'tail\n{}\n']);
  const kept = [];
  for await (const line of lines(oversized)) kept.push(line);
  assert.deepEqual(kept, ['{}']);
});