import { ImageError } from "./image-error.mjs";

/** Check MIME, signature, dimensions and end marker before writing images.
 * This is structural validation, not a full pixel decode.
 * @param {Buffer} body @param {string} mime
 * @returns {{extension: string, width: number, height: number}} */
export function imageInfo(body, mime) {
  if (!body.length || body.length > 32 * 1024 * 1024) throw new ImageError("image_size_invalid");
  let width = 0, height = 0, extension = "";
  if (mime === "image/png" && body.length >= 45 && body.subarray(0, 8).toString("hex") === "89504e470d0a1a0a" &&
      body.subarray(12, 16).toString() === "IHDR" && body.subarray(-8, -4).toString() === "IEND") {
    width = body.readUInt32BE(16); height = body.readUInt32BE(20); extension = "png";
  } else if (mime === "image/jpeg" && body.length > 12 && body.subarray(0, 2).toString("hex") === "ffd8" &&
      body.subarray(-2).toString("hex") === "ffd9") {
    for (let p = 2; p + 4 < body.length;) {
      if (body[p] !== 0xff) break;
      while (body[p] === 0xff) p++;
      const marker = body[p++];
      if (marker === 0xda || marker === 0xd9 || p + 2 > body.length) break;
      const size = body.readUInt16BE(p);
      if (size < 2 || p + size > body.length) break;
      if ([0xc0, 0xc1, 0xc2, 0xc3, 0xc5, 0xc6, 0xc7, 0xc9, 0xca, 0xcb, 0xcd, 0xce, 0xcf].includes(marker) && size >= 8) {
        height = body.readUInt16BE(p + 3); width = body.readUInt16BE(p + 5); extension = "jpg"; break;
      }
      p += size;
    }
  }
  if (!extension || !width || !height || width * height > 100_000_000) throw new ImageError("image_content_invalid");
  return { extension, width, height };
}