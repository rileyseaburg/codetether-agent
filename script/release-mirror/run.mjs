// Mirror a published Forgejo release without building or overwriting conflicting files.
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { sourceRelease, checksums } from './source.mjs';
import { matchingCommit } from './refs.mjs';
import { findRelease, createDraft, publish } from './github.mjs';
import { mirrorAssets } from './asset.mjs';
const base = process.env.CODETETHER_RELEASE_MIRROR_EVIDENCE || path.resolve('artifacts/release-mirror');
mkdirSync(base, { recursive: true, mode: 0o700 });
const evidence = mkdtempSync(path.join(base, 'run-'));
try {
  const source = await sourceRelease(process.argv[2]);
  const sha = matchingCommit(source.tag_name);
  const sums = await checksums(source);
  let target = findRelease(source.tag_name) || createDraft(source, sha);
  writeFileSync(path.join(evidence, 'before.json'), JSON.stringify({ source, target }, null, 2));
  target = await mirrorAssets(source, target, sums, evidence);
  target = publish(target, source);
  const proof = { checkedAt: new Date().toISOString(), sourceReleaseId: source.id,
    githubReleaseId: target.id, tag: source.tag_name, commit: sha, url: target.html_url,
    assets: target.assets.map(asset => ({ id: asset.id, name: asset.name, size: asset.size, digest: asset.digest })),
    buildsRun: false, result: 'success' };
  writeFileSync(path.join(evidence, 'proof.json'), JSON.stringify(proof, null, 2));
  console.log(JSON.stringify({ ...proof, evidence }));
} catch (error) {
  writeFileSync(path.join(evidence, 'failure.txt'), String(error));
  console.error(`Release mirror failed; evidence retained at ${evidence}`);
  process.exitCode = 1;
}
