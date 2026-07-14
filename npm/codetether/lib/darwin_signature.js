const { spawnSync } = require('node:child_process');

function codesign(binaryPath, args, run) {
  return run('codesign', [...args, binaryPath], { stdio: 'ignore' });
}

function preserveOrAdhocSign(binaryPath, run = spawnSync) {
  const verification = codesign(binaryPath, ['--verify', '--strict'], run);
  if (verification.error) {
    if (verification.error.code === 'ENOENT') return;
    throw verification.error;
  }
  if (verification.status === 0) return;

  const signing = codesign(binaryPath, ['--force', '--sign', '-'], run);
  if (signing.error) {
    if (signing.error.code === 'ENOENT') return;
    throw signing.error;
  }
  if (signing.status !== 0) {
    throw new Error(`Unable to apply an ad-hoc signature to ${binaryPath}`);
  }
}

module.exports = { preserveOrAdhocSign };
