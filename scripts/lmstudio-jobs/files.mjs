// Private atomic checkpoints. No credentials enter these files.
import { constants } from 'node:fs';
import { mkdir, lstat, realpath, open, rename } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { randomUUID } from 'node:crypto';
/** @param {string} root @param {string} id @param {boolean} create */
export async function directory(root, id, create = false) {
  if (!/^[a-f0-9-]{36}$/.test(id)) throw new Error('Invalid job ID');
  root = resolve(root);
  await mkdir(root, { recursive: true, mode: 0o700 });
  if (await realpath(root) !== root) throw new Error('Job root must not traverse symlinks');
  const path = join(root, id);
  if (create) await mkdir(path, { mode: 0o700 });
  for (const item of [root, path]) {
    const stat = await lstat(item);
    if (!stat.isDirectory() || stat.isSymbolicLink() || stat.uid !== process.getuid?.() || (stat.mode & 0o077)) {
      throw new Error('Job directories must be private, owned directories');
    }
  }
  return path;
}
/** @param {string} path */
export async function readJSON(path) {
  const file = await open(path, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if ((await file.stat()).size > 2 * 1024 * 1024) throw new Error('Checkpoint too large');
    return JSON.parse(await file.readFile('utf8'));
  } finally { await file.close(); }
}
/** @param {string} dir @param {string} name @param {object} value */
export async function atomic(dir, name, value) {
  const data = JSON.stringify(value, null, 2) + '\n';
  if (Buffer.byteLength(data) > 2 * 1024 * 1024) throw new Error('Checkpoint too large');
  const temporary = join(dir, `${name}.${randomUUID()}.tmp`);
  const file = await open(temporary, 'wx', 0o600);
  try { await file.writeFile(data); await file.sync(); } finally { await file.close(); }
  await rename(temporary, join(dir, name));
  const parent = await open(dir, 'r');
  try { await parent.sync(); } finally { await parent.close(); }
}
