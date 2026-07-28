#!/usr/bin/env node
const http = require('http');

const url = new URL(process.argv[2] || 'http://127.0.0.1:8788/');
const requests = Number(process.argv[3] || 500);
const concurrency = Number(process.argv[4] || 10);
const agent = new http.Agent({ keepAlive: true, maxSockets: concurrency });

let started = process.hrtime.bigint();
let completed = 0;
let failed = 0;
let inFlight = 0;
let next = 0;
let bytes = 0;
let min = Number.POSITIVE_INFINITY;
let max = 0;
let sum = 0;

function one(done) {
  const t0 = process.hrtime.bigint();
  const req = http.get({
    hostname: url.hostname,
    port: url.port,
    path: url.pathname,
    agent,
  }, res => {
    res.on('data', chunk => { bytes += chunk.length; });
    res.on('end', () => {
      const ms = Number(process.hrtime.bigint() - t0) / 1e6;
      min = Math.min(min, ms);
      max = Math.max(max, ms);
      sum += ms;
      if (res.statusCode !== 200) failed++;
      done();
    });
  });
  req.on('error', () => { failed++; done(); });
}

function pump() {
  while (inFlight < concurrency && next < requests) {
    next++;
    inFlight++;
    one(() => {
      completed++;
      inFlight--;
      if (completed === requests) finish();
      else pump();
    });
  }
}

function finish() {
  const elapsed = Number(process.hrtime.bigint() - started) / 1e9;
  const rps = requests / elapsed;
  console.log(JSON.stringify({
    requests,
    concurrency,
    failed,
    seconds: Number(elapsed.toFixed(3)),
    rps: Number(rps.toFixed(1)),
    avg_ms: Number((sum / requests).toFixed(3)),
    min_ms: Number(min.toFixed(3)),
    max_ms: Number(max.toFixed(3)),
    bytes,
  }, null, 2));
  agent.destroy();
}

pump();
