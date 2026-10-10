import { randomUUID } from 'node:crypto';
import { publish } from './events.ts';
import { pendingReply } from './replies.ts';
import { HTTPError, record } from './types.ts';
import type { ScreenSession } from './types.ts';

/** Owner-only fresh-frame requests; the device sees an opaque ID, not the question. */
export function requestCapture(session: ScreenSession, value: unknown, now = Date.now()): { request_id: string } {
  commandSnapshot(session, now);
  if (!record(value) || typeof value.question !== 'string' || !value.question.trim() || value.question.length > 2000) {
    throw new HTTPError(400, 'Question must contain 1–2000 characters');
  }
  if (!session.deviceHash || session.controller || session.pending) throw new HTTPError(409, 'Device unpaired or capture busy');
  if (session.frames >= 120) throw new HTTPError(429, 'Session capture budget exhausted');
  session.pending = { id: randomUUID(), question: value.question.trim(), created: now };
  session.status = 'requested';
  publish(session, { type: 'snapshot', status: session.status, text: session.text, captured_at: session.capturedAt });
  return { request_id: session.pending.id };
}

/** Expire stale requests even while the iPhone is disconnected. */
export function commandSnapshot(session: ScreenSession, now = Date.now()): { request_id: string | null; reply?: { id: string; text: string } } {
  if (session.pending && now - session.pending.created >= 60000) {
    session.pending = undefined;
    session.status = 'error';
    publish(session, { type: 'error', status: 'error', text: 'Windows did not provide a fresh screenshot within 60 seconds.' });
  }
  const commands: { request_id: string | null; reply?: { id: string; text: string } } = { request_id: session.pending?.id ?? null };
  const { reply } = pendingReply(session, now);
  if (reply) commands.reply = reply;
  return commands;
}