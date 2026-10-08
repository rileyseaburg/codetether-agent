import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readAnalysis } from '../upstream-sse.ts';

function stream(text: string): ReadableStream<Uint8Array> {
  const bytes = new TextEncoder().encode(text);
  return new ReadableStream({ start(controller): void {
    for (const byte of bytes) controller.enqueue(new Uint8Array([byte]));
    controller.close();
  } });
}
test('mocked local: split UTF-8 and CRLF stream deltas before DONE', async () => {
  const text: string[] = [];
  await readAnalysis(stream(': heartbeat\r\n\r\ndata: {"choices":[{"delta":{"content":"héllo"}}]}\r\n\r\ndata: [DONE]\r\n\r\n'), value => { text.push(value); });
  assert.deepEqual(text, ['héllo']);
});
test('mocked local: reject truncated streams, tools and upstream errors', async () => {
  for (const text of [
    'data: {"choices":[{"delta":{"content":"partial"}}]}\n\n',
    'data: {"choices":[{"delta":{"tool_calls":[{}]}}]}\n\n',
    'data: {"error":{"message":"provider rejected"}}\n\n',
    `data: ${'x'.repeat(131073)}\n\n`
  ]) await assert.rejects(readAnalysis(stream(text), () => undefined));
});
test('mocked local: ignore role-only chunks without manufacturing output', async () => {
  let called = false;
  await readAnalysis(stream('data: {"choices":[{"delta":{"role":"assistant"}}]}\n\ndata: [DONE]\n\n'), () => { called = true; });
  assert.equal(called, false);
});