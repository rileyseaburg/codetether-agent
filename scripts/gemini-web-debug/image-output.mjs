import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";

/** Create a fresh private directory; never overwrite prior evidence.
 * Parent directories must exist.
 * @param {string} directory @returns {(name: string, value: Record<string, unknown>) => void} */
export function imageOutput(directory) {
  mkdirSync(directory, { mode: 0o700 });
  return (name, value) => {
    writeFileSync(join(directory, `${name}.json`), `${JSON.stringify(value, null, 2)}\n`, { mode: 0o600, flag: "wx" });
  };
}

/** Persist checked bytes with a generated, non-remote filename.
 * @param {string} directory @param {number} index
 * @param {Awaited<ReturnType<typeof import("./image-download.mjs").downloadImage>>} image
 * @returns {{path: string, sha256: string, bytes: number, width: number, height: number}} */
export function saveImage(directory, index, image) {
  const path = join(directory, `image-${index + 1}.${image.extension}`);
  writeFileSync(path, image.body, { mode: 0o600, flag: "wx" });
  return { path, sha256: createHash("sha256").update(image.body).digest("hex"), bytes: image.body.length, width: image.width, height: image.height };
}