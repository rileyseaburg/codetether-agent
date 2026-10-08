import { randomBytes, randomUUID } from 'node:crypto';
import { digest } from './auth.ts';
import { publish, stop } from './events.ts';
import { HTTPError } from './types.ts';
import type { ScreenSession, SessionInput, SessionReceipt } from './types.ts';

/** Bounded, memory-only pairing registry; process restart revokes capabilities. */
export class Registry {
  readonly sessions = new Map<string, ScreenSession>();
  private attempts = 0;
  private window = 0;
  sweep(now = Date.now()): void {
    for (const [id, session] of this.sessions) {
      if (session.expires <= now || session.stopped) { stop(session); this.sessions.delete(id); }
    }
  }
  create(input: SessionInput, now = Date.now()): SessionReceipt {
    this.sweep(now);
    if (this.sessions.size >= 4) throw new HTTPError(429, 'Stop an existing screen session first');
    const session: ScreenSession = { id: randomUUID(), code: randomBytes(6).toString('hex').toUpperCase(),
      pairExpires: now + 300000, expires: now + 3600000, model: input.model, prompt: input.prompt,
      interval: input.interval_seconds, text: '', previous: '', status: 'waiting', seq: 0,
      frames: 0, lastAt: 0, stopped: false, viewers: new Set() };
    this.sessions.set(session.id, session);
    return { id: session.id, code: session.code, pair_expires_at: new Date(session.pairExpires).toISOString(),
      expires_at: new Date(session.expires).toISOString(), interval_seconds: session.interval };
  }
  get(id: string, now = Date.now()): ScreenSession {
    const session = this.sessions.get(id);
    if (!session) throw new HTTPError(404, 'Screen session not found');
    if (session.expires <= now || session.stopped) { stop(session); throw new HTTPError(410, 'Screen session ended'); }
    return session;
  }
  pair(code: string, now = Date.now()): { id: string; device_token: string; interval_seconds: number; expires_at: string } {
    if (now - this.window >= 60000) { this.attempts = 0; this.window = now; }
    if (++this.attempts > 30) throw new HTTPError(429, 'Too many pairing attempts; wait one minute');
    const session = [...this.sessions.values()].find(item => item.code === code && !item.deviceHash);
    if (!session || session.pairExpires <= now || session.expires <= now || session.stopped) {
      throw new HTTPError(401, 'Pairing code invalid or expired');
    }
    const token = randomBytes(32).toString('base64url');
    session.deviceHash = digest(token); session.code = ''; session.status = 'paired';
    publish(session, { type: 'snapshot', status: 'paired', text: session.text });
    return { id: session.id, device_token: token, interval_seconds: session.interval, expires_at: new Date(session.expires).toISOString() };
  }
}