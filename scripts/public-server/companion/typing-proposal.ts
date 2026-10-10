import { record } from './types.ts';

/** Bounded keyboard text only; no key commands or raw analysis execution. */
export interface TypingProposal { text: string; target: string }
const controls = /[\u0000-\u001f\u007f-\u009f\u2028\u2029]/u;
const fences = /```[ \t]*(windows-reply|json)?[ \t]*\n([\s\S]*?)\n?[ \t]*```/gu;
function singleLine(value: unknown, limit: number): value is string {
  return typeof value === 'string' && !!value.trim() && value.length <= limit && !controls.test(value);
}
/** Only a fresh owner request beginning with a typing instruction authorizes input. */
export function requestsTyping(prompt: string): boolean {
  const words = prompt.trim().toLowerCase().split(/\s+/u);
  if (words[0] === 'please') words.shift();
  if (['can', 'could', 'would', 'will'].includes(words[0]) && words[1] === 'you') words.splice(0, 2);
  if (words[0] === 'please') words.shift();
  return words.length > 1 && ['type', 'enter', 'write', 'fill'].includes(words[0]);
}
function parse(body: string): TypingProposal | undefined {
  try {
    const value: unknown = JSON.parse(body.trim());
    if (record(value) && singleLine(value.text, 2000)) {
      const target = singleLine(value.target, 200) ? value.target : 'focused field';
      return { text: value.text, target };
    }
  } catch { /* Malformed model output is not actionable. */ }
}
/** Use the last windows-reply fence (or a JSON fence with text); prose is never typed. */
export function typingProposal(analysis: string): TypingProposal | undefined {
  if (analysis.length > 32000) return;
  const blocks = [...analysis.replaceAll('\r\n', '\n').matchAll(fences)];
  const labelled = blocks.filter((m) => m[1] === 'windows-reply');
  const pool = labelled.length ? labelled : blocks.filter((m) => m[1] === 'json' || !m[1]);
  for (const m of pool.reverse()) {
    const proposal = parse(m[2]);
    if (proposal) return proposal;
  }
}
