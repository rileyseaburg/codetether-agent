import assert from "node:assert/strict";
import test from "node:test";
import { mkdtempSync, readFileSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { imageInfo } from "./image-file.mjs";
import { imageOutput, saveImage } from "./image-output.mjs";
import { png } from "./image-fixture.mjs";

test("mocked local: image validation rejects text, truncated and mismatched bytes", () => {
  assert.deepEqual(imageInfo(png, "image/png"), {extension: "png", width: 1, height: 1});
  assert.throws(() => imageInfo(png, "image/jpeg"), /image_content_invalid/);
  assert.throws(() => imageInfo(png.subarray(0, -12), "image/png"), /image_content_invalid/);
  assert.throws(() => imageInfo(Buffer.alloc(0), "image/jpeg"), /image_size_invalid/);
  const invalid = Buffer.from(png); invalid.writeUInt32BE(0, 16);
  assert.throws(() => imageInfo(invalid, "image/png"), /image_content_invalid/);
  // Synthetic JPEG marker layout tests structure only, not pixel decoding.
  const jpeg = Buffer.from("ffd8ffc0000b080002000301011100ffd9", "hex");
  assert.deepEqual(imageInfo(jpeg, "image/jpeg"), {extension: "jpg", width: 3, height: 2});
  assert.throws(() => imageInfo(jpeg.subarray(0, -2), "image/jpeg"), /image_content_invalid/);
});
test("mocked local: outputs are private, named locally, and never overwritten", () => {
  const root = mkdtempSync(join(tmpdir(), "gemini-image-test-"));
  const directory = join(root, "output"), save = imageOutput(directory);
  const record = saveImage(directory, 0, {body: png, ...imageInfo(png, "image/png")});
  save("summary", {images: [record]});
  assert.deepEqual(readFileSync(record.path), png);
  assert.equal(record.bytes, png.length); assert.equal(record.sha256.length, 64);
  assert.throws(() => imageOutput(directory));
  assert.throws(() => saveImage(directory, 0, {body: png, ...imageInfo(png, "image/png")}));
  if (process.platform !== "win32") {
    assert.equal(statSync(directory).mode & 0o777, 0o700);
    assert.equal(statSync(record.path).mode & 0o777, 0o600);
  }
});