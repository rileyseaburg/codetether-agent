import { createServer } from 'node:http';
import { once } from 'node:events';
import { mkdtemp } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import assert from 'node:assert/strict';
import { plan } from './plan.mjs';
import { directory, atomic, readJSON } from './files.mjs';
/** @param {import('node:test').TestContext} t @param {object[]} replies @param {object} spec */
export async function fixture(t, replies, spec = {}) {
  const calls = [];
  const server = createServer(async (request, response) => {
    assert.equal(request.headers.authorization, 'Bearer fixture');
    const chunks = []; for await (const chunk of request) chunks.push(chunk);
    const body = JSON.parse(Buffer.concat(chunks).toString('utf8')); calls.push(body);
    const reply = replies[calls.length - 1];
    if (reply?.hang) return;
    response.writeHead(reply ? 200 : 500, { 'Content-Type': 'application/json' });
    response.end(JSON.stringify({ id: `reply-${calls.length}`, model: body.model,
      choices: [{ message: { role: 'assistant', content: reply?.text }, finish_reason: reply?.finish || 'stop' }],
      usage: reply?.noUsage ? undefined : { completion_tokens: reply?.used ?? 3, prompt_tokens: 80, total_tokens: 83 } }));
  });
  server.listen(0, '127.0.0.1'); await once(server, 'listening');
  t.after(() => { server.closeAllConnections(); server.close(); });
  const origin = `http://127.0.0.1:${server.address().port}`;
  const root = await mkdtemp(join(tmpdir(), 'lmstudio-job-test-'));
  const job = { ...plan({ objective: 'Preserve original constraints', steps: ['one', 'two'], output: 64, ...spec }), origin };
  const dir = await directory(root, job.id, true); await atomic(dir, 'state.json', job);
  console.log(`Retained mocked-local checkpoint: ${dir}`);
  return { root, job, dir, calls, options: { origin, token: 'fixture' },
    load: () => readJSON(join(dir, 'state.json')) };
}
