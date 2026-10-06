// Exact-value credential audit independent of credential retrieval.
import { lstatSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { archiveContent, isArchive } from './archive-content.mjs';

/**
 * @param {string[]} paths
 * @param {Buffer[]} secrets
 * @returns {{count: number, archives: number, matches: string[]}}
 */
export function auditFiles(paths, secrets) {
  if (!secrets.length || secrets.some((secret) => !Buffer.isBuffer(secret) || !secret.length))
    throw new Error('Required audit values missing');
  /** @type {{count: number, archives: number, matches: string[]}} */
  const result = { count: 0, archives: 0, matches: [] };
  /** @param {string} path @returns {void} */
  function scan(path) {
    const stat = lstatSync(path);
    if (stat.isDirectory()) {
      for (const entry of readdirSync(path)) scan(join(path, entry));
      return;
    }
    if (!stat.isFile()) throw new Error(`Unsupported audit input: ${path}`);
    const raw = readFileSync(path);
    const contents = [raw];
    if (isArchive(path)) {
      contents.push(archiveContent(path));
      result.archives += 1;
    }
    if (contents.some((bytes) => secrets.some((secret) => bytes.includes(secret))))
      result.matches.push(path);
    result.count += 1;
  }
  for (const path of paths) scan(path);
  if (!result.count) throw new Error('No audit files supplied');
  return result;
}