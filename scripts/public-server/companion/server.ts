import { companionServer } from './server-factory.ts';
import { visionAnalyzer } from './upstream.ts';

const token = process.env.CODETETHER_AUTH_TOKEN ?? '';
const server = companionServer(token, visionAnalyzer(token));
server.listen(4099, '127.0.0.1', () => {
  console.info(JSON.stringify({ event: 'screen_relay_listening', address: '127.0.0.1:4099' }));
});
for (const signal of ['SIGTERM', 'SIGINT'] as const) {
  process.on(signal, () => {
    server.close(); server.closeAllConnections();
  });
}