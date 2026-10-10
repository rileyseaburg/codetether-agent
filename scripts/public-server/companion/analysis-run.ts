import { publish } from './events.ts';
import { finishTyping } from './model-typing.ts';
import type { Analyze, Capture, ScreenSession } from './types.ts';

/** Own one bounded model stream independently of iPhone connectivity. */
export function runAnalysis(session: ScreenSession, frame: Capture, analyze: Analyze,
  controller: AbortController, prompt: string): void {
  const timer = setTimeout(() => controller.abort(), 120000);
  const started = Date.now();
  console.info(JSON.stringify({ event: 'screen_analysis_started', model: session.model }));
  void Promise.resolve().then(() => analyze({ image: frame.image, model: session.model, prompt,
    previous: session.previous, signal: controller.signal, delta: (text: string): void => {
      if (controller.signal.aborted || session.stopped) return;
      if (session.text.length + text.length > 32000) { controller.abort(); return; }
      session.text += text; publish(session, { type: 'delta', text });
    } })).then(() => {
    if (controller.signal.aborted || session.stopped || session.controller !== controller) return;
    if (!session.text.trim()) throw new Error('Empty analysis');
    finishTyping(session, frame, prompt);
    session.previous = session.text.slice(-4000); session.status = 'ready';
    publish(session, { type: 'done', text: session.text, status: 'ready' });
  }).catch((error: unknown) => {
    console.warn(JSON.stringify({ event: 'screen_analysis_failed', model: session.model,
      reason: error instanceof Error ? error.message.slice(0, 200) : 'unknown',
      aborted: controller.signal.aborted, chars: session.text.length, ms: Date.now() - started }));
    if (session.stopped || controller.signal.aborted) return;
    session.status = 'error'; publish(session, { type: 'error',
      text: 'Analysis failed. Check the selected vision model and try again.', status: 'error' });
  }).finally(() => {
    clearTimeout(timer);
    if (session.controller !== controller) return;
    session.controller = undefined;
    if (controller.signal.aborted && !session.stopped && !session.pending && session.status !== 'paused') {
      session.status = 'error'; publish(session, { type: 'error',
        text: 'Analysis interrupted or exceeded its limit. Request another capture.', status: 'error' });
    }
  });
}