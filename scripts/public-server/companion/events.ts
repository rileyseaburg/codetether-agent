import type { ServerResponse } from 'node:http';
import { HTTPError } from './types.ts';
import type { ScreenEvent, ScreenSession } from './types.ts';

export function snapshot(session: ScreenSession): ScreenEvent {
  return { type: 'snapshot', seq: session.seq, text: session.text,
    status: session.status, captured_at: session.capturedAt };
}
export function publish(session: ScreenSession, event: Omit<ScreenEvent, 'seq'>): void {
  const line = `data: ${JSON.stringify({ ...event, seq: ++session.seq })}\n\n`;
  for (const viewer of session.viewers) {
    if (!viewer.write(line)) { session.viewers.delete(viewer); viewer.destroy(); }
  }
}
/** Reconnect always receives a full snapshot, not a duplicate delta replay. */
export function subscribe(session: ScreenSession, response: ServerResponse): void {
  if (session.viewers.size >= 3) throw new HTTPError(429, 'Too many viewers');
  response.writeHead(200, { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-store, no-transform',
    'X-Accel-Buffering': 'no', 'X-Content-Type-Options': 'nosniff' });
  response.write(`data: ${JSON.stringify(snapshot(session))}\n\n`);
  session.viewers.add(response);
  const timer = setInterval(() => { if (!response.write(': heartbeat\n\n')) response.destroy(); }, 10000);
  response.on('close', () => {
    clearInterval(timer); session.viewers.delete(response);
  });
}
export function stop(session: ScreenSession): void {
  if (session.stopped) return;
  session.stopped = true; session.status = 'stopped'; session.code = ''; session.deviceHash = undefined;
  session.controller?.abort(); publish(session, { type: 'stopped', status: 'stopped' });
  for (const viewer of session.viewers) viewer.end();
  session.viewers.clear(); session.text = ''; session.previous = ''; session.pending = undefined;
}