import { readFileSync } from 'node:fs';
import { IncomingMessage } from 'node:http';
import { Socket } from 'node:net';

interface SecurityFixtures {
  owner: string;
  authorization: { header: string | null; status: number }[];
  allowed_origin: string;
  origins: { origin: string | null; status: number }[];
  normalized_codes: { input: string; output: string }[];
}
/** Synthetic security cases shared with the Rust session core. */
export const security = JSON.parse(readFileSync(new URL(
  '../../../../crates/codetether-companion-core/fixtures/security.json', import.meta.url
), 'utf8')) as SecurityFixtures;

export function request(header: 'authorization' | 'origin', value: string | null): IncomingMessage {
  const message = new IncomingMessage(new Socket());
  if (value !== null) message.headers[header] = value;
  return message;
}