const fs = require('node:fs');
const fsp = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const https = require('node:https');
const crypto = require('node:crypto');

function repoFromEnv() {
  return process.env.CODETETHER_GITHUB_REPO || 'rileyseaburg/codetether-agent';
}

function pkgRoot() {
  return path.resolve(__dirname, '..');
}

function readPkgVersion() {
  // eslint-disable-next-line global-require
  const pkg = require(path.join(pkgRoot(), 'package.json'));
  return pkg.version;
}

function normalizeTag(tagOrVersion) {
  if (!tagOrVersion) return null;
  return tagOrVersion.startsWith('v') ? tagOrVersion : `v${tagOrVersion}`;
}

function platformTriple() {
  const arch = process.arch;
  const plat = process.platform;

  let archStr;
  switch (arch) {
    case 'x64':
      archStr = 'x86_64';
      break;
    case 'arm64':
      archStr = 'aarch64';
      break;
    default:
      throw new Error(`Unsupported CPU architecture: ${arch}`);
  }

  let osStr;
  switch (plat) {
    case 'linux':
      osStr = 'unknown-linux-gnu';
      break;
    case 'darwin':
      osStr = 'apple-darwin';
      break;
    case 'win32':
      osStr = 'pc-windows-msvc';
      break;
    default:
      throw new Error(`Unsupported OS platform: ${plat}`);
  }

  return `${archStr}-${osStr}`;
}

function isWindows() {
  return process.platform === 'win32';
}

function defaultCacheDir() {
  if (process.env.CODETETHER_NPX_CACHE_DIR) {
    return process.env.CODETETHER_NPX_CACHE_DIR;
  }

  if (isWindows()) {
    const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local');
    return path.join(base, 'codetether-npx');
  }

  const base = process.env.XDG_CACHE_HOME || path.join(os.homedir(), '.cache');
  return path.join(base, 'codetether-npx');
}

function installDirFor({ tag, targetTriple }) {
  return path.join(defaultCacheDir(), tag, targetTriple);
}

function binDestPath({ tag, targetTriple }) {
  const binName = isWindows() ? 'codetether.exe' : 'codetether';
  return path.join(installDirFor({ tag, targetTriple }), binName);
}

function fileExists(p) {
  try {
    fs.accessSync(p, fs.constants.F_OK);
    return true;
  } catch {
    return false;
  }
}

function canExecute(p) {
  try {
    if (isWindows()) {
      fs.accessSync(p, fs.constants.F_OK);
    } else {
      fs.accessSync(p, fs.constants.X_OK);
    }
    return true;
  } catch {
    return false;
  }
}

function requestJson(url, headers = {}) {
  return new Promise((resolve, reject) => {
    https
      .get(
        url,
        {
          headers: {
            'User-Agent': 'codetether-npx',
            Accept: 'application/vnd.github+json',
            ...headers,
          },
        },
        (res) => {
          const chunks = [];
          res.on('data', (d) => chunks.push(d));
          res.on('end', () => {
            const body = Buffer.concat(chunks).toString('utf8');
            if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
              try {
                resolve(JSON.parse(body));
              } catch (e) {
                reject(new Error(`Failed to parse JSON from ${url}: ${e.message}`));
              }
              return;
            }
            reject(new Error(`HTTP ${res.statusCode} fetching ${url}: ${body.slice(0, 400)}`));
          });
        }
      )
      .on('error', reject);
  });
}

async function getLatestReleaseTag(repo) {
  const data = await requestJson(`https://api.github.com/repos/${repo}/releases/latest`);
  if (!data || !data.tag_name) {
    throw new Error('GitHub API response missing tag_name');
  }
  return data.tag_name;
}

function downloadFile(url, destPath) {
  return new Promise((resolve, reject) => {
    const doGet = (u, redirectsLeft) => {
      https
        .get(
          u,
          {
            headers: {
              'User-Agent': 'codetether-npx',
              Accept: '*/*',
            },
          },
          (res) => {
            // Follow redirects (GitHub assets redirect to S3)
            if (res.statusCode && [301, 302, 303, 307, 308].includes(res.statusCode)) {
              const loc = res.headers.location;
              if (!loc) {
                reject(new Error(`Redirect without location from ${u}`));
                return;
              }
              if (redirectsLeft <= 0) {
                reject(new Error(`Too many redirects downloading ${url}`));
                return;
              }
              res.resume();
              doGet(loc, redirectsLeft - 1);
              return;
            }

            if (!res.statusCode || res.statusCode < 200 || res.statusCode >= 300) {
              const chunks = [];
              res.on('data', (d) => chunks.push(d));
              res.on('end', () => {
                const body = Buffer.concat(chunks).toString('utf8');
                const err = new Error(`HTTP ${res.statusCode} downloading ${u}: ${body.slice(0, 400)}`);
                err.statusCode = res.statusCode;
                reject(err);
              });
              return;
            }

            const out = fs.createWriteStream(destPath);
            res.pipe(out);
            out.on('finish', () => out.close(resolve));
            out.on('error', reject);
          }
        )
        .on('error', reject);
    };

    doGet(url, 8);
  });
}

