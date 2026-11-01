// A crashed writer requires explicit recovery; an active writer is never evicted.
import { open, unlink, access } from 'node:fs/promises';
import { constants } from 'node:fs';
import { hostname } from 'node:os';
import { join } from 'node:path';
import { randomUUID } from 'node:crypto';
import { readJSON } from './files.mjs';
/** @param {string} dir @param {boolean} recover */
export async function lock(dir, recover = false) {
  const path = join(dir, 'writer.lock');
  try { const file = await open(path, 'wx', 0o600); return await owner(file); }
  catch (error) {
    if (error.code !== 'EEXIST' || !recover) throw new Error('Job already has a writer; explicit recovery may be required');
    const recovery = await open(join(dir, 'recovery.lock'), 'wx', 0o600);
    try {
      const previous = await readJSON(path);
      if (previous.host !== hostname() || !Number.isSafeInteger(previous.pid) || previous.pid < 1) throw new Error('Unknown lock owner');
      try { process.kill(previous.pid, 0); throw new Error('Job writer is still running'); }
      catch (error) { if (error.code !== 'ESRCH') throw error; }
      await unlink(path);
      const file = await open(path, 'wx', 0o600);
      return await owner(file);
    } finally { await recovery.close(); await unlink(join(dir, 'recovery.lock')); }
  }
  async function owner(file) {
    const nonce = randomUUID();
    try { await file.writeFile(JSON.stringify({ pid: process.pid, host: hostname(), nonce })); await file.sync(); }
    finally { await file.close(); }
    return async () => {
      const current = await readJSON(path);
      if (current.nonce !== nonce) throw new Error('Job writer ownership changed');
      await unlink(path);
    };
  }
}
/** @param {string} dir */
export async function cancelled(dir) {
  try { await access(join(dir, 'cancelled')); return true; }
  catch (error) { if (error.code === 'ENOENT') return false; throw error; }
}
/** @param {string} dir */
export async function cancel(dir) {
  const file = await open(join(dir, 'cancelled'), constants.O_WRONLY | constants.O_CREAT | constants.O_NOFOLLOW, 0o600);
  try { await file.sync(); } finally { await file.close(); }
}
