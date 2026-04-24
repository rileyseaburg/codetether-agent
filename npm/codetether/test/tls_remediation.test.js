const test = require('node:test');
const assert = require('node:assert/strict');

const { isTlsCertificateError, tlsRemediationFor } = require('../lib/tls_remediation');

test('detects TLS issuer failures from error code', () => {
  const err = new Error('certificate download failed');
  err.code = 'UNABLE_TO_GET_ISSUER_CERT_LOCALLY';

  assert.equal(isTlsCertificateError(err), true);
});

test('explains safe TLS remediation', () => {
  const err = new Error('unable to get local issuer certificate');
  const help = tlsRemediationFor(err).join('\n');

  assert.match(help, /Recommended for this install only/);
  assert.match(help, /NODE_EXTRA_CA_CERTS/);
  assert.match(help, /Optional persistent npm setting/);
  assert.match(help, /npm config delete cafile/);
  assert.match(help, /Do not disable TLS verification/);
});