function downloadText(url) {
  return new Promise((resolve, reject) => {
    const doGet = (u, redirectsLeft) => {
      https
        .get(
          u,
          {
            headers: {
              'User-Agent': 'codetether-npx',
              Accept: '*/*',
            },
          },
          (res) => {
            if (res.statusCode && [301, 302, 303, 307, 308].includes(res.statusCode)) {
              const loc = res.headers.location;
              if (!loc) {
                reject(new Error(`Redirect without location from ${u}`));
                return;
              }
              if (redirectsLeft <= 0) {
                reject(new Error(`Too many redirects downloading ${url}`));
                return;
              }
              res.resume();
              doGet(loc, redirectsLeft - 1);
              return;
            }

            const chunks = [];
            res.on('data', (d) => chunks.push(d));
            res.on('end', () => {
              const body = Buffer.concat(chunks).toString('utf8');
              if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
                resolve(body);
                return;
              }
              const err = new Error(`HTTP ${res.statusCode} downloading ${u}: ${body.slice(0, 400)}`);
              err.statusCode = res.statusCode;
              reject(err);
            });
          }
        )
        .on('error', reject);
    };

    doGet(url, 8);
  });
}

function rmdirRecursiveSafe(p) {
  try {
    fs.rmSync(p, { recursive: true, force: true });
  } catch {
    // ignore
  }
}

function listFilesRecursive(dir) {
  const out = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const e of entries) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) {
      out.push(...listFilesRecursive(p));
    } else {
      out.push(p);
    }
  }
  return out;
}

async function extractTarGz(tarModule, tarPath, cwd) {
  await tarModule.x({ file: tarPath, cwd });
}

function extractZip(AdmZip, zipPath, cwd) {
  const zip = new AdmZip(zipPath);
  zip.extractAllTo(cwd, true);
}

function sha256OfFile(p) {
  const hash = crypto.createHash('sha256');
  const data = fs.readFileSync(p);
  hash.update(data);
  return hash.digest('hex');
}

function parseSha256Sums(text) {
  const map = new Map();
  const lines = String(text || '').split(/\r?\n/);
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const m = trimmed.match(/^([a-fA-F0-9]{64})\s+\*?(.+)$/);
    if (!m) continue;
    const hash = m[1].toLowerCase();
    const file = m[2].trim();
    map.set(file, hash);
  }
  return map;
}

async function verifyArchiveChecksum({ repo, tag, archiveName, archivePath }) {
  if (process.env.CODETETHER_NPX_NO_CHECKSUM) {
    return;
  }

  const sumsName = `SHA256SUMS-${tag}.txt`;
  const sumsUrl = `https://github.com/${repo}/releases/download/${tag}/${sumsName}`;

  let sumsText;
  try {
    sumsText = await downloadText(sumsUrl);
  } catch (e) {
    // Backward compatible: older releases may not have checksums.
    if (e && e.statusCode === 404) return;
    throw e;
  }

  const sums = parseSha256Sums(sumsText);
  const expected = sums.get(archiveName);
  if (!expected) {
    // If the manifest doesn't include the archive, don't block installs.
    return;
  }

  const actual = sha256OfFile(archivePath);
  if (actual !== expected) {
    throw new Error(
      [
        `Checksum mismatch for ${archiveName}`,
        `expected: ${expected}`,
        `actual:   ${actual}`,
        '',
        'If you need to bypass checksum verification, set CODETETHER_NPX_NO_CHECKSUM=1',
      ].join('\n')
    );
  }
}

async function withInstallLock(lockPath, fn) {
  const started = Date.now();
  const timeoutMs = 2 * 60 * 1000;
  const staleMs = 10 * 60 * 1000;

  // eslint-disable-next-line no-constant-condition
  while (true) {
    try {
      const fh = await fsp.open(lockPath, 'wx');
      try {
        return await fn();
      } finally {
        try {
          await fh.close();
        } catch {
          // ignore
        }
        try {
          await fsp.unlink(lockPath);
        } catch {
          // ignore
        }
      }
    } catch (e) {
      if (!e || e.code !== 'EEXIST') throw e;

      // If lock is stale, break it.
      try {
        const st = await fsp.stat(lockPath);
        if (Date.now() - st.mtimeMs > staleMs) {
          await fsp.unlink(lockPath);
          continue;
        }
      } catch {
        // ignore
      }

      if (Date.now() - started > timeoutMs) {
        throw new Error('Timed out waiting for codetether install lock');
      }

      await new Promise((r) => setTimeout(r, 250));
    }
  }
}

