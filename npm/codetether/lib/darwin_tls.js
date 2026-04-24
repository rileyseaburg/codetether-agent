const https = require('node:https');
const tls = require('node:tls');
const { execSync } = require('node:child_process');

const DARWIN_CERT_CMD =
  'security find-certificate -a -p /Library/Keychains/System.keychain ' +
  '/System/Library/Keychains/SystemRootCertificates.keychain';

let cachedCa;
let cachedAgent;

function isDarwin() {
  return process.platform === 'darwin';
}

function loadDarwinCaBundle() {
  if (!isDarwin()) {
    return null;
  }

  if (cachedCa !== undefined) {
    return cachedCa;
  }

  try {
    const systemRoots = execSync(DARWIN_CERT_CMD, {
      encoding: 'utf8',
      timeout: 10_000,
      maxBuffer: 8 * 1024 * 1024,
    }).trim();
    cachedCa = systemRoots ? [...tls.rootCertificates, systemRoots].join('\n') : null;
  } catch {
    cachedCa = null;
  }

  return cachedCa;
}

function tlsOptions() {
  if (!isDarwin()) {
    return {};
  }

  if (cachedAgent !== undefined) {
    return cachedAgent ? { agent: cachedAgent } : {};
  }

  const ca = loadDarwinCaBundle();
  cachedAgent = ca ? new https.Agent({ ca }) : null;
  return cachedAgent ? { agent: cachedAgent } : {};
}

module.exports = {
  tlsOptions,
};
