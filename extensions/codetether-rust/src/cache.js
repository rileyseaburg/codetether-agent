'use strict';
const { execFile } = require('node:child_process');

/** Only override Cargo's wrapper when enabled; inherit credentials/config otherwise. */
function environment(config) {
  return config.get('useSccache', false)
    ? { CARGO_TERM_COLOR: 'never', RUSTC_WRAPPER: config.get('sccacheExecutable', 'sccache'), CARGO_INCREMENTAL: '0' }
    : { CARGO_TERM_COLOR: 'never' };
}
const count = value => typeof value === 'number' && Number.isFinite(value) && value >= 0
  ? value : value && typeof value === 'object' ? Object.values(value).reduce((sum, item) => sum + count(item), 0) : 0;

/** Allowlist statistics, never expose raw output, URLs, environment or credentials. */
function summary(value) {
  const stats = value.stats || {};
  return {
    backend: /s3/i.test(value.cache_location || '') ? 'S3-compatible (MinIO/S3)' : /local|disk/i.test(value.cache_location || '') ? 'Local disk' : 'Other / unknown',
    hits: count(stats.cache_hits), misses: count(stats.cache_misses),
    requests: count(stats.compile_requests), errors: count(stats.cache_read_errors) + count(stats.cache_write_errors)
  };
}

/** Explicit, bounded read of daemon statistics; no restart, zeroing or credential changes. */
function statistics(executable, cwd) {
  return new Promise((resolve, reject) => {
    execFile(executable, ['--show-stats', '--stats-format', 'json'],
      { cwd, timeout: 10000, maxBuffer: 1024 * 1024, windowsHide: true }, (error, stdout) => {
        if (error) return reject(Error('sccache statistics unavailable. Check the executable and host-side cache configuration.'));
        try { resolve(summary(JSON.parse(stdout))); }
        catch { reject(Error('sccache returned invalid statistics.')); }
      });
  });
}
module.exports = { environment, summary, statistics };