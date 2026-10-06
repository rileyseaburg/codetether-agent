export class HTTPError extends Error {
  constructor(status, message) { super(message); this.status = status; }
}

// Delegate authentication to the Rust server; do not duplicate or store its key.
export async function authorize(header, origin, signal) {
  if (typeof header !== 'string' || !/^Bearer \S+$/.test(header)) {
    throw new HTTPError(401, 'Bearer authentication required');
  }
  const response = await fetch(`${origin}/api/version`, {
    headers: { Authorization: header }, redirect: 'error', signal
  });
  await response.body?.cancel();
  if (!response.ok) throw new HTTPError([401, 403].includes(response.status) ? response.status : 503, 'Authorization unavailable or denied');
}
