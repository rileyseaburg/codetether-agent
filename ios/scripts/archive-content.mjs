// Read decompressed entries in memory; never extract untrusted paths to disk.
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';

/** @param {string} path @returns {Buffer} */
export function archiveContent(path) {
  const archive = resolve(path);
  const options = { maxBuffer: 128 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] };
  try {
    execFileSync('unzip', ['-tqq', archive], options);
    return execFileSync('unzip', ['-p', archive], options);
  } catch {
    // Child-process errors can contain captured entry bytes: do not expose them.
    throw new Error(`Cannot safely audit archive: ${archive}`);
  }
}

/** @param {string} path @returns {boolean} */
export const isArchive = (path) => /\.(ipa|zip)$/i.test(path);