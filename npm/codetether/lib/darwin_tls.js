const tls = require('node:tls');
const https = require('node:https');
const { execSync } = require('node:child_process');

const isDarwin = process.platform === 'darwin';

let cachedAgent = null;

function loadDarwinRootCerts() {
  if (!isDarwin) {
    return null;
  }

  try {
    const pem = execSync(
      'security find-certificate -a -p /Library/Keychains/System.keychain ' +
        '/System/Library/Keychains/SystemRootCertificates.keychain',
      { encoding: 'utf8', timeout: 10000 }
    );
    return pem;
  } catch {
    return null;
  }
}

function macOsHttpsAgent() {
  if (!isDarwin) {
    return undefined;
  }

  if (cachedAgent) {
    return cachedAgent;
  }

  const rootCerts = loadDarwinRootCerts();
  if (!rootCerts) {
    return undefined;
  }

  cachedAgent = new https.Agent({
    ca: rootCerts,
    keepAlive: true,
  });

  return cachedAgent;
}

function tlsOptions() {
  if (!isDarwin) {
    return {};
  }

  const agent = macOsHttpsAgent();
  if (agent) {
    return { agent };
  }

  const certs = loadDarwinRootCerts();
  if (certs) {
    return { ca: certs };
  }

  return {};
}

module.exports = { macOsHttpsAgent, tlsOptions, loadDarwinRootCerts };
