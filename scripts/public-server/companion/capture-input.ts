import { HTTPError, record } from './types.ts';
import type { Capture } from './types.ts';

/** Validate JPEG dimensions without decoding or saving the screenshot. */
function boundedJPEG(bytes: Buffer): boolean {
  if (bytes.readUInt16BE(0) !== 0xffd8 || bytes.readUInt16BE(bytes.length - 2) !== 0xffd9) return false;
  let offset = 2;
  while (offset + 9 < bytes.length) {
    if (bytes[offset] !== 255) return false;
    const marker = bytes[offset + 1];
    const size = bytes.readUInt16BE(offset + 2);
    if (size < 2 || offset + 2 + size > bytes.length) return false;
    if (marker === 0xc0 || marker === 0xc2) {
      const height = bytes.readUInt16BE(offset + 5), width = bytes.readUInt16BE(offset + 7);
      return width > 0 && height > 0 && width <= 1920 && height <= 1920;
    }
    offset += size + 2;
  }
  return false;
}
export function captureInput(value: unknown, now = Date.now()): Capture {
  if (!record(value) || typeof value.image !== 'string' || value.image.length > 700000
    || !/^[A-Za-z0-9+/]+={0,2}$/.test(value.image) || typeof value.captured_at !== 'string') {
    throw new HTTPError(400, 'Expected a bounded JPEG screenshot and capture time');
  }
  const time = Date.parse(value.captured_at), bytes = Buffer.from(value.image, 'base64');
  if (!Number.isFinite(time) || now - time > 300000 || time - now > 60000 || bytes.length < 24
    || bytes.length > 512 * 1024 || bytes.toString('base64') !== value.image || !boundedJPEG(bytes)) {
    throw new HTTPError(400, 'Screenshot must be a recent JPEG under 512 KiB and 1920px');
  }
  const trigger = value.trigger, id = value.request_id;
  if (trigger !== undefined && trigger !== 'periodic' && trigger !== 'right_click'
    && trigger !== 'double_click' && trigger !== 'manual') throw new HTTPError(400, 'Invalid capture trigger');
  if (id !== undefined && (typeof id !== 'string' || !/^[a-f0-9-]{36}$/.test(id))) {
    throw new HTTPError(400, 'Invalid capture request');
  }
  return { image: value.image, captured_at: new Date(time).toISOString(),
    ...(trigger ? { trigger } : {}), ...(id ? { request_id: id } : {}) };
}