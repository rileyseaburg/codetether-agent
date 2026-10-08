// @ts-check
import {parseSession} from './session.js';
/** Narrow HTTP errors; never display or log arbitrary response bodies. */
export class HttpError extends Error {
  /** @param {number} status */
  constructor(status) { super('Companion request rejected'); this.status = status; }
}
/** Same-origin, no redirects, no cookies, no caching; bearer stays in memory.
 * @param {string} path @param {object} body @param {AbortSignal} signal
 * @param {string|null} token @param {boolean} [keepalive] */
async function post(path, body, signal, token, keepalive = false) {
  const headers = new Headers({'Content-Type':'application/json'});
  if (token) headers.set('Authorization', `Bearer ${token}`);
  return fetch(`/companion/${path}`, {method:'POST', headers,
    body:JSON.stringify(body), cache:'no-store', credentials:'omit',
    redirect:'error', signal, keepalive});
}
/** @param {string} code @param {AbortSignal} signal */
export async function pair(code, signal) {
  const response = await post('pair', {code}, signal, null);
  if (response.status !== 200) throw new HttpError(response.status);
  return parseSession(/** @type {unknown} */ (await response.json()));
}
/** @param {import('./models.js').Session} session @param {'frames'|'pause'} action
 * @param {object} body @param {AbortSignal} signal @param {boolean} [keepalive] */
export const send = async (session, action, body, signal, keepalive = false) =>
  (await post(`sessions/${session.id}/${action}`, body, signal, session.token, keepalive)).status;