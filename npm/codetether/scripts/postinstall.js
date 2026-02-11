const { ensureInstalled } = require('../lib/installer');

(async () => {
  try {
    await ensureInstalled({ allowLatestFallback: true });
  } catch (err) {
    // Postinstall failures should be noisy but not totally fatal to installation.
    // The bin wrapper will retry on first run.
    // eslint-disable-next-line no-console
    console.warn(String(err && err.message ? err.message : err));
  }
})();
