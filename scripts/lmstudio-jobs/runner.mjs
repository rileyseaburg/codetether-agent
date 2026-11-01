// Text-only jobs use the authenticated CodeTether API, hence its provider queue.
import { join } from 'node:path';
import { directory, readJSON, atomic } from './files.mjs';
import { lock, cancelled } from './lock.mjs';
import { endpoint, complete } from './http.mjs';
import { prepare, commit } from './transition.mjs';
import { failure } from './failure.mjs';
/** @param {string} root @param {string} id @param {{origin:string,token:string,recover?:boolean,onStep?:(step:object)=>void}} options */
export async function run(root, id, options) {
  const dir = await directory(root, id), release = await lock(dir, options.recover ?? false);
  try {
    const job = await readJSON(join(dir, 'state.json'));
    if (job.version !== 1 || job.id !== id || job.origin !== endpoint(options.origin)) throw new Error('Job identity or endpoint mismatch');
    if (['complete', 'cancelled', 'exhausted', 'stalled'].includes(job.status)) return job;
    if (job.status === 'running' || job.status === 'uncertain') {
      job.status = 'uncertain'; await atomic(dir, 'state.json', job);
      if (!options.recover) throw new Error('Previous request outcome is uncertain; use explicit resume');
      job.pending = null; job.status = 'pending';
    }
    while (job.status === 'pending') {
      if (await cancelled(dir)) { job.status = 'cancelled'; break; }
      const body = prepare(job);
      await atomic(dir, 'state.json', job);
      if (!body) break;
      try {
        const response = await complete(job.origin, options.token, body, dir, job.deadline);
        if (await cancelled(dir)) { job.status = 'cancelled'; break; }
        const next = structuredClone(job);
        commit(next, response);
        await atomic(dir, 'state.json', next);
        Object.assign(job, next);
        try { options.onStep?.({ id, calls: job.calls, status: job.status, completedSteps: job.index }); } catch { /* reporting is not a job mutation */ }
      } catch (error) {
        job.status = await cancelled(dir) ? 'cancelled' : 'uncertain';
        job.failure = failure(error);
        await atomic(dir, 'state.json', job);
        throw new Error(`Job ${job.status} (${job.failure}); committed progress and reserved budget retained`);
      }
    }
    await atomic(dir, 'state.json', job);
    return job;
  } finally { await release(); }
}
