import assert from "node:assert/strict";
import test from "node:test";
import { GoogleRpcDecoder } from "./rpc-framing.mjs";

/** @param {unknown[][]} frames @returns {Buffer} */
function batch(frames) {
  const json = JSON.stringify(frames);
  return Buffer.from(`${Buffer.byteLength(json) + 2}\n${json}\n`);
}

test("mocked local: RPC lengths include framing newlines and survive UTF-8 splits", () => {
  const frames = [["wrb.fr", null, "café 🦉"], ["e", 9, 11629]];
  const body = Buffer.concat([Buffer.from(")]}'\n\n"), batch(frames)]);
  for (let split = 0; split <= body.length; split++) {
    const decoder = new GoogleRpcDecoder();
    assert.deepEqual([...decoder.push(body.subarray(0, split)), ...decoder.push(body.subarray(split))], frames);
    decoder.end();
    assert.equal(decoder.batchCount, 1);
  }
});

test("mocked local: one-byte chunks and multiple RPC batches", () => {
  const decoder = new GoogleRpcDecoder();
  const body = Buffer.concat([Buffer.from(")]}'\r\n"), batch([["di"]]), batch([["e"]])]);
  /** @type {unknown[][]} */ const frames = [];
  for (const byte of body) frames.push(...decoder.push(Buffer.from([byte])));
  decoder.end();
  assert.deepEqual(frames, [["di"], ["e"]]);
  assert.equal(decoder.batchCount, 2);
});

test("mocked local: invalid RPC framing yields safe typed diagnostics", () => {
  const cases = [
    ["wrong\n", "missing_anti_xssi_prefix"], [")]}'\nx\n", "invalid_batch_length"],
    [")]}'\n0\n", "batch_length_limit"], [")]}'\n16777217\n", "batch_length_limit"],
    [")]}'\n3\n{\n", "invalid_batch_json"], [")]}'\n4\n{}\n", "invalid_batch_shape"],
  ];
  for (const [body, error] of cases) {
    assert.throws(() => new GoogleRpcDecoder().push(Buffer.from(body)), { message: error });
  }
  assert.throws(() => new GoogleRpcDecoder().push(Buffer.concat([
    Buffer.from(")]}'\n3\n"), Buffer.from([0xff, 10]),
  ])), { message: "invalid_batch_json" });
  for (const body of ["", ")]}'", ")]}'\n2", ")]}'\n20\n[]"]) {
    const decoder = new GoogleRpcDecoder();
    decoder.push(Buffer.from(body));
    assert.throws(() => decoder.end(), { message: "truncated_rpc_stream" });
  }
});