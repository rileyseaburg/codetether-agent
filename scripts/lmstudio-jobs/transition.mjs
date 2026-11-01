// Charge each attempt before sending. Unknown outcomes never refund the budget.
import { randomUUID, createHash } from 'node:crypto';
import { bounded, request } from './plan.mjs';
/** @param {ReturnType<import('./plan.mjs').plan>} job */
export function prepare(job) {
  if (Date.now() >= job.deadline || job.calls >= job.maxCalls || job.reserved >= job.totalOutput) {
    job.status = 'exhausted'; return null;
  }
  const body = request(job);
  body.max_tokens = Math.min(body.max_tokens, job.totalOutput - job.reserved);
  job.calls++; job.reserved += body.max_tokens;
  job.pending = { id: randomUUID(), tokens: body.max_tokens, step: job.index, phase: job.phase };
  job.status = 'running';
  return body;
}
/** @param {ReturnType<import('./plan.mjs').plan>} job @param {object} response */
export function commit(job, response) {
  const choice = response?.choices?.[0], text = choice?.message?.content;
  if (response?.model !== job.model || typeof text !== 'string' || !text.trim() || Buffer.byteLength(text) > 128000 ||
      choice.message.tool_calls?.length || !['stop', 'length'].includes(choice.finish_reason)) throw new Error('Invalid text-only completion');
  const digest = createHash('sha256').update(text.trim()).digest('hex');
  if (job.receipts.some(item => item.step === job.index && item.phase === job.phase && item.digest === digest)) {
    job.status = 'stalled'; job.pending = null; return;
  }
  const used = response.usage?.completion_tokens;
  if (Number.isSafeInteger(used) && used > job.pending.tokens) {
    job.reserved += used - job.pending.tokens;
    job.status = 'exhausted'; job.pending = null; return;
  }
  if (Number.isSafeInteger(used) && used >= 0 && used <= job.pending.tokens) job.reserved -= job.pending.tokens - used;
  job.receipts.push({ ...job.pending, digest, response_id: response.id, finish_reason: choice.finish_reason, usage: response.usage });
  job.pending = null;
  if (job.phase === 'summary') {
    job.summary = bounded(text, 4000); job.index++; job.phase = 'answer';
  } else {
    job.parts.push({ step: job.index, text });
    if (choice.finish_reason === 'stop') {
      if (job.index === job.steps.length - 1) job.index++;
      else job.phase = 'summary';
    }
  }
  job.status = job.index >= job.steps.length ? 'complete' : 'pending';
}
