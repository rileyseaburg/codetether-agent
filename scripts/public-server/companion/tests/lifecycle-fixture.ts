import { readFileSync } from 'node:fs';
import { HTTPError } from '../types.ts';
import type { SessionInput } from '../types.ts';

interface Boundary { age: number; status: number }
interface LifecycleFixture {
  now: number;
  input: SessionInput;
  pairing: Boundary[];
  session: Boundary[];
  capacity: number;
  attempts: number;
  window_ms: number;
}
/** Synthetic lifecycle boundaries shared with the Rust session core. */
export const lifecycle = JSON.parse(readFileSync(new URL(
  '../../../../crates/codetether-companion-core/fixtures/lifecycle.json', import.meta.url
), 'utf8')) as LifecycleFixture;

export function status(operation: () => unknown): number {
  try { operation(); return 200; }
  catch (error: unknown) { if (error instanceof HTTPError) return error.status; throw error; }
}