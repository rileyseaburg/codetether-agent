/** Records only numeric framing diagnostics, never payload or token contents.
 * @param {Buffer} body
 * @returns {Array<{declared: number, utf8Bytes: number, utf16Units: number, codePoints: number}>} */
export function rpcLengths(body) {
  const lines = body.toString("utf8").split("\n");
  /** @type {Array<{declared: number, utf8Bytes: number, utf16Units: number, codePoints: number}>} */
  const result = [];
  for (let index = 0; index < lines.length - 1; index++) {
    const marker = lines[index].trim();
    if (!/^\d+$/.test(marker)) continue;
    const payload = lines[index + 1];
    try {
      const value = JSON.parse(payload);
      if (!Array.isArray(value)) continue;
    } catch { continue; }
    result.push({
      declared: Number(marker),
      utf8Bytes: Buffer.byteLength(payload),
      utf16Units: payload.length,
      codePoints: [...payload].length,
    });
  }
  return result;
}