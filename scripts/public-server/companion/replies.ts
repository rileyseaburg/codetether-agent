import { randomUUID } from 'node:crypto';
import { publish } from './events.ts';
import { HTTPError, record } from './types.ts';
import type { ScreenSession } from './types.ts';

/** Owner replies queued for the device to type; one at a time, no history. */
export function queueReply(session: ScreenSession, value: unknown, now = Date.now()): { reply_id: string } {
  expireReply(session, now);
  if (!record(value) || typeof value.text !== 'string' || !value.text.trim() || value.text.length > 2000) {
    throw new HTTPError(400, 'Reply must contain 1–2000 characters');
  }
  if (session.stopped || session.status === 'paused' || session.expires <= now || !session.deviceHash) {
    throw new HTTPError(409, 'Device unavailable or paused');
  }
  if (session.reply) throw new HTTPError(409, 'A reply is already queued; wait for it to be typed');
  session.reply = { id: randomUUID(), text: value.text, created: now };
  return { reply_id: session.reply.id };
}

/** Devices re-receive the pending reply until they acknowledge the typed outcome. */
export function pendingReply(session: ScreenSession, now = Date.now()): { reply: { id: string; text: string } | null } {
  expireReply(session, now);
  return { reply: session.reply ? { id: session.reply.id, text: session.reply.text } : null };
}

/** Acknowledge a typed reply from the device; unknown IDs are treated as already cleared. */
export function ackReply(session: ScreenSession, value: unknown): { typed: boolean } {
  if (!record(value) || typeof value.reply_id !== 'string') throw new HTTPError(400, 'Expected a reply acknowledgment');
  if (session.reply && session.reply.id === value.reply_id) {
    session.reply = undefined;
    publish(session, { type: 'snapshot', status: session.status, text: session.text, captured_at: session.capturedAt });
    return { typed: true };
  }
  return { typed: false };
}

/** Expire undelivered replies after 60 s so queues never grow stale. */
export function expireReply(session: ScreenSession, now = Date.now()): void {
  if (session.reply && now - session.reply.created >= 60000) {
    session.reply = undefined;
    publish(session, { type: 'error', text: 'Windows did not type the reply within 60 seconds.' });
  }
}
