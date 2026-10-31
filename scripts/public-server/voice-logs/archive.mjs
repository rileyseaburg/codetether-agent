import { mkdir, open, readFile, rename, stat, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
const run = promisify(execFile);
export async function verifyNAS() {
  const { stdout } = await run('findmnt', ['-n', '-t', 'cifs', '-o', 'SOURCE', '--mountpoint', '/mnt/nas']);
  if (stdout.trim() !== '//192.168.50.133/NAS') throw new Error('NAS mount unavailable');
}
export async function loadCursor(state) {
  try { return (await readFile(join(state, 'cursor'), 'utf8')).trim(); }
  catch (error) { if (error.code === 'ENOENT') return ''; throw error; }
}
/** Acknowledge only after NAS fsync; a crash may replay a journal_id, never silently skip it. */
export async function archive(record, cursor, directory, state, mounted = verifyNAS) {
  if (typeof cursor !== 'string' || !cursor) throw new Error('Journal cursor missing');
  if (record) {
    await mounted();
    await mkdir(directory, { recursive: true, mode: 0o700 });
    const base = `${record.source}-${record.timestamp.slice(0, 10)}`;
    let path;
    for (let part = 0; ; part++) {
      path = join(directory, `${base}-${part}.jsonl`);
      const size = await stat(path).then(info => info.size).catch(error => { if (error.code === 'ENOENT') return 0; throw error; });
      if (size < 16 * 1024 * 1024) break;
    }
    const file = await open(path, 'a', 0o600);
    try { await file.writeFile(JSON.stringify(record) + '\n'); await file.sync(); } finally { await file.close(); }
  }
  await mkdir(state, { recursive: true, mode: 0o700 });
  await writeFile(join(state, 'cursor.next'), cursor, { mode: 0o600 });
  await rename(join(state, 'cursor.next'), join(state, 'cursor'));
}