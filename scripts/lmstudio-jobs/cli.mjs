#!/usr/bin/env node
// Usage: node cli.mjs create SPEC.json | run ID | resume ID | status ID | cancel ID
import { readFile } from 'node:fs/promises';
import { homedir } from 'node:os';
import { join } from 'node:path';
import { plan } from './plan.mjs';
import { endpoint } from './http.mjs';
import { directory, atomic, readJSON } from './files.mjs';
import { cancel } from './lock.mjs';
import { run } from './runner.mjs';
const root = process.env.CODETETHER_JOB_ROOT || join(homedir(), '.local/share/codetether-lmstudio-jobs');
const origin = endpoint(process.env.CODETETHER_URL);
const [command, value] = process.argv.slice(2);
try {
  if (command === 'create') {
    const raw = await readFile(value, 'utf8');
    if (Buffer.byteLength(raw) > 160000) throw new Error('Job specification too large');
    const job = { ...plan(JSON.parse(raw)), origin };
    const dir = await directory(root, job.id, true);
    await atomic(dir, 'state.json', job);
    console.log(JSON.stringify({ id: job.id, status: job.status, steps: job.steps.length }));
  } else if (['run', 'resume'].includes(command)) {
    const job = await run(root, value, { origin, token: process.env.CODETETHER_AUTH_TOKEN,
      recover: command === 'resume', onStep: event => console.log(JSON.stringify(event)) });
    console.log(JSON.stringify({ id: job.id, status: job.status, calls: job.calls, reserved: job.reserved }));
    if (job.status !== 'complete') process.exitCode = 2;
  } else if (command === 'status') {
    const dir = await directory(root, value);
    const job = await readJSON(join(dir, 'state.json'));
    console.log(JSON.stringify({ id: job.id, status: job.status, calls: job.calls, reserved: job.reserved,
      completedSteps: job.index, totalSteps: job.steps.length, checkpoint: join(dir, 'state.json') }));
  } else if (command === 'cancel') {
    const dir = await directory(root, value); await cancel(dir);
    console.log(JSON.stringify({ id: value, cancellationRequested: true }));
  } else {
    console.log('Usage: node cli.mjs create SPEC.json | run ID | resume ID | status ID | cancel ID');
    process.exitCode = command === '--help' ? 0 : 2;
  }
} catch (error) { console.error(error.message); process.exitCode = 1; }
