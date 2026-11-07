/**
 * Parses one byte range into inclusive bounds, clamped to the file size.
 * Missing headers mean full-file delivery; invalid ranges throw RangeError.
 * @param {string | undefined} header The HTTP Range header.
 * @param {number} size Full asset length in bytes.
 * @returns {{start: number, end: number} | null} Bounds or full-file selection.
 * @example
 * import { byteRange } from './byte-range.mjs';
 * const bounds = byteRange('bytes=0-31', 100);
 * console.assert(bounds.start === 0 && bounds.end === 31);
 */
export function byteRange(header, size) {
  if (header === undefined) return null;
  const match = /^bytes=(\d*)-(\d*)$/.exec(header);
  if (!match || (!match[1] && !match[2]) || size === 0) {
    throw new RangeError('Unsupported or empty byte range');
  }
  let start = Number(match[1]);
  let end = match[2] ? Number(match[2]) : size - 1;
  if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end)) {
    throw new RangeError('Unsafe byte range');
  }
  if (!match[1]) {
    if (end === 0) throw new RangeError('Empty suffix');
    start = Math.max(0, size - end);
    end = size - 1;
  }
  if (start >= size || end < start) throw new RangeError('Unsatisfiable byte range');
  return { start, end: Math.min(end, size - 1) };
}
