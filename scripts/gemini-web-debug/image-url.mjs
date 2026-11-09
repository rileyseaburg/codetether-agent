import { ImageError } from "./image-error.mjs";

/** Validate every hop, including redirects, before sending credentials.
 * @param {string} value @returns {URL} */
export function imageUrl(value) {
  /** @type {URL} */ let url;
  try { url = new URL(value); } catch { throw new ImageError("image_url_invalid"); }
  const allowed = url.hostname === "lh3.googleusercontent.com"
    ? /^\/(?:gg-dl|rd-gg-dl)\//.test(url.pathname)
    : url.hostname === "work.fife.usercontent.google.com" && url.pathname.startsWith("/rd-gg-dl/");
  if (url.protocol !== "https:" || url.port || url.username || url.password || url.hash || !allowed) {
    throw new ImageError("image_url_disallowed");
  }
  return url;
}

/** Select the captured 1024px JPEG rendition. Returned URLs are sensitive.
 * @param {string} value @returns {string} */
export function imageRendition(value) {
  const url = imageUrl(value);
  if (url.hostname !== "lh3.googleusercontent.com" || !url.pathname.startsWith("/gg-dl/")) {
    throw new ImageError("image_reference_disallowed");
  }
  url.pathname = `${url.pathname.replace(/=s\d+(?:-[a-z]+)?$/, "")}=s1024-rj`;
  url.searchParams.set("alr", "yes"); return url.href;
}