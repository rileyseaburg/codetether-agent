'use strict';
const actions = ['build', 'run', 'test', 'check', 'cache-build', 'cache-publish', 'status', 'doctor', 'stop'];

/** Construct literal CLI arguments; example: plan('check', '/repo', {}).args. */
function plan(action, cwd, config) {
  if (!actions.includes(action)) throw new Error(`Unsupported Rust action: ${action}`);
  if (!cwd) throw new Error('Select a Cargo workspace folder first.');
  const args = ['rust', '--cwd', cwd];
  if (!action.startsWith('cache-')) args.push('--toolchain', config.toolchain || 'stable');
  if (['build', 'run', 'test', 'check'].includes(action)) {
    if (config.package) args.push('--package', config.package);
    if (config.bin) args.push('--bin', config.bin);
  }
  args.push(action);
  if (action === 'run' && config.runArgs?.length) args.push(...config.runArgs.map(arg => `--arg=${arg}`));
  return { executable: config.executable || 'codetether', args, cwd };
}

module.exports = { actions, plan };