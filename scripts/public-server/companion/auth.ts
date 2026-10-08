import { createHash, timingSafeEqual } from 'node:crypto';
import type { IncomingMessage } from 'node:http';
import { HTTPError } from './types.ts';

/** Hash device capabilities so the registry does not retain their raw values. */
export function digest(value: string): string {
  return createHash('sha256').update(value).digest('hex');
}
export function bearer(request: IncomingMessage): string {
  const value = request.headers.authorization;
  if (!value || !/^Bearer [A-Za-z0-9._~-]+$/.test(value)) throw new HTTPError(401, 'Authentication required');
  return value.slice(7);
}
export function requireToken(request: IncomingMessage, expectedHash: string): void {
  const actual = Buffer.from(digest(bearer(request)), 'hex');
  const expected = Buffer.from(expectedHash, 'hex');
  if (expected.length !== actual.length || !timingSafeEqual(expected, actual)) {
    throw new HTTPError(401, 'Authentication rejected');
  }
}
/** Native clients omit Origin; browser mutations must come from our fixed origin. */
export function requireOrigin(request: IncomingMessage, origin: string): void {
  if (request.headers.origin && request.headers.origin !== origin) {
    throw new HTTPError(403, 'Cross-origin requests are not allowed');
  }
}