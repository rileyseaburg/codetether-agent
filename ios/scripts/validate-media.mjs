import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const directory = process.argv[2];
const image = readFileSync(join(directory, 'fixture.png'));
const token = execFileSync('vault', ['kv', 'get', '-field=token', 'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const base = 'https://server.codetether.run';
const headers = { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' };
const anonymous = await fetch(`${base}/mobile/attachments`, { method: 'POST', body: '{}' });
if (anonymous.status !== 401) throw new Error(`Anonymous upload returned ${anonymous.status}`);
const response = await fetch(`${base}/mobile/attachments`, { method: 'POST', headers,
  body: JSON.stringify({ data: image.toString('base64') }) });
if (!response.ok) throw new Error(`Upload returned ${response.status}`);
const result = await response.json();
const download = await fetch(`${base}/mobile/image?path=${encodeURIComponent(result.path)}`, { headers });
const bytes = Buffer.from(await download.arrayBuffer());
if (!download.ok || !image.equals(bytes)) throw new Error('Image bytes changed in transit');
const forbidden = await fetch(`${base}/mobile/image?path=${encodeURIComponent('/etc/passwd')}`, { headers });
if (forbidden.status !== 404) throw new Error(`Unapproved file access returned ${forbidden.status}`);
writeFileSync(join(directory, 'media-proof.json'), JSON.stringify({
  anonymousStatus: anonymous.status, uploadStatus: response.status, downloadStatus: download.status,
  forbiddenStatus: forbidden.status, path: result.path, bytes: bytes.length, exactBytes: true
}, null, 2));
writeFileSync(join(directory, 'uploaded-path.txt'), result.path);
console.log(JSON.stringify({ ...result, anonymous: 401, uploaded: 200, downloaded: 200, fileIsolation: 404, exactBytes: true }));
