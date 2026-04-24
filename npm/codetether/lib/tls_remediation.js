const CERT_ERROR_PATTERNS = [
  /unable to get local issuer certificate/i,
  /self[ -]?signed certificate/i,
  /unable to verify the first certificate/i,
  /certificate has expired/i,
  /\bUNABLE_TO_GET_ISSUER_CERT\b/i,
  /\bSELF_SIGNED_CERT_IN_CHAIN\b/i,
];

function isTlsCertificateError(err) {
  const text = err && (err.stack || err.message || String(err));
  return CERT_ERROR_PATTERNS.some((pattern) => pattern.test(text || ''));
}

function tlsRemediationFor(err) {
  if (!isTlsCertificateError(err)) {
    return [];
  }

  return [
    '',
    'TLS certificate verification failed while downloading from GitHub Releases.',
    'If you are behind corporate TLS interception or a custom CA, export the',
    'issuer certificate as PEM and retry with one of:',
    '  NODE_EXTRA_CA_CERTS=/path/to/issuer.pem npx codetether',
    '  npm config set cafile /path/to/issuer.pem',
    '',
    'Do not disable TLS verification. The installer will keep refusing',
    'untrusted release downloads until Node can validate the issuer.',
  ];
}

module.exports = {
  isTlsCertificateError,
  tlsRemediationFor,
};
