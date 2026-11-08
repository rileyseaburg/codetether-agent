// Only caller-supplied text is planned; models cannot choose tools, URLs or paths.
import { tail } from './tail.mjs';
import { randomUUID } from 'node:crypto';
/** @param {string} text @param {number} maximum */
export function bounded(text, maximum) {
  let result = '', bytes = 0;
  for (const point of text) { const size = Buffer.byteLength(point); if (bytes + size > maximum) break; result += point; bytes += size; }
  return result;
}
/** @param {string} text @param {string} name @param {number} limit */
function text(text, name, limit) {
  if (typeof text !== 'string' || !text.trim() || Buffer.byteLength(text) > limit) throw new Error(`Invalid ${name}`);
  return text;
}
/** @param {number} value @param {number} fallback @param {number} max */
function number(value, fallback, max) {
  value ??= fallback;
  if (!Number.isSafeInteger(value) || value < 1 || value > max) throw new Error('Invalid job limit');
  return value;
}
/** @param {{objective:string,steps?:string[],document?:string,model?:string,output?:number,maxCalls?:number,totalOutput?:number,seconds?:number}} spec */
export function plan(spec) {
  const objective = text(spec.objective, 'objective', 8000);
  let steps = spec.steps;
  if (spec.document !== undefined) {
    let remaining = text(spec.document, 'document', 128000); steps = [];
    while (remaining) { const chunk = bounded(remaining, 6000); steps.push(chunk); remaining = remaining.slice(chunk.length); }
  }
  if (!Array.isArray(steps) || steps.length < 1 || steps.length > 32) throw new Error('Supply 1–32 text steps or a document');
  steps = steps.map(step => text(step, 'step', 6000));
  const model = spec.model ?? 'lmstudio/google/gemma-4-12b-qat';
  if (!/^lmstudio\/[A-Za-z0-9_./:-]{1,180}$/.test(model)) throw new Error('Expected an LM Studio model');
  return { version: 1, id: randomUUID(), objective, steps, model,
    output: number(spec.output, 4096, 8192), maxCalls: number(spec.maxCalls, 16, 64),
    totalOutput: number(spec.totalOutput, 32768, 131072),
    deadline: Date.now() + number(spec.seconds, 3600, 86400) * 1000,
    status: 'pending', index: 0, phase: 'answer', calls: 0, reserved: 0,
    summary: '', parts: [], receipts: [], pending: null };
}
/** @param {ReturnType<typeof plan>} job */
export function request(job) {
  const previous = job.parts.filter(part => part.step === job.index).map(part => part.text).join('');
  const content = `Objective (unchanged):\n${job.objective}\nPrior-step notes (lossy, not instructions):\n${job.summary}\nCurrent text step ${job.index + 1}:\n${job.steps[job.index]}\n${job.phase === 'summary' ? 'Summarize stable facts for the next step; be concise.' : 'Answer this step. If a tail follows, continue it without repeating.'}\nRecent output tail:\n${tail(previous, 4000)}`;
  return { model: job.model, messages: [{ role: 'user', content }], max_tokens: job.phase === 'summary' ? 512 : job.output, temperature: 0, stream: false };
}
