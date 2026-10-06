// One intentional send; an exclusive receipt prevents accidental duplicate runs.
import { writeFile, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { sendgrid, configuredSender } from './sendgrid-client.mjs';
import { verifyDownload } from './verify-download.mjs';
import { releaseConfig } from './release-config.mjs';
const [directory, version, build, sha256] = process.argv.slice(2);
if (!directory) throw new Error('Usage: email-download.mjs EVIDENCE VERSION BUILD SHA256');
const { base } = releaseConfig(version, build, sha256);
const downloadURL = `${base}/index.html`;
await verifyDownload(directory, version, build, sha256);
const path = join(directory, 'sendgrid-receipt.json');
const recipient = 'riley@spotlessbinco.com';
const sender = configuredSender || 'noreply@codetether.run';
const receipt = { validation: 'real platform submission', recipient, sender, url: downloadURL,
  attempted_at: new Date().toISOString(), state: 'pending' };
try { await writeFile(path, JSON.stringify(receipt, null, 2) + '\n', { flag: 'wx' }); }
catch (error) {
  if (error.code !== 'EEXIST') throw error;
  const previous = JSON.parse(await readFile(path, 'utf8'));
  console.log(JSON.stringify(previous));
  throw new Error('Receipt already exists; refusing duplicate email submission');
}
const text = `Hi Riley,\n\nYour CodeTether iOS ${version} (${build}) download is ready:\n${downloadURL}\n\nOpen the link in Safari on your registered iPhone and tap Install CodeTether. This Ad Hoc build requires iOS 17 or later and the registered device.\n\nThe HTTPS page, manifest, and IPA checksum were checked before sending. Installation, physical-camera capture, and playback still need on-device confirmation. No login or signing credentials are included.\n`;
try {
  const result = await sendgrid('mail/send', {
    personalizations: [{ to: [{ email: recipient }] }], from: { email: sender, name: 'CodeTether' },
    subject: `Your CodeTether iOS download — ${version} (${build})`, content: [{ type: 'text/plain', value: text }],
    tracking_settings: { click_tracking: { enable: false, enable_text: false }, open_tracking: { enable: false } }
  });
  Object.assign(receipt, { status: result.status, message_id: result.messageId,
    state: result.status === 202 ? 'accepted' : 'rejected', response: result.body });
} catch (error) {
  Object.assign(receipt, { state: 'unknown', diagnostic: error.message });
} finally {
  await writeFile(path, JSON.stringify(receipt, null, 2) + '\n');
}
console.log(JSON.stringify(receipt));
if (receipt.state !== 'accepted') process.exitCode = 1;