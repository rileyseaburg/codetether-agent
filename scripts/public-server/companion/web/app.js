// @ts-check
import {createFlow} from './flow.js';
import {button, render} from './render.js';
const video = document.querySelector('video');
if (!video) throw new Error('Missing preview');
const flow = createFlow(video, render);
document.getElementById('pair-form')?.addEventListener('submit', event => {
  event.preventDefault();
  const code = /** @type {HTMLInputElement} */ (document.getElementById('code'));
  void flow.pair(code.value);
});
button('start').addEventListener('click', () => void flow.start());
button('pause').addEventListener('click', () => void flow.pause());
button('stop').addEventListener('click', () => void flow.stop());
document.getElementById('interval')?.addEventListener('change', event => {
  if (event.target instanceof HTMLInputElement) flow.interval(Number(event.target.value));
});
window.addEventListener('pagehide', () => void flow.stop(true));
flow.refresh();
if ('serviceWorker' in navigator) void navigator.serviceWorker.register('/companion/sw.js', {scope:'/companion/'}).catch(() => undefined);