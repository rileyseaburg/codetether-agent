import { publish } from './events.ts';
import { runAnalysis } from './analysis-run.ts';
import { HTTPError } from './types.ts';
import type { Analyze, Capture, ScreenSession } from './types.ts';

/** Bound automatic/click captures; owner questions require fresh, matching frames. */
export function capture(session: ScreenSession, frame: Capture, analyze: Analyze, now = Date.now()): void {
  if (session.stopped) throw new HTTPError(410, 'Screen session ended');
  const pending = session.pending;
  if (pending && (frame.request_id !== pending.id || Date.parse(frame.captured_at) < pending.created)) {
    throw new HTTPError(409, 'A fresh requested screenshot is required');
  }
  if (frame.request_id && !pending) throw new HTTPError(409, 'Capture request is no longer active');
  const cooldown = frame.trigger && frame.trigger !== 'periodic' ? 5000 : session.interval * 1000;
  if (session.controller || (!pending && now - session.lastAt < cooldown)) {
    throw new HTTPError(409, 'Analysis busy or capture interval not reached');
  }
  if (session.frames >= 120) throw new HTTPError(410, 'Capture budget reached; create a new session');
  const controller = new AbortController();
  session.controller = controller; session.lastAt = now; session.frames++;
  session.pending = undefined;
  session.text = ''; session.status = 'analyzing'; session.capturedAt = frame.captured_at;
  publish(session, { type: 'capture', status: 'analyzing', captured_at: frame.captured_at });
  runAnalysis(session, frame, analyze, controller, pending?.question ?? session.prompt);
}
export function pause(session: ScreenSession): void {
  session.controller?.abort(); session.pending = undefined; session.reply = undefined; session.status = 'paused';
  publish(session, { type: 'snapshot', status: 'paused', text: session.text, captured_at: session.capturedAt });
}