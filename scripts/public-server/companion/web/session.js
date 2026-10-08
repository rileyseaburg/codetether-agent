// @ts-check
/** Check an untrusted pair response without retaining the response object.
 * @param {unknown} value @returns {import('./models.js').Session} */
export function parseSession(value) {
  if (!value || typeof value !== 'object' || !('id' in value)
    || typeof value.id !== 'string' || !/^[A-Za-z0-9_-]{1,128}$/.test(value.id)
    || !('device_token' in value) || typeof value.device_token !== 'string'
    || !/^[\x21-\x7e]{1,4096}$/.test(value.device_token)
    || !('expires_at' in value) || typeof value.expires_at !== 'string') {
    throw new Error('Invalid pairing response');
  }
  const expires = Date.parse(value.expires_at);
  const seconds = 'interval_seconds' in value ? value.interval_seconds : 30;
  if (!Number.isFinite(expires) || expires <= Date.now()
    || typeof seconds !== 'number' || !Number.isFinite(seconds)
    || seconds <= 0 || seconds > 86400) throw new Error('Invalid session limits');
  return {id:value.id, token:value.device_token,
    minimum:Math.max(15, Math.ceil(seconds)), expires};
}
/** @param {number} status */
export const isRevoked = status => [401,404,410].includes(status);