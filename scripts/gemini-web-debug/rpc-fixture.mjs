/** Builds a complete proprietary RPC response for mocked local tests.
 * @param {string} answer @param {unknown[][]} extra
 * @returns {import("./curl-wire.mjs").CurlResult} */
export function fixture(answer = "RPC_CLIENT_OK 1591", extra = []) {
  const inner = [null, null, null, null, [[null, [answer]]]];
  const json = JSON.stringify([["wrb.fr", null, JSON.stringify(inner)], ...extra]);
  return { exitCode: 0, body: Buffer.from(`)]}'\n${Buffer.byteLength(json) + 2}\n${json}\n`),
    headers: "", chunks: [], metrics: { http_code: 200, http_version: "2", content_type: "application/json" } };
}

/** @param {unknown[][]} frames @returns {Buffer} */
export function framed(frames) {
  const json = JSON.stringify(frames);
  return Buffer.from(`)]}'\n${Buffer.byteLength(json) + 2}\n${json}\n`);
}