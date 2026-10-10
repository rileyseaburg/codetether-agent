// Preserve the complete targeted test output, including failures.
import { spawnSync } from 'node:child_process';
import { writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
const dir = fileURLToPath(new URL('.', import.meta.url));
const result = spawnSync(process.execPath, ['--test',
  dir + 'http-typing.test.ts', dir + 'model-typing.test.ts'], { encoding: 'utf8' });
const output = (result.stdout ?? '') + (result.stderr ?? '');
writeFileSync(dir + 'typing-tests.log', output);
writeFileSync(dir + 'typing-tests.exit.txt', String(result.status ?? 1) + '\n');
console.log(output);
process.exitCode = result.status ?? 1;
