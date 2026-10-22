'use strict';
const vscode = require('vscode');
const fs = require('node:fs/promises');
const path = require('node:path');
const { inside } = require('./shared-envelope');

/** Capture only selected source, manifests and applicable instructions. Example: capture(uri, diagnostics). */
async function capture(uri, diagnostics) {
  const folder = vscode.workspace.getWorkspaceFolder(uri);
  if (!folder || !vscode.workspace.isTrusted) throw Error('Select a trusted workspace first.');
  const root = await fs.realpath(folder.uri.fsPath), source = await fs.realpath(uri.fsPath);
  if (!inside(root, source)) throw Error('Source resolves outside the selected workspace.');
  const snapshots = new Map();
  async function add(file, editable, label) {
    const canonical = await fs.realpath(file);
    if (editable && !inside(root, canonical)) throw Error('Editable context escapes the workspace.');
    const key = label || path.relative(root, canonical).split(path.sep).join('/');
    if (snapshots.has(key)) return;
    const text = (await fs.readFile(canonical, 'utf8')).replace(/^\uFEFF/, '');
    if (Buffer.byteLength(text) > 128 * 1024) throw Error(`Context file is too large: ${key}`);
    const document = editable ? await vscode.workspace.openTextDocument(vscode.Uri.file(canonical)) : undefined;
    if (document && (document.isDirty || document.getText() !== text)) throw Error(`Save or reload ${key} before requesting a fix.`);
    snapshots.set(key, { path: key, text, editable, file: canonical, version: document?.version });
  }
  await add(source, true);
  let owner;
  for (let dir = path.dirname(source); inside(root, dir); dir = path.dirname(dir)) {
    try { await fs.access(path.join(dir, 'Cargo.toml')); owner = path.join(dir, 'Cargo.toml'); break; } catch {}
    if (dir === root) break;
  }
  if (!owner) throw Error('No owning Cargo.toml found inside the workspace.');
  await add(owner, true);
  await add(path.join(root, 'Cargo.toml'), true);
  const ancestors = [];
  for (let dir = path.dirname(source); ; dir = path.dirname(dir)) {
    ancestors.unshift(dir); if (dir === path.dirname(dir)) break;
  }
  let index = 0;
  for (const dir of ancestors) for (const name of ['AGENTS.md', 'AGENTS.override.md']) {
    const file = path.join(dir, name);
    try { await fs.access(file); } catch { continue; }
    await add(file, false, `instructions/${index++}-${name}`);
  }
  const relative = path.relative(root, source).split(path.sep).join('/');
  const request = { schema_version: 1, diagnostics: diagnostics.slice(0, 50).map(item => ({
    path: relative, code: String(typeof item.code === 'object' ? item.code.value : item.code || ''),
    message: item.message, range: item.range
  })), files: [...snapshots.values()].map(({ path, text, editable }) => ({ path, text, editable })) };
  if (Buffer.byteLength(JSON.stringify(request)) > 480 * 1024 || snapshots.size > 16) throw Error('Repair context exceeds its safe limit; select a smaller file.');
  let inPackage = false, packageName;
  for (const line of (await fs.readFile(owner, 'utf8')).split('\n')) {
    if (/^\s*\[/.test(line)) inPackage = line.trim() === '[package]';
    const match = inPackage && line.match(/^\s*name\s*=\s*"([A-Za-z0-9_.-]+)"/);
    if (match) packageName = match[1];
  }
  return { folder, root, source, snapshots, request, packageName };
}

/** Refuse stale proposals or dirty concurrent edits. Example: await fresh(snapshot). */
async function fresh(context) {
  for (const item of context.snapshots.values()) {
    if (await fs.realpath(item.file) !== item.file) throw Error(`Path changed: ${item.path}`);
    const disk = (await fs.readFile(item.file, 'utf8')).replace(/^\uFEFF/, '');
    if (disk !== item.text) throw Error(`File changed while planning: ${item.path}. Request a new fix.`);
    if (item.editable) {
      const document = await vscode.workspace.openTextDocument(vscode.Uri.file(item.file));
      if (document.isDirty || document.version !== item.version || document.getText() !== item.text) throw Error(`Editor changed: ${item.path}. Request a new fix.`);
    }
  }
}
module.exports = { capture, fresh };