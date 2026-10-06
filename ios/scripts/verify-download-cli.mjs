// Record live HTTPS evidence without sending installation emails.
import { verifyDownload } from './verify-download.mjs';

const [directory, version, build, sha256] = process.argv.slice(2);
if (process.argv.length !== 6) {
  throw new Error('Usage: verify-download-cli.mjs EVIDENCE VERSION BUILD SHA256');
}

console.log(JSON.stringify(
  await verifyDownload(directory, version, build, sha256),
  null, 2,
));