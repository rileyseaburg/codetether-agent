// Cache only the public app shell. Credentials, API responses and screenshots never enter CacheStorage.
const CACHE = 'codetether-screen-shell-v1';
const SHELL = ['','index.html','style.css','app.js','render.js','manifest.webmanifest','icon.svg',
  'api.js','session.js','models.js','flow.js','pairing.js','sharing.js','capture.js','pump.js','halt.js']
  .map(file => `/companion/${file}`);
self.addEventListener('install', event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(SHELL)));
});
self.addEventListener('activate', event => { event.waitUntil(self.clients.claim()); });
self.addEventListener('fetch', event => {
  const url = new URL(event.request.url);
  if (event.request.method !== 'GET' || event.request.headers.has('Authorization')
    || url.origin !== self.location.origin || url.search || !SHELL.includes(url.pathname)) return;
  event.respondWith(fetch(event.request).catch(async () => {
    const cached = await caches.match(event.request, {cacheName:CACHE});
    return cached || new Response('Open the companion online to pair.', {status:503});
  }));
});