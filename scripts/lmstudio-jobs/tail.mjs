// A UTF-8-bounded suffix includes the actual end, not a prefix of the last chunk.
/** @param {string} text @param {number} maximum */
export function tail(text, maximum) {
  let result = '', bytes = 0;
  for (const point of [...text].reverse()) {
    const size = Buffer.byteLength(point);
    if (bytes + size > maximum) break;
    result = point + result; bytes += size;
  }
  return result;
}
