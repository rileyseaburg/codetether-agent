const test = require('node:test');
const assert = require('node:assert/strict');

const {
  configuredPkgTag,
  defaultCacheDirFor,
  selectAssetCandidates,
} = require('../lib/installer');
const { downloadFile, downloadText, requestJson } = require('../lib/http');

function mockHttpsGet(responses, calls) {
  return (url, options, onResponse) => {
    const spec = responses.get(url);
    const req = {
      on(event, handler) {
        if (event === 'error') {
          this.onError = handler;
        }
        return this;
      },
    };

    process.nextTick(() => {
      if (!spec) {
        req.onError(new Error(`Unexpected URL: ${url}`));
        return;
      }

      const listeners = {};
      const res = {
        statusCode: spec.statusCode,
        headers: spec.headers || {},
        on(event, handler) {
          listeners[event] = handler;
          return this;
        },
        resume() {},
        pipe(dest) {
          process.nextTick(() => {
            if (typeof dest.emit === 'function') {
              dest.emit('finish');
            }
          });
          return dest;
        },
      };

      calls.push({ url, options });
      onResponse(res);
      if (spec.error && listeners.error) {
        listeners.error(spec.error);
        return;
      }
      if (spec.body && listeners.data) {
        listeners.data(Buffer.from(spec.body));
      }
      if (listeners.end) {
        listeners.end();
      }
    });

    return req;
  };
}

test('prefers explicit release tags over npm package versions', () => {
  assert.equal(
    configuredPkgTag({
      version: '1.1.6-alpha-7.2',
      codetetherReleaseTag: 'v4.6.0',
    }),
    'v4.6.0'
  );
});

test('uses the standard macOS cache directory', () => {
  assert.equal(
    defaultCacheDirFor({
      platform: 'darwin',
      env: {},
      homedir: '/Users/tester',
    }),
    '/Users/tester/Library/Caches/codetether-npx'
  );
});

test('prefers published Windows GNU fallback assets when MSVC assets are missing', () => {
  const candidates = selectAssetCandidates({
    tag: 'v4.4.1',
    targetTriple: 'x86_64-pc-windows-msvc',
    isWindowsTarget: true,
    availableAssetNames: [
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
      'SHA256SUMS-v4.4.1.txt',
    ],
  });

  assert.deepEqual(
    candidates.map((candidate) => candidate.name),
    [
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
    ]
  );
});

test('keeps preferred Windows asset order when release assets cannot be enumerated', () => {
  const candidates = selectAssetCandidates({
    tag: 'v4.4.1',
    targetTriple: 'x86_64-pc-windows-msvc',
    isWindowsTarget: true,
    availableAssetNames: [],
  });

  assert.deepEqual(
    candidates.map((candidate) => candidate.name),
    [
      'codetether-v4.4.1-x86_64-pc-windows-msvc.zip',
      'codetether-v4.4.1-x86_64-pc-windows-msvc.exe',
      'codetether-v4.4.1-x86_64-pc-windows-msvc.tar.gz',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.zip',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.exe',
      'codetether-v4.4.1-x86_64-pc-windows-gnu.tar.gz',
    ]
  );
});

test('preserves TLS options across redirected installer downloads', async () => {
  const agent = { name: 'darwin-agent' };
  const calls = [];
  const responses = new Map([
    [
      'https://github.com/rileyseaburg/codetether-agent/releases/download/v4.6.1/test.txt',
      { statusCode: 302, headers: { location: 'https://objects.example.com/test.txt' } },
    ],
    ['https://objects.example.com/test.txt', { statusCode: 200, body: 'ok' }],
  ]);

  const body = await downloadText(
    'https://github.com/rileyseaburg/codetether-agent/releases/download/v4.6.1/test.txt',
    {
      httpsGet: mockHttpsGet(responses, calls),
      tlsOptions: () => ({ agent }),
    }
  );

  assert.equal(body, 'ok');
  assert.equal(calls.length, 2);
  assert.equal(calls[0].options.agent, agent);
  assert.equal(calls[1].options.agent, agent);
});

test('propagates response stream errors from redirected downloads', async () => {
  const boom = new Error('socket reset');
  const responses = new Map([['https://objects.example.com/test.txt', { statusCode: 200, error: boom }]]);

  await assert.rejects(
    downloadText('https://objects.example.com/test.txt', {
      httpsGet: mockHttpsGet(responses, []),
    }),
    /socket reset/
  );
});

test('reports requestJson redirect exhaustion as fetching', async () => {
  const url = 'https://api.github.com/repos/rileyseaburg/codetether-agent/releases/latest';
  const responses = new Map([[url, { statusCode: 302, headers: { location: url } }]]);

  await assert.rejects(
    requestJson(url, {}, { httpsGet: mockHttpsGet(responses, []) }),
    /Too many redirects fetching/
  );
});

test('rejects downloadFile when closing the output stream fails', async () => {
  const closeError = new Error('close failed');
  const responses = new Map([['https://objects.example.com/test.bin', { statusCode: 200 }]]);
  const createWriteStream = () => {
    const listeners = {};
    return {
      on(event, handler) {
        listeners[event] = handler;
        return this;
      },
      emit(event, value) {
        if (listeners[event]) {
          listeners[event](value);
        }
      },
      close(callback) {
        callback(closeError);
      },
    };
  };

  await assert.rejects(
    downloadFile('https://objects.example.com/test.bin', '/tmp/test.bin', {
      httpsGet: mockHttpsGet(responses, []),
      createWriteStream,
    }),
    /close failed/
  );
});
