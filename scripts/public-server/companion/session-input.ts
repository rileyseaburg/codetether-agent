import { HTTPError, record } from './types.ts';
import type { SessionInput } from './types.ts';

/** Require a deliberate model and bounded observation budget. */
export function sessionInput(value: unknown): SessionInput {
  if (!record(value) || typeof value.model !== 'string'
    || !/^[a-zA-Z0-9_.:-]+\/[a-zA-Z0-9_./:-]+$/.test(value.model)
    || value.model.length > 200 || typeof value.prompt !== 'string'
    || !value.prompt.trim() || value.prompt.length > 2000
    || typeof value.interval_seconds !== 'number' || !Number.isInteger(value.interval_seconds)
    || value.interval_seconds < 15 || value.interval_seconds > 300) {
    throw new HTTPError(400, 'Choose a provider/model, instructions, and an interval from 15 to 300 seconds');
  }
  return { model: value.model, prompt: value.prompt.trim(), interval_seconds: value.interval_seconds };
}
export function pairCode(value: unknown): string {
  if (!record(value) || typeof value.code !== 'string') throw new HTTPError(400, 'Pairing code required');
  const code = value.code.replace(/[\s-]/g, '').toUpperCase();
  if (!/^[A-F0-9]{12}$/.test(code)) throw new HTTPError(401, 'Pairing code invalid or expired');
  return code;
}