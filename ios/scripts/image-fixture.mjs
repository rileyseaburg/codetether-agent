// Deterministic PNG test fixture: one white circle on a blue background.
import { deflateSync } from 'node:zlib';
import { writeFileSync } from 'node:fs';
function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ (crc & 1 ? 0xedb88320 : 0);
  }
  return (crc ^ 0xffffffff) >>> 0;
}
function chunk(type, data) {
  const name = Buffer.from(type), size = Buffer.alloc(4), checksum = Buffer.alloc(4);
  size.writeUInt32BE(data.length); checksum.writeUInt32BE(crc32(Buffer.concat([name, data])));
  return Buffer.concat([size, name, data, checksum]);
}
const width = 320, height = 240, pixels = Buffer.alloc((width * 3 + 1) * height);
for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
  const offset = y * (width * 3 + 1) + 1 + x * 3;
  const circle = (x - 160) ** 2 + (y - 120) ** 2 < 60 ** 2;
  pixels[offset] = circle ? 255 : 20; pixels[offset + 1] = circle ? 255 : 80; pixels[offset + 2] = circle ? 255 : 220;
}
const header = Buffer.alloc(13);
header.writeUInt32BE(width); header.writeUInt32BE(height, 4); header[8] = 8; header[9] = 2;
writeFileSync(process.argv[2], Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
  chunk('IHDR', header), chunk('IDAT', deflateSync(pixels)), chunk('IEND', Buffer.alloc(0))]));
console.log('Deterministic image fixture written.');
