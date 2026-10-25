/** Credential-free image workflow failure. Details contain only safe metadata. */
export class ImageError extends Error {
  /** @param {string} code @param {Record<string, unknown>} [details] */
  constructor(code, details = {}) {
    super(code);
    this.name = "ImageError";
    this.details = details;
  }
}