'use strict';
const path = require('node:path');
const { fileURLToPath, pathToFileURL } = require('node:url');

/** Enforce canonical workspace containment. Example: inside('/repo', '/repo/a.rs'). */
function inside(root, file) {
  const relative = path.relative(root, file);
  return relative !== '..' && !relative.startsWith(`..${path.sep}`) && !path.isAbsolute(relative);
}
/** Read a native producer's liveness without contacting a server. Example: alive(process.pid). */
function alive(pid) {
  try { process.kill(pid, 0); return true; } catch (error) { return error.code === 'EPERM'; }
}
/** Validate native snapshots before publishing UI diagnostics. Example: await decode(snapshot, root, realpath). */
async function decode(value, root, realpath, now = Date.now(), isAlive = alive) {
  if (value.schema_version !== 1 || !Number.isSafeInteger(value.pid) || value.pid <= 0 || !isAlive(value.pid)) return [];
  if (!Number.isFinite(value.updated_at_ms) || now - value.updated_at_ms > 15000 || value.updated_at_ms > now + 5000) return [];
  if (await realpath(fileURLToPath(value.workspace_uri)) !== root) return [];
  if (!value.diagnostics || typeof value.diagnostics !== 'object' || Array.isArray(value.diagnostics)) return [];
  const result = [];
  for (const [uri, items] of Object.entries(value.diagnostics).slice(0, 2000)) {
    if (!Array.isArray(items)) continue;
    let file;
    try { file = await realpath(fileURLToPath(uri)); } catch { continue; }
    if (!inside(root, file)) continue;
    const diagnostics = items.slice(0, 500).filter(item => {
      const start = item.range?.start, end = item.range?.end;
      return typeof item.message === 'string' && item.message.length <= 65536 &&
        [start?.line, start?.character, end?.line, end?.character].every(n => Number.isInteger(n) && n >= 0) &&
        (end.line > start.line || (end.line === start.line && end.character >= start.character));
    }).map(item => ({ message: item.message, range: item.range,
      severity: [1, 2, 3, 4].includes(item.severity) ? item.severity : 1,
      code: typeof item.code === 'string' || typeof item.code === 'number' ? item.code : undefined }));
    result.push([pathToFileURL(file).href, diagnostics]);
  }
  return result;
}
module.exports = { decode, inside };