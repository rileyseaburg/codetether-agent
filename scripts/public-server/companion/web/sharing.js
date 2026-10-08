// @ts-check
/** Only called by a direct Start-button activation; no auto reacquisition.
 * @param {import('./models.js').Context} context */
export async function start(context) {
  const {state, video} = context;
  if (!state.session || !state.resumeAllowed || !['ready','paused'].includes(state.phase)) return;
  if (state.session.expires <= Date.now()) { await context.halt(true, 'Pairing expired. Get a new code.'); return; }
  if (!navigator.mediaDevices?.getDisplayMedia) {
    state.status = 'Screen sharing needs desktop Edge or Chrome on a secure HTTPS connection.';
    context.refresh(); return;
  }
  const epoch = ++state.epoch;
  state.phase = 'choosing'; state.status = 'Choose only the window or screen you intend to share.';
  context.refresh();
  try {
    const stream = await navigator.mediaDevices.getDisplayMedia({video:true, audio:false});
    if (epoch !== state.epoch) { stream.getTracks().forEach(track => track.stop()); return; }
    state.stream = stream; video.srcObject = stream;
    const track = stream.getVideoTracks()[0];
    if (!track || track.readyState === 'ended') throw new Error('Capture ended');
    track.addEventListener('ended', () => {
      if (state.stream === stream) void context.halt(true, 'Screen sharing ended. Pair again to share.');
    }, {once:true});
    await video.play();
    if (epoch !== state.epoch) return;
    state.phase = 'sharing'; state.status = 'Sharing is on. Preparing a screenshot…';
    context.refresh(); void context.tick(epoch);
  } catch {
    if (epoch !== state.epoch) return;
    state.stream?.getTracks().forEach(track => track.stop());
    state.stream = null; video.srcObject = null;
    state.phase = 'paused'; state.resumeAllowed = true;
    state.status = 'Nothing is being captured. Sharing was cancelled or unavailable. Select Start to try again.';
    context.refresh();
  }
}