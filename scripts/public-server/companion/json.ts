import type { IncomingMessage, ServerResponse } from 'node:http';
import { HTTPError } from './types.ts';

/** Read bounded JSON without writing request bodies to disk or logs. */
export async function readJSON(request: IncomingMessage, limit = 4096): Promise<unknown> {
  if (request.headers['content-type']?.split(';')[0].trim() !== 'application/json') {
    throw new HTTPError(415, 'Use application/json');
  }
  if (Number(request.headers['content-length']) > limit) throw new HTTPError(413, 'Request too large');
  const chunks: Buffer[] = [];
  let size = 0;
  for await (const chunk of request) {
    const bytes: Buffer = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    size += bytes.length;
    if (size > limit) throw new HTTPError(413, 'Request too large');
    chunks.push(bytes);
  }
  try { return JSON.parse(Buffer.concat(chunks).toString('utf8')) as unknown; }
  catch { throw new HTTPError(400, 'Invalid JSON'); }
}
export function json(response: ServerResponse, status: number, value: unknown): void {
  response.writeHead(status, { 'Content-Type': 'application/json', 'Cache-Control': 'no-store',
    'X-Content-Type-Options': 'nosniff', 'Referrer-Policy': 'no-referrer' });
  response.end(JSON.stringify(value));
}