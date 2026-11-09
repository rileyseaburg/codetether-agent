import { ImageError } from "./image-error.mjs";

/** Refresh only at Google's fixed HTTPS endpoint, with no redirect forwarding.
 * @param {string} cookie @returns {Promise<{status: number, headers: string[]}>} */
export async function rotateCookieRequest(cookie) {
  try {
    const response = await fetch("https://accounts.google.com/RotateCookies", {
      method: "POST", redirect: "manual", signal: AbortSignal.timeout(30000),
      headers: {Cookie: cookie, "Content-Type": "application/json", Origin: "https://accounts.google.com"},
      body: '[000,"-0000000000000000000"]',
    });
    const result = {status: response.status, headers: response.headers.getSetCookie()};
    await response.body?.cancel();
    return result;
  } catch { throw new ImageError("cookie_refresh_transport_error"); }
}