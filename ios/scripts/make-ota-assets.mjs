// Generate only credential-free, release-pinned installation assets.
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { releaseConfig } from './release-config.mjs';
const [ipa, out, version, build, expected] = process.argv.slice(2);
if (!ipa || !out) throw new Error('Usage: make-ota-assets.mjs IPA OUTPUT VERSION BUILD SHA256');
const bytes = readFileSync(ipa);
const sha256 = createHash('sha256').update(bytes).digest('hex');
const { base } = releaseConfig(version, build, expected);
if (sha256 !== expected) throw new Error('IPA checksum mismatch');
mkdirSync(out, { recursive: true, mode: 0o755 });
writeFileSync(join(out, 'CodeTether.ipa'), bytes);
writeFileSync(join(out, 'manifest.plist'), `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict><key>items</key><array><dict>
<key>assets</key><array><dict><key>kind</key><string>software-package</string>
<key>url</key><string>${base}/CodeTether.ipa</string></dict></array>
<key>metadata</key><dict><key>bundle-identifier</key><string>run.codetether.ios</string>
<key>bundle-version</key><string>${build}</string><key>kind</key><string>software</string>
<key>title</key><string>CodeTether</string></dict></dict></array></dict></plist>\n`);
const install = `itms-services://?action=download-manifest&url=${encodeURIComponent(`${base}/manifest.plist`)}`;
writeFileSync(join(out, 'index.html'), `<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Install CodeTether</title><body><h1>CodeTether ${version} (${build})</h1>
<p>Open this page in Safari on Riley’s registered iPhone. iOS 17 or newer required.</p>
<p><a href="${install.replaceAll('&', '&amp;')}">Install CodeTether</a></p>
<p>This Ad Hoc build installs only on devices included in its Apple provisioning profile.</p>
<p>Chat, voice mode, model selection, Markdown and copy controls. Existing Keychain login should be retained.</p>
<p>Installation, physical-camera capture, and playback still require on-device confirmation.</p>
<p><a href="CodeTether.ipa">Download IPA</a> · <a href="SHA256SUMS">Checksums</a></p>
</body></html>\n`);
writeFileSync(join(out, 'release.json'), JSON.stringify({ version, build,
  bundle_id: 'run.codetether.ios', sha256, bytes: bytes.length, install_page: `${base}/index.html` }, null, 2) + '\n');
const files = ['CodeTether.ipa', 'manifest.plist', 'index.html', 'release.json'];
writeFileSync(join(out, 'SHA256SUMS'), files.map(file =>
  `${createHash('sha256').update(readFileSync(join(out, file))).digest('hex')}  ${file}\n`).join(''));
console.log(JSON.stringify({ validation_level: 'static/local', directory: out,
  install_page: `${base}/index.html`, sha256, files: [...files, 'SHA256SUMS'] }));