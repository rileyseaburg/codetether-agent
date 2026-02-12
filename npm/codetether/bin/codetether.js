#!/usr/bin/env node

const { spawn } = require('node:child_process');

const { ensureInstalled } = require('../lib/installer');

(async () => {
  try {
    const { destPath } = await ensureInstalled({ allowLatestFallback: true });

    const child = spawn(destPath, process.argv.slice(2), {
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
