import http from 'node:http';
import { join } from 'node:path';
import { assetHandler } from './assets.mjs';
const root = join(process.env.HOME, '.local/share/codetether-ios/releases');
const latest = process.env.CODETETHER_IOS_RELEASE ?? '1.4.0-8';
const server = http.createServer(assetHandler(root, latest));
server.requestTimeout = 30_000;
server.headersTimeout = 10_000;
server.listen(4098, '127.0.0.1');
for (const signal of ['SIGTERM', 'SIGINT']) process.on(signal, () => {
  server.close(() => process.exit(0));
  setTimeout(() => process.exit(1), 5000).unref();
});