import type { ServerResponse } from 'node:http';

/** Wire events contain analysis text and timing, never screenshot bytes or credentials. */
export interface ScreenEvent {
  type: 'snapshot' | 'capture' | 'delta' | 'done' | 'error' | 'stopped';
  seq: number; text?: string; status?: string; captured_at?: string;
}
/** Ephemeral session; process restart deliberately revokes all pairings. */
export interface ScreenSession {
  id: string; code: string; pairExpires: number; expires: number;
  model: string; prompt: string; interval: number; deviceHash?: string;
  text: string; previous: string; status: string; capturedAt?: string;
  seq: number; frames: number; lastAt: number; stopped: boolean;
  controller?: AbortController; viewers: Set<ServerResponse>;
  pending?: { id: string; question: string; created: number };
  reply?: { id: string; text: string; created: number };
}
export interface Capture {
  image: string; captured_at: string;
  trigger?: 'periodic' | 'right_click' | 'double_click' | 'manual';
  request_id?: string;
}
export interface Analysis {
  image: string; model: string; prompt: string; previous: string;
  signal: AbortSignal; delta: (text: string) => void;
}
export type Analyze = (input: Analysis) => Promise<void>;
export interface SessionInput { model: string; prompt: string; interval_seconds: number }
export interface SessionReceipt {
  id: string; code: string; pair_expires_at: string;
  expires_at: string; interval_seconds: number;
}
export class HTTPError extends Error {
  readonly status: number;
  constructor(status: number, message: string) { super(message); this.status = status; }
}
export function record(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}