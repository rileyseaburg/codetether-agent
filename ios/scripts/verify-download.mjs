// Live HTTPS evidence, separate from signature and on-device validation.
import { createHash } from 'node:crypto';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { releaseConfig } from './release-config.mjs';
export async function verifyDownload(directory, version, build, sha256) {
  const { base, sha256: expected } = releaseConfig(version, build, sha256);
  await mkdir(directory, { recursive: true });
  const evidence = { validation: 'live deployment', checked_at: new Date().toISOString(), url: base + "/index.html", assets: {} };
  for (const file of ['index.html', 'manifest.plist', 'CodeTether.ipa']) {
    const response = await fetch(`${base}/${file}`, { redirect: 'error', signal: AbortSignal.timeout(30_000) });
    if (response.status !== 200) throw new Error(`${file}: HTTP ${response.status}`);
    const bytes = Buffer.from(await response.arrayBuffer());
    if (file === 'CodeTether.ipa' && createHash('sha256').update(bytes).digest('hex') !== expected)
      throw new Error('Live IPA checksum mismatch');
    if (file === 'manifest.plist' && !bytes.toString().includes(`${base}/CodeTether.ipa`))
      throw new Error('Manifest does not reference the verified HTTPS IPA');
    if (file === 'index.html' && !bytes.toString().includes('itms-services://'))
      throw new Error('Missing Safari installation link');
    await writeFile(join(directory, file), bytes);
    evidence.assets[file] = { status: response.status, bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') };
  }
  await writeFile(join(directory, 'https-verification.json'), JSON.stringify(evidence, null, 2) + '\n');
  return evidence;
}