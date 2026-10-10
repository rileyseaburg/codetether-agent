import { record } from './types.ts';

/** Parse bounded SSE records, including split UTF-8 and CRLF, until explicit DONE. */
export async function readAnalysis(body: ReadableStream<Uint8Array>, delta: (text: string) => void): Promise<void> {
  const reader = body.getReader(), decoder = new TextDecoder();
  let pending = '', data: string[] = [];
  try {
    for (;;) {
      const chunk = await reader.read();
      if (chunk.done) throw new Error('Analysis stream ended without DONE');
      pending += decoder.decode(chunk.value, { stream: true });
      if (pending.length > 131072) throw new Error('Analysis event exceeds limit');
      let newline: number;
      while ((newline = pending.indexOf('\n')) >= 0) {
        const line = pending.slice(0, newline).replace(/\r$/, ''); pending = pending.slice(newline + 1);
        if (line.startsWith('data:')) data.push(line.slice(5).trimStart());
        if (data.reduce((total, part) => total + part.length, 0) > 131072) throw new Error('Analysis event exceeds limit');
        if (line !== '' || !data.length) continue;
        const payload = data.join('\n'); data = [];
        if (payload === '[DONE]') return;
        const value: unknown = JSON.parse(payload);
        if (!record(value) || value.error) {
          const detail = record(value) && record(value.error) ? String(value.error.message ?? value.error.code ?? '') : String(record(value) ? value.error : '');
          throw new Error(`Analysis provider error: ${detail.slice(0, 160)}`);
        }
        if (!Array.isArray(value.choices)) continue;
        for (const choice of value.choices as unknown[]) {
          if (!record(choice)) continue;
          if (choice.finish_reason === 'error') throw new Error('Analysis provider error');
          if (!record(choice.delta)) continue;
          if (choice.delta.tool_calls) throw new Error('Tools are not permitted for screen analysis');
          if (typeof choice.delta.content === 'string') delta(choice.delta.content);
        }
      }
    }
  } finally {
    await reader.cancel().catch(() => undefined); reader.releaseLock();
  }
}