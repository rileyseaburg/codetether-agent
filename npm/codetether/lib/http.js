const fs = require('node:fs');
const https = require('node:https');
const { tlsOptions: defaultTlsOptions } = require('./darwin_tls');

const REDIRECT_STATUSES = new Set([301, 302, 303, 307, 308]);

function requestOptions(headers, tlsOptionsFn) {
  return {
    ...tlsOptionsFn(),
    headers: {
      'User-Agent': 'codetether-npx',
      Accept: '*/*',
      ...headers,
    },
  };
}

function resolveRedirectUrl(currentUrl, location) {
  return new URL(location, currentUrl).toString();
}

function getWithRedirects(url, headers, handleResponse, deps = {}) {
  const httpsGet = deps.httpsGet || https.get;
  const tlsOptions = deps.tlsOptions || defaultTlsOptions;

  return new Promise((resolve, reject) => {
    const doGet = (currentUrl, redirectsLeft) => {
      httpsGet(currentUrl, requestOptions(headers, tlsOptions), (res) => {
        if (res.statusCode && REDIRECT_STATUSES.has(res.statusCode)) {
          const loc = res.headers.location;
          if (!loc) {
            reject(new Error(`Redirect without location from ${currentUrl}`));
            return;
          }
          if (redirectsLeft <= 0) {
            reject(new Error(`Too many redirects downloading ${url}`));
            return;
          }
          res.resume();
          doGet(resolveRedirectUrl(currentUrl, loc), redirectsLeft - 1);
          return;
        }

        Promise.resolve(handleResponse(res, currentUrl)).then(resolve, reject);
      }).on('error', reject);
    };

    doGet(url, 8);
  });
}

function requestJson(url, headers = {}, deps = {}) {
  return getWithRedirects(
    url,
    {
      Accept: 'application/vnd.github+json',
      ...headers,
    },
    (res, currentUrl) =>
      new Promise((resolve, reject) => {
        const chunks = [];
        res.on('data', (d) => chunks.push(d));
        res.on('end', () => {
          const body = Buffer.concat(chunks).toString('utf8');
          if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
            try {
              resolve(JSON.parse(body));
            } catch (e) {
              reject(new Error(`Failed to parse JSON from ${currentUrl}: ${e.message}`));
            }
            return;
          }
          reject(new Error(`HTTP ${res.statusCode} fetching ${currentUrl}: ${body.slice(0, 400)}`));
        });
      }),
    deps
  );
}

function downloadText(url, deps = {}) {
  return getWithRedirects(
    url,
    {},
    (res, currentUrl) =>
      new Promise((resolve, reject) => {
        const chunks = [];
        res.on('data', (d) => chunks.push(d));
        res.on('end', () => {
          const body = Buffer.concat(chunks).toString('utf8');
          if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
            resolve(body);
            return;
          }
          const err = new Error(`HTTP ${res.statusCode} downloading ${currentUrl}: ${body.slice(0, 400)}`);
          err.statusCode = res.statusCode;
          reject(err);
        });
      }),
    deps
  );
}

function downloadFile(url, destPath, deps = {}) {
  return getWithRedirects(
    url,
    {},
    (res, currentUrl) =>
      new Promise((resolve, reject) => {
        if (!res.statusCode || res.statusCode < 200 || res.statusCode >= 300) {
          const chunks = [];
          res.on('data', (d) => chunks.push(d));
          res.on('end', () => {
            const body = Buffer.concat(chunks).toString('utf8');
            const err = new Error(`HTTP ${res.statusCode} downloading ${currentUrl}: ${body.slice(0, 400)}`);
            err.statusCode = res.statusCode;
            reject(err);
          });
          return;
        }

        const out = fs.createWriteStream(destPath);
        res.pipe(out);
        out.on('finish', () => out.close(resolve));
        out.on('error', reject);
      }),
    deps
  );
}

module.exports = {
  downloadFile,
  downloadText,
  requestJson,
};
