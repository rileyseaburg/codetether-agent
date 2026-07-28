#!/usr/bin/env node
const http = require('http');
const fs = require('fs');
const path = require('path');
const port = Number(process.env.PORT || 8791);
const body = fs.readFileSync(path.join(__dirname, 'content_site', 'dist', 'index.html'));
http.createServer((_req, res) => {
  res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' });
  res.end(body);
}).listen(port, '127.0.0.1', () => {
  if (process.env.QUIET !== '1') console.log(`Node cached server on http://127.0.0.1:${port}/`);
});
