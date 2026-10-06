// Explicit immutable release coordinates; never infer a trusted hash from an IPA.
/** @param {string} version @param {string} build @param {string} sha256
 * @returns {{version: string, build: string, sha256: string, id: string, base: string}} */
export function releaseConfig(version, build, sha256) {
  if (!/^\d+\.\d+\.\d+$/.test(version ?? '') || !/^[1-9]\d*$/.test(build ?? ''))
    throw new Error('Expected semantic version and positive build number');
  if (!/^[a-f0-9]{64}$/.test(sha256 ?? '')) throw new Error('Expected pinned SHA-256');
  const id = version + '-' + build;
  return { version, build, sha256, id, base: 'https://ios.codetether.run/releases/' + id };
}
