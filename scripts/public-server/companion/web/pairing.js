// @ts-check
import {pair, HttpError} from './api.js';
/** Pairing never starts capture and stores no credentials outside state.
 * @param {import('./models.js').Context} context @param {string} code */
export async function connect(context, code) {
  const {state} = context;
  if (state.phase !== 'unpaired') return;
  const clean = code.trim();
  if (!clean || clean.length > 128) {
    state.status = 'Enter the pairing code shown on your iPhone.';
    context.refresh(); return;
  }
  state.phase = 'pairing'; state.status = 'Pairing with your iPhone…';
  const epoch = ++state.epoch, request = new AbortController();
  state.request = request; context.refresh();
  try {
    const session = await pair(clean, AbortSignal.any([request.signal, AbortSignal.timeout(15000)]));
    if (epoch !== state.epoch) return;
    state.session = session; state.interval = Math.max(30, session.minimum);
    state.sent = 0; state.resumeAllowed = true; state.phase = 'ready';
    state.status = 'Paired. Choose a screen or window when you are ready.';
  } catch (error) {
    if (epoch !== state.epoch) return;
    state.phase = 'unpaired';
    state.status = error instanceof HttpError && error.status < 500
      ? 'Code unavailable or pairing limited. Check Screen on your iPhone and try again.'
      : 'Pairing unavailable. Check your connection and try again.';
  } finally { if (epoch === state.epoch) { state.request = null; context.refresh(); } }
}