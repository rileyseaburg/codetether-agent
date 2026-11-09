// Upload staging readable by a snap-confined gh (it cannot read hidden dirs).
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
/** Staging root: CODETETHER_RELEASE_MIRROR_STAGING or ~/codetether-release-mirror-staging. */
export const stagingRoot = () =>
  process.env.CODETETHER_RELEASE_MIRROR_STAGING || path.join(os.homedir(), 'codetether-release-mirror-staging');
/** Write `bytes` to a private non-hidden file, run `use(file)`, then always delete it. */
export function withStagedFile(name, bytes, use) {
  mkdirSync(stagingRoot(), { recursive: true, mode: 0o700 });
  const dir = mkdtempSync(path.join(stagingRoot(), 'upload-'));
  try {
    const file = path.join(dir, name);
    writeFileSync(file, bytes, { mode: 0o600 });
    return use(file);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
