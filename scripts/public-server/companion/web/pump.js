// @ts-check
import {send} from './api.js';
import {capture} from './capture.js';
import {isRevoked} from './session.js';
/** Serialize capture + upload; schedule only after the previous request ends.
 * @param {import('./models.js').Context} context @param {number} epoch */
export async function tick(context, epoch) {
  const {state, video} = context, session = state.session;
  if (epoch !== state.epoch || state.phase !== 'sharing' || !session || state.request) return;
  state.timer = undefined;
  if (session.expires <= Date.now()) { await context.halt(true, 'Pairing expired. Get a new code.'); return; }
  const request = new AbortController(); state.request = request;
  const signal = AbortSignal.any([request.signal, AbortSignal.timeout(25000)]);
  try {
    const captured_at = new Date().toISOString();
    const image = await capture(video, signal);
    if (epoch !== state.epoch) return;
    const status = await send(session, 'frames', {image, captured_at}, signal);
    if (epoch !== state.epoch) return;
    if (isRevoked(status)) { await context.halt(true, 'Pairing ended. Get a new code on your iPhone.'); return; }
    if (status === 202) {
      state.sent += 1; state.status = 'Screenshot sent. Listen for the response on your iPhone.';
    } else if (status === 412) {
      state.status = 'Waiting for your iPhone. Keep Screen open in the foreground; retrying next interval.';
    } else if (status === 409) {
      state.status = 'CodeTether is busy or the interval budget is in use. Retrying next interval.';
    } else {
      await context.halt(false, 'Upload unavailable. Sharing paused; select Start to retry.'); return;
    }
  } catch {
    if (epoch !== state.epoch) return;
    await context.halt(false, 'Connection or capture interrupted. Sharing paused; select Start to retry.'); return;
  } finally {
    if (epoch === state.epoch) state.request = null;
  }
  if (epoch !== state.epoch) return;
  context.refresh(); state.timer = window.setTimeout(() => void context.tick(epoch), state.interval * 1000);
}