// Minimal framed LSP fixture; serialize delayed handlers like a real process.
/** @typedef {{id?: number, method?: string, result?: object|null, params?: {textDocument: {uri: string}}}} Request */
let mode = process.argv[2];
let input = Buffer.alloc(0);
let pending = Promise.resolve();
/** @param {number} ms @returns {Promise<void>} */
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
/** @param {object} message @returns {void} */
function send(message) {
    const body = Buffer.from(JSON.stringify({jsonrpc: '2.0', ...message}));
    process.stdout.write(`Content-Length: ${body.length}\r\n\r\n`);
    process.stdout.write(body);
}
/** @param {Request} message @returns {Promise<void>} */
async function respond(message) {
    if (message.method === 'initialize') {
        if (mode === 'slow_init') { await delay(400); mode = 'healthy'; }
        if (mode === 'startup_hang') await delay(60000);
        send({id: message.id, result: {capabilities: {}}});
    } else if (message.method === 'shutdown') {
        send({id: message.id, result: null});
    } else if (message.method === 'exit') {
        process.exit(0);
    } else if (['textDocument/didOpen', 'textDocument/didChange'].includes(message.method)
        && ['healthy', 'slow'].includes(mode)) {
        if (mode === 'slow') { await delay(5400); mode = 'healthy'; }
        send({method: 'textDocument/publishDiagnostics', params: {
            uri: message.params.textDocument.uri, diagnostics: []}});
    }
}
process.stdin.on('data', chunk => {
    input = Buffer.concat([input, chunk]);
    for (;;) {
        const end = input.indexOf('\r\n\r\n');
        if (end < 0) return;
        const length = Number(/Content-Length:\s*(\d+)/i.exec(input.subarray(0, end).toString())[1]);
        if (input.length < end + 4 + length) return;
        const message = JSON.parse(input.subarray(end + 4, end + 4 + length));
        input = input.subarray(end + 4 + length);
        pending = pending.then(() => respond(message));
    }
});