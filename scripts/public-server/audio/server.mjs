import { createServer } from 'node:http';
import { createHandler } from './handler.mjs';

const server = createServer(createHandler('http://127.0.0.1:4096', 'http://127.0.0.1:8016'));
server.requestTimeout = 15000;
server.headersTimeout = 10000;
server.listen(4097, '127.0.0.1', () => console.log('Authenticated Kokoro bridge listening on loopback port 4097'));
for (const signal of ['SIGTERM', 'SIGINT']) process.on(signal, () => server.close());
