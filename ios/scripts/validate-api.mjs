import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { probe } from './api-probe.mjs';

const directory = process.argv[2];
if (!directory) throw new Error('Evidence directory required');
mkdirSync(directory, { recursive: true });
const token = execFileSync('vault', ['kv', 'get', '-field=token',
  'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const base = 'https://server.codetether.run';
const local = 'http://127.0.0.1:8016';
const message = [{ role: 'user', content: 'Reply with only: ready' }];
const history = [{ role: 'user', content: 'My check word is cedar.' },
  { role: 'assistant', content: 'Your check word is cedar.' },
  { role: 'user', content: 'What is my check word? Reply with the word only.' }];
const chat = (model, messages) => ({ model, messages, stream: false, max_tokens: 4096 });
const speech = { script: 'CodeTether voice validation. Kokoro is speaking.', voice_id: 'af_heart' };
const checks = [
  { name: 'health', url: `${base}/health`, expected: 200 },
  { name: 'anonymous-version', url: `${base}/api/version`, expected: 401 },
  { name: 'authenticated-version', url: `${base}/api/version`, auth: true, expected: 200 },
  { name: 'phone-selected-chat', url: `${base}/v1/chat/completions`, auth: true, expected: 200,
    body: chat('github-copilot/gpt-5.6-luna', message), reply: 'ready' },
  { name: 'codex-multiturn', url: `${base}/v1/chat/completions`, auth: true, expected: 200,
    body: chat('openai-codex/gpt-5.5', history), reply: 'cedar' },
  { name: 'kokoro-health', url: `${local}/health`, expected: 200 },
  { name: 'kokoro-voices', url: `${local}/voices`, expected: 200 },
  { name: 'kokoro-local-speech', url: `${local}/tts/speak`, body: speech, expected: 200, audio: true },
  { name: 'kokoro-public-speech', url: `${base}/tts/speak`, auth: true, body: speech, expected: 200, audio: true }
];
const results = [];
for (const check of checks) { const result = await probe(check, token, directory); results.push(result); console.log(JSON.stringify(result)); }
writeFileSync(join(directory, 'summary.json'), JSON.stringify({ checkedAt: new Date(), results }, null, 2));
if (results.some(result => !result.ok)) process.exitCode = 1;
