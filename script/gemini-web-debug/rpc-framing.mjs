/** Incrementally decodes Google's anti-XSSI, byte-length-prefixed RPC batches.
 * Length includes the newline after its marker and the newline after the JSON.
 * Only decoded frame arrays escape; errors never contain response contents. */
export class GoogleRpcDecoder {
  /** @type {Buffer} */ buffer = Buffer.alloc(0);
  started = false;
  batchCount = 0;
  /** @param {Buffer} chunk @returns {unknown[][]} */
  push(chunk) {
    this.buffer = Buffer.concat([this.buffer, chunk]);
    /** @type {unknown[][]} */ const frames = [];
    if (!this.started) {
      if (this.buffer.length < 5) return frames;
      if (this.buffer.subarray(0, 4).toString() !== ")]}'") throw new Error("missing_anti_xssi_prefix");
      const newline = this.buffer.indexOf(10);
      if (newline < 0) return frames;
      this.buffer = this.buffer.subarray(newline + 1);
      this.started = true;
    }
    while (this.buffer.length) {
      let offset = 0;
      while ([9, 10, 13, 32].includes(this.buffer[offset])) offset++;
      this.buffer = this.buffer.subarray(offset);
      if (!this.buffer.length) break;
      const newline = this.buffer.indexOf(10);
      if (newline < 0) break;
      const lengthText = this.buffer.subarray(0, newline).toString().trim();
      if (!/^\d+$/.test(lengthText)) throw new Error("invalid_batch_length");
      const length = Number(lengthText);
      if (!Number.isSafeInteger(length) || length < 1 || length > 16 * 1024 * 1024) throw new Error("batch_length_limit");
      if (this.buffer.length < newline + length) break;
      /** @type {unknown} */ let batch;
      try {
        const json = new TextDecoder("utf-8", { fatal: true }).decode(this.buffer.subarray(newline, newline + length));
        batch = JSON.parse(json);
      } catch { throw new Error("invalid_batch_json"); }
      if (!Array.isArray(batch) || !batch.every(frame => Array.isArray(frame) && typeof frame[0] === "string")) throw new Error("invalid_batch_shape");
      frames.push(...batch);
      this.batchCount++;
      this.buffer = this.buffer.subarray(newline + length);
    }
    return frames;
  }
  /** Rejects truncated prefixes, length fields, and payloads.
   * @returns {void} */
  end() {
    if (!this.started || this.buffer.length) throw new Error("truncated_rpc_stream");
  }
}