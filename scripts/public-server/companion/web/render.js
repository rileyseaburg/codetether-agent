// @ts-check
/** @param {string} id @returns {HTMLButtonElement} */
export function button(id) {
  const value = document.getElementById(id);
  if (!(value instanceof HTMLButtonElement)) throw new Error('Missing button');
  return value;
}
/** @param {import('./models.js').State} state @returns {void} */
export function render(state) {
  const code = /** @type {HTMLInputElement} */ (document.getElementById('code'));
  const interval = /** @type {HTMLInputElement} */ (document.getElementById('interval'));
  const status = document.getElementById('status'), indicator = document.getElementById('indicator');
  const count = document.getElementById('count');
  if (!status || !indicator || !count) return;
  status.textContent = state.status; count.textContent = `${state.sent} screenshots sent`;
  indicator.textContent = state.phase === 'sharing' ? 'SHARING SCREEN' : 'NOT SHARING';
  indicator.classList.toggle('sharing', state.phase === 'sharing');
  code.disabled = state.phase !== 'unpaired';
  if (state.session) code.value = '';
  button('pair').disabled = state.phase !== 'unpaired';
  button('start').disabled = !state.session || !state.resumeAllowed || !['ready', 'paused'].includes(state.phase);
  button('pause').disabled = !state.session || state.phase === 'pausing';
  button('stop').disabled = state.phase === 'unpaired' || state.phase === 'pausing';
  interval.disabled = !state.session; interval.min = String(state.session?.minimum ?? 15);
  interval.value = String(state.interval);
}