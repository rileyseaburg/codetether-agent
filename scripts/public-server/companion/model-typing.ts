import { queueReply } from './replies.ts';
import { requestsTyping, typingProposal } from './typing-proposal.ts';
import type { Capture, ScreenSession } from './types.ts';

/** Hand off once, only after a successful fresh owner-requested analysis. */
export function finishTyping(session: ScreenSession, frame: Capture, prompt: string): void {
  if (!frame.request_id || !requestsTyping(prompt)) return;
  const proposal = typingProposal(session.text);
  if (!proposal) {
    session.text = 'No typing queued: the model did not return valid keyboard text. Ask again with the intended field visible.';
    return;
  }
  let notice = 'Typing queued for Windows, not confirmed as inserted. No automatic retry.';
  try { queueReply(session, { text: proposal.text }); }
  catch { notice = 'Typing was not queued. Check Windows before making a new request.'; }
  const explanation = session.text.replaceAll('\r\n', '\n').split('```windows-reply\n')[0].trimEnd();
  const bounded = Array.from(explanation).slice(0, 12000).join('');
  session.text = `${bounded}\n\n${notice}\n\nTarget: ${proposal.target}\n\n\`\`\`text\n${proposal.text}\n\`\`\``;
}
