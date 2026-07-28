#!/usr/bin/env node
const http = require('http');
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, 'content_site', 'dist');
const port = Number(process.env.PORT || 8789);
const quiet = process.env.QUIET === '1';

function typeFor(file) {
  if (file.endsWith('.html')) return 'text/html; charset=utf-8';
  if (file.endsWith('.json')) return 'application/json';
  if (file.endsWith('.xml')) return 'application/xml';
  return 'text/plain; charset=utf-8';
}

const server = http.createServer((req, res) => {
  let urlPath = decodeURIComponent((req.url || '/').split('?')[0]);
  if (urlPath.includes('..')) {
    res.writeHead(403, { 'content-type': 'text/plain; charset=utf-8' });
    res.end('forbidden\n');
    return;
  }
  if (urlPath === '/') urlPath = '/index.html';
  if (urlPath.endsWith('/')) urlPath += 'index.html';
  const file = path.join(root, urlPath);
  fs.readFile(file, (err, data) => {
    if (err) {
      res.writeHead(404, { 'content-type': 'text/plain; charset=utf-8' });
      res.end(`not found: ${urlPath}\n`);
      return;
    }
    res.writeHead(200, { 'content-type': typeFor(file) });
    res.end(data);
  });
});

server.listen(port, '127.0.0.1', () => {
  if (!quiet) console.log(`Node.js static site server on http://127.0.0.1:${port}/`);
});
