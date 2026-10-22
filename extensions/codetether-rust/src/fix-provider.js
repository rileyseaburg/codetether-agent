'use strict';
const { spawn } = require('node:child_process');

/** Request a tool-free native proposal; no shell or edit authority. Example: propose(exe, request, cwd, model, token). */
function propose(executable, request, cwd, model, token) {
  return new Promise((resolve, reject) => {
    if (token?.isCancellationRequested) { reject(Error('Repair cancelled.')); return; }
    const args = ['rust-propose'];
    if (model) args.push('--model', model);
    const child = spawn(executable, args, { cwd, shell: false, stdio: ['pipe', 'pipe', 'pipe'] });
    let output = '', errors = '', bytes = 0, settled = false;
    let timer, cancellation;
    const finish = (error, value) => {
      if (settled) return;
      settled = true; clearTimeout(timer); cancellation?.dispose();
      if (error) { child.kill(); reject(error); } else resolve(value);
    };
    timer = setTimeout(() => finish(Error('CodeTether proposal timed out after 130 seconds.')), 130000);
    cancellation = token?.onCancellationRequested(() => finish(Error('Repair cancelled. No proposal was applied.')));
    if (token?.isCancellationRequested) { finish(Error('Repair cancelled.')); return; }
    child.on('error', error => finish(error));
    child.stdout.setEncoding('utf8'); child.stderr.setEncoding('utf8');
    child.stdout.on('data', chunk => {
      bytes += Buffer.byteLength(chunk);
      if (bytes > 1024 * 1024) finish(Error('CodeTether proposal exceeded its output limit.'));
      else output += chunk.toString();
    });
    child.stderr.on('data', chunk => { errors = (errors + chunk.toString()).slice(-16000); });
    child.stdin.on('error', error => finish(error));
    child.on('close', code => {
      if (code !== 0) return finish(Error(`CodeTether proposal failed (${code}): ${errors || output}`));
      try { finish(undefined, JSON.parse(output)); } catch { finish(Error('CodeTether returned an invalid proposal; no files changed.')); }
    });
    child.stdin.end(JSON.stringify(request));
  });
}
module.exports = { propose };