'use strict';
// Read-only allowlisted statistics; never print raw config, endpoints or stderr.
const { statistics } = require('../../extensions/codetether-rust/src/cache');
statistics(process.env.CODETETHER_SCCACHE_BINARY || 'sccache', process.cwd())
  .then(value => console.log(JSON.stringify({ validation: 'static/local', ...value }, null, 2)))
  .catch(() => { console.error('Cache statistics unavailable; raw output withheld.'); process.exitCode = 1; });