import { readFileSync } from 'node:fs';
import type { Capture, ScreenEvent, SessionInput, SessionReceipt } from '../types.ts';

interface WireFixtures {
  session_input: SessionInput;
  session_receipt: SessionReceipt;
  pair_request: { code: string };
  pair_receipt: { id: string; device_token: string; interval_seconds: number; expires_at: string };
  capture_request: { question: string };
  request_receipt: { request_id: string };
  command_idle: { request_id: null };
  command_pending: { request_id: string };
  frame_legacy: Capture;
  frames: Capture[];
  accepted: { accepted: boolean };
  paused: { paused: boolean };
  stopped: { stopped: boolean };
  error: { error: string };
  events: ScreenEvent[];
  invalid_frames: unknown[];
}

/** Synthetic data shared with Rust tests; not credentials or a decodable photograph. */
export const wire = JSON.parse(readFileSync(new URL(
  '../../../../crates/codetether-companion-protocol/fixtures/wire.json', import.meta.url
), 'utf8')) as WireFixtures;
export const wireNow = Date.parse(wire.frame_legacy.captured_at);

/** Only normalize generated sequence/time values, retaining exact wire shape. */
export function normalizeEvent(event: ScreenEvent, expected: ScreenEvent): ScreenEvent {
  return { ...event, seq: expected.seq,
    ...(event.captured_at ? { captured_at: wire.frame_legacy.captured_at } : {}) };
}
export function decodeEvents(text: string): ScreenEvent[] {
  return text.split('\n').filter(line => line.startsWith('data: '))
    .map(line => JSON.parse(line.slice(6)) as ScreenEvent);
}