import { mkdir, writeFile, readFile, realpath, stat } from 'node:fs/promises';
import { randomUUID } from 'node:crypto';
import { join, sep } from 'node:path';
import { homedir } from 'node:os';
import { HTTPError } from './auth.mjs';

const uploads = join(homedir(), '.local/share/codetether-mobile/uploads');
const generated = join(homedir(), 'spotlessbinco/.codetether-agent/generated_images');
export async function upload(request) {
  const chunks = [];
  let size = 0;
  for await (const chunk of request) {
    size += chunk.length;
    if (size > 6 * 1024 * 1024) throw new HTTPError(413, 'Image is too large');
    chunks.push(chunk);
  }
  let payload;
  try { payload = JSON.parse(Buffer.concat(chunks).toString()); }
  catch { throw new HTTPError(400, 'Invalid image request'); }
  if (typeof payload.data !== 'string' || !/^[A-Za-z0-9+/]+=*$/.test(payload.data)) throw new HTTPError(400, 'Invalid image encoding');
  const bytes = Buffer.from(payload.data, 'base64');
  const png = bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]));
  const jpeg = bytes[0] === 255 && bytes[1] === 216 && bytes[2] === 255;
  if ((!png && !jpeg) || bytes.length > 4 * 1024 * 1024) throw new HTTPError(400, 'A PNG or JPEG under 4 MiB is required');
  await mkdir(uploads, { recursive: true, mode: 0o700 });
  const path = join(uploads, `${randomUUID()}.${png ? 'png' : 'jpg'}`);
  await writeFile(path, bytes, { mode: 0o600 });
  return { path };
}
export async function image(path) {
  if (!path || path.length > 1024) throw new HTTPError(404, 'Image not found');
  let actual;
  try { actual = await realpath(path); } catch { throw new HTTPError(404, 'Image not found'); }
  if (![uploads, generated].some(root => actual.startsWith(root + sep))) throw new HTTPError(404, 'Image not found');
  if (!/\.(png|jpe?g)$/i.test(actual)) throw new HTTPError(404, 'Image not found');
  const info = await stat(actual);
  if (!info.isFile() || info.size > 16 * 1024 * 1024) throw new HTTPError(404, 'Image not found');
  return { data: await readFile(actual), type: actual.endsWith('.png') ? 'image/png' : 'image/jpeg' };
}
export async function serveMedia(request, response) {
  const uploading = request.method === 'POST';
  const value = uploading ? await upload(request) : await image(new URL(request.url, 'http://localhost').searchParams.get('path'));
  response.writeHead(200, { 'Content-Type': uploading ? 'application/json' : value.type, 'Cache-Control': 'no-store', 'X-Content-Type-Options': 'nosniff' });
  response.end(uploading ? JSON.stringify(value) : value.data);
}
