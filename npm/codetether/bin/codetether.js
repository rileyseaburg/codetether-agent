#!/usr/bin/env node

const { spawn } = require('node:child_process');
const path = require('node:path');
const fs = require('node:fs');

const { ensureInstalled, platformTriple } = require('../lib/installer');

function binPath() {
  const triple = platformTriple();
  const name = process.platform === 'win32' ? 'codetether.exe' : 'codetether';
  return path.join(__dirname, '..', 'vendor', triple, name);
}

function exists(p) {
  try {
    fs.accessSync(p, fs.constants.X_OK);
    return true;
  } catch {
    return false;
  }
}

(async () => {
  try {
    const p = binPath();
    if (!exists(p)) {
      await ensureInstalled({ allowLatestFallback: true });
    }

    const child = spawn(binPath(), process.argv.slice(2), {
      stdio: 'inherit',
      windowsHide: true,
    });

    child.on('exit', (code, signal) => {
      if (signal) {
        // Match common CLI behavior.
        process.kill(process.pid, signal);
        return;
      }
      process.exit(code == null ? 1 : code);
    });

    child.on('error', (err) => {
      // eslint-disable-next-line no-console
      console.error(`Failed to run codetether: ${err.message}`);
      process.exit(1);
    });
  } catch (err) {
    // eslint-disable-next-line no-console
    console.error(String(err && err.message ? err.message : err));
    process.exit(1);
  }
})();
