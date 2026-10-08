// @ts-check
import {initialState} from './models.js';
import {connect} from './pairing.js';
import {start} from './sharing.js';
import {halt} from './halt.js';
import {tick} from './pump.js';
/** Bind the lifecycle to one preview, without persistence or global tokens.
 * @param {HTMLVideoElement} video
 * @param {(state:import('./models.js').State)=>void} render */
export function createFlow(video, render) {
  const state = initialState();
  /** @type {import('./models.js').Context} */
  const context = {state, video, refresh:() => render(state),
    halt:(unpair, notice, keepalive) => halt(context, unpair, notice, keepalive),
    tick:epoch => tick(context, epoch)};
  return {
    refresh:context.refresh,
    pair:/** @param {string} code */ code => connect(context, code),
    start:() => start(context),
    pause:() => context.halt(false, 'Paused. No screen is being captured. Start opens the chooser again.'),
    stop:/** @param {boolean} [keepalive] */ keepalive => context.halt(true, 'Stopped and unpaired. No screen is being captured.', keepalive),
    interval:/** @param {number} seconds */ seconds => {
      if (!Number.isFinite(seconds) || !state.session) return;
      state.interval = Math.min(86400, Math.max(state.session.minimum, Math.ceil(seconds)));
      if (state.timer !== undefined) {
        clearTimeout(state.timer); const epoch = state.epoch;
        state.timer = window.setTimeout(() => void context.tick(epoch), state.interval * 1000);
      }
      context.refresh();
    }};
}