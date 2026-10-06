// Normalize only trailing whitespace and EOF newlines in this iOS project.
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { resolve, join } from 'node:path';
const rootOption = process.argv.indexOf('--root');
const root = rootOption >= 0 ? resolve(process.argv[rootOption + 1]) : resolve(new URL('..', import.meta.url).pathname);
const write = process.argv.includes('--write');
let changed = 0;
function visit(directory) {
  for (const item of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, item.name);
    if (item.isDirectory()) {
      if (!item.name.startsWith('.') && !item.name.startsWith('build') && !item.name.endsWith('.xcodeproj')) visit(path);
    } else if (/\.(swift|mjs|sh|rb|yml|md|conf)$/.test(item.name)) {
      const original = readFileSync(path, 'utf8');
      const normalized = original.split('\n').map(line => line.trimEnd()).join('\n').trimEnd() + '\n';
      if (original !== normalized) {
        changed++;
        if (write) writeFileSync(path, normalized);
      }
    }
  }
}
visit(root);
console.log(`${write ? 'Normalized' : 'Needs normalization:'} ${changed} text files`);
if (changed && !write) process.exitCode = 1;
