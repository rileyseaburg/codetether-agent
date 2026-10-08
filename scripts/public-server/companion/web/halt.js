// @ts-check
import {send} from './api.js';
import {release} from './models.js';
import {isRevoked} from './session.js';
/** Immediately stop local pixels, then best-effort abort server analysis.
 * @param {import('./models.js').Context} context @param {boolean} unpair
 * @param {string} notice @param {boolean} [keepalive] */
export async function halt(context, unpair, notice, keepalive = false) {
  const {state} = context, session = state.session;
  release(context);
  const epoch = state.epoch;
  if (unpair) state.session = null;
  state.phase = session ? 'pausing' : 'unpaired';
  state.resumeAllowed = false; state.status = notice; context.refresh();
  if (!session) return;
  const request = new AbortController(); state.request = request;
  try {
    const status = await send(session, 'pause', {},
      AbortSignal.any([request.signal, AbortSignal.timeout(5000)]), keepalive);
    if (epoch !== state.epoch) return;
    if (isRevoked(status)) { state.session = null; state.status = 'Pairing ended. Get a new code on your iPhone.'; }
    else if (status !== 200) throw new Error('Pause unavailable');
    else state.resumeAllowed = true;
  } catch {
    if (epoch !== state.epoch) return;
    state.status = 'Capture stopped locally. Server pause not confirmed; check your iPhone. Retry Pause or Stop & unpair.';
  } finally {
    if (epoch === state.epoch) { state.request = null; state.phase = state.session ? 'paused' : 'unpaired'; context.refresh(); }
  }
}