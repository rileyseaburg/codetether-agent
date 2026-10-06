import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const out = process.argv[2];
const tools = JSON.parse(readFileSync(join(out, 'tools.json'), 'utf8'));
const path = tools.find(tool => tool.name === 'image_gen')?.output?.match(/Generated image saved to (\/[^\n]+\.png)/)?.[1];
if (!path) throw new Error('No successful generated image artifact recorded');
const token = execFileSync('vault', ['kv', 'get', '-field=token', 'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const response = await fetch(`https://server.codetether.run/mobile/image?path=${encodeURIComponent(path)}`,
  { headers: { Authorization: `Bearer ${token}` }, redirect: 'error' });
if (!response.ok) throw new Error(`Generated image download returned ${response.status}`);
const data = Buffer.from(await response.arrayBuffer());
if (data.toString('ascii', 1, 4) !== 'PNG') throw new Error('Invalid PNG');
writeFileSync(join(out, 'generated-image.png'), data);
writeFileSync(join(out, 'image-download.json'), JSON.stringify({ path, status: response.status, bytes: data.length }));
console.log(JSON.stringify({ status: response.status, bytes: data.length, artifact: join(out, 'generated-image.png') }));
