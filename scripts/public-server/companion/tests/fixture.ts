import { once } from 'node:events';
import type { AddressInfo } from 'node:net';
import type { TestContext } from 'node:test';
import { companionServer } from '../server-factory.ts';
import type { Analyze } from '../types.ts';

export const token = 'mock-owner-credential-not-a-real-token-0000';
export const input = { model: 'mock/vision', prompt: 'What is on screen?', interval_seconds: 15 };
/** Header-only JPEG fixture: validates framing, never used for a live vision call. */
export function image(): string {
  const bytes = Buffer.alloc(32);
  bytes.set([255, 216, 255, 192, 0, 17, 8, 0, 16, 0, 16]);
  bytes.set([255, 217], 30); return bytes.toString('base64');
}
export async function fixture(t: TestContext, analyze: Analyze): Promise<string> {
  const server = companionServer(token, analyze);
  server.listen(0, '127.0.0.1'); await once(server, 'listening');
  t.after(() => { server.closeAllConnections(); server.close(); });
  return `http://127.0.0.1:${(server.address() as AddressInfo).port}/companion`;
}
export async function request(base: string, path: string, body: unknown, credential = token, method = 'POST'): Promise<Response> {
  return fetch(`${base}${path}`, { method, headers: {
    'Content-Type': 'application/json', ...(credential ? { Authorization: `Bearer ${credential}` } : {})
  }, body: body === undefined ? undefined : JSON.stringify(body) });
}
export async function readUntil(reader: ReadableStreamDefaultReader<Uint8Array>, expected: string): Promise<string> {
  let text = '';
  while (!text.includes(expected)) {
    const chunk = await reader.read(); if (chunk.done) break;
    text += new TextDecoder().decode(chunk.value);
  }
  return text;
}