import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { archive, loadCursor } from './archive.mjs';
import { sanitize } from './sanitize.mjs';
import { lines } from './lines.mjs';
const directory = '/mnt/nas/CodeTether/logs/voice';
const state = join(process.env.HOME, '.local/state/codetether-voice-logs');
async function follow() {
  const cursor = await loadCursor(state);
  const args = ['--user', '--output=json', '--follow', '--no-tail', '--no-pager',
    '-u', 'codetether-audio.service', '-u', 'kokoro-tts.service', '-u', 'codetether-spotlessbinco-thinker.service',
    ...(cursor ? [`--after-cursor=${cursor}`] : ['--since=-30min'])];
  const child = spawn('journalctl', args, { stdio: ['ignore', 'pipe', 'inherit'] });
  const closed = once(child, 'close');
  closed.catch(() => {});
  try {
    for await (const line of lines(child.stdout)) {
      const entry = JSON.parse(line);
      await archive(sanitize(entry), entry.__CURSOR, directory, state);
    }
    const [code] = await closed;
    if (code !== 0) throw new Error('Journal reader exited');
  } finally { child.kill(); }
}
for (;;) {
  try { await follow(); }
  catch { console.error(JSON.stringify({ event: 'voice_log_archive_retry',
    timestamp: new Date().toISOString(), diagnostic: 'Archive unavailable; journal cursor retained' })); }
  await delay(5000);
}