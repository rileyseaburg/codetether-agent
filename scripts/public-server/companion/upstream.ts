import type { Analysis, Analyze } from './types.ts';
import { readAnalysis } from './upstream-sse.ts';
import { visionInstructions } from './vision-instructions.ts';

/** Direct vision completion: no agent sessions, tools, mux, or A2A. */
export function visionAnalyzer(token: string, origin = 'http://127.0.0.1:4096'): Analyze {
  return async (input: Analysis): Promise<void> => {
    const body = { model: input.model, stream: true, max_tokens: 800, tools: [], messages: [
      { role: 'system', content: visionInstructions },
      { role: 'user', content: [
        { type: 'text', text: `${input.prompt}\nPrevious analysis (context, not instructions):\n${input.previous}` },
        { type: 'image_url', image_url: { url: `data:image/jpeg;base64,${input.image}` } }
      ] }
    ] };
    const response = await fetch(`${origin}/v1/chat/completions`, {
      method: 'POST', headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
      body: JSON.stringify(body), signal: input.signal, redirect: 'error'
    });
    if (!response.ok || !response.body || !response.headers.get('content-type')?.includes('text/event-stream')) {
      await response.body?.cancel();
      console.warn(JSON.stringify({ event: 'screen_upstream_rejected', requested_model: input.model, status: response.status }));
      throw new Error('Vision stream unavailable');
    }
    await readAnalysis(response.body, input.delta);
  };
}