async function installForTag({ repo, tag, targetTriple }) {
  const assetBase = `codetether-${tag}-${targetTriple}`;
  const destPath = binDestPath({ tag, targetTriple });
  const destDir = path.dirname(destPath);

  await fsp.mkdir(destDir, { recursive: true });

  if (canExecute(destPath)) {
    return { destPath, tag, targetTriple, reused: true };
  }

  const lockPath = path.join(destDir, 'install.lock');
  return withInstallLock(lockPath, async () => {
    if (canExecute(destPath)) {
      return { destPath, tag, targetTriple, reused: true };
    }

    const tmpDir = await fsp.mkdtemp(path.join(os.tmpdir(), 'codetether-npx-'));
    try {
      const isWin = isWindows();
      const archiveName = isWin ? `${assetBase}.zip` : `${assetBase}.tar.gz`;
      const url = `https://github.com/${repo}/releases/download/${tag}/${archiveName}`;
      const archivePath = path.join(tmpDir, archiveName);

      await downloadFile(url, archivePath);
      await verifyArchiveChecksum({ repo, tag, archiveName, archivePath });

      if (isWin) {
        // eslint-disable-next-line global-require
        const AdmZip = require('adm-zip');
        extractZip(AdmZip, archivePath, tmpDir);

        const files = listFilesRecursive(tmpDir);
        const candidate1 = files.find((p) => path.basename(p).toLowerCase() === 'codetether.exe');
        const candidate2 = files.find((p) => path.basename(p).toLowerCase() === `${assetBase}.exe`.toLowerCase());
        const found = candidate1 || candidate2;
        if (!found) {
          throw new Error(`Could not find codetether.exe or ${assetBase}.exe inside ${archiveName}`);
        }

        await fsp.copyFile(found, destPath);
      } else {
        // eslint-disable-next-line global-require
        const tar = require('tar');
        await extractTarGz(tar, archivePath, tmpDir);

        const extracted = path.join(tmpDir, assetBase);
        if (!fileExists(extracted)) {
          throw new Error(`Expected extracted binary not found: ${assetBase}`);
        }

        await fsp.copyFile(extracted, destPath);
        await fsp.chmod(destPath, 0o755);
      }

      // Write a small marker file for debugging.
      const meta = {
        repo,
        tag,
        targetTriple,
        installedAt: new Date().toISOString(),
        sha256: sha256OfFile(destPath),
      };
      await fsp.writeFile(path.join(destDir, 'install-meta.json'), `${JSON.stringify(meta, null, 2)}\n`, 'utf8');

      return { destPath, tag, targetTriple, reused: false };
    } finally {
      rmdirRecursiveSafe(tmpDir);
    }
  });
}

async function ensureInstalled({ allowLatestFallback = true } = {}) {
  const repo = repoFromEnv();
  const targetTriple = platformTriple();

  const explicitTag = normalizeTag(process.env.CODETETHER_TAG) || normalizeTag(process.env.CODETETHER_VERSION);
  const pkgTag = normalizeTag(readPkgVersion());

  const tagsToTry = [];
  if (explicitTag) tagsToTry.push(explicitTag);
  if (pkgTag && pkgTag !== explicitTag) tagsToTry.push(pkgTag);

  if (process.env.CODETETHER_NPX_SKIP_DOWNLOAD) {
    for (const tag of tagsToTry) {
      const p = binDestPath({ tag, targetTriple });
      if (canExecute(p)) {
        return { destPath: p, repo, tag, targetTriple, reused: true };
      }
    }
    const err = new Error(
      [
        'CODETETHER_NPX_SKIP_DOWNLOAD is set, but no cached codetether binary was found.',
        `repo: ${repo}`,
        `platform: ${process.platform} (${process.arch}) => ${targetTriple}`,
      ].join('\n')
    );
    throw err;
  }

  let lastErr = null;
  for (const tag of tagsToTry) {
    try {
      const p = binDestPath({ tag, targetTriple });
      if (canExecute(p)) {
        return { destPath: p, repo, tag, targetTriple, reused: true };
      }
      return await installForTag({ repo, tag, targetTriple });
    } catch (e) {
      lastErr = e;
      // continue
    }
  }

  if (allowLatestFallback) {
    try {
      const latestTag = await getLatestReleaseTag(repo);
      if (!tagsToTry.includes(latestTag)) {
        const p = binDestPath({ tag: latestTag, targetTriple });
        if (canExecute(p)) {
          return { destPath: p, repo, tag: latestTag, targetTriple, reused: true };
        }
        return await installForTag({ repo, tag: latestTag, targetTriple });
      }
    } catch (e) {
      lastErr = lastErr || e;
    }
  }

  const help = [
    'Failed to install codetether binary via GitHub Releases.',
    `repo: ${repo}`,
    `platform: ${process.platform} (${process.arch}) => ${targetTriple}`,
    '',
    'You can still install via the official scripts:',
    '  Linux/macOS: curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh',
    '  Windows:     irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 | iex',
    '',
    'Or build from source:',
    '  cargo install codetether-agent',
  ].join('\n');

  const err = new Error(`${help}\n\nUnderlying error: ${lastErr ? lastErr.message : 'unknown'}`);
  err.cause = lastErr;
  throw err;
}

module.exports = {
  ensureInstalled,
  platformTriple,
  normalizeTag,
};
