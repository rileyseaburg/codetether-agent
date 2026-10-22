'use strict';
const vscode = require('vscode');
const { randomUUID } = require('node:crypto');
const { fresh } = require('./fix-context');

/** Validate proposed replacements against captured editable files. Example: replacements(context, proposal). */
function replacements(context, proposal) {
  if (!proposal || proposal.schema_version !== 1 || typeof proposal.summary !== 'string' || !Array.isArray(proposal.edits) || proposal.edits.length > 8) throw Error('Invalid repair proposal.');
  if (Object.keys(proposal).some(key => !['schema_version', 'summary', 'edits'].includes(key))) throw Error('Unexpected proposal fields.');
  const seen = new Set();
  return proposal.edits.map(edit => {
    if (!edit || Object.keys(edit).some(key => !['path', 'old_text', 'new_text'].includes(key))) throw Error('Invalid edit fields.');
    const snapshot = context.snapshots.get(edit.path);
    if (!snapshot?.editable || seen.has(edit.path)) throw Error(`Edit is outside captured scope or duplicated: ${edit.path}`);
    seen.add(edit.path);
    if (typeof edit.old_text !== 'string' || !edit.old_text || typeof edit.new_text !== 'string' || edit.old_text === edit.new_text) throw Error('Invalid replacement text.');
    const offset = snapshot.text.indexOf(edit.old_text);
    if (offset < 0 || snapshot.text.lastIndexOf(edit.old_text) !== offset) throw Error(`Replacement is not unique in ${edit.path}.`);
    if (Buffer.byteLength(edit.new_text) > 128 * 1024) throw Error('Replacement is too large.');
    return { snapshot, offset, old: edit.old_text, text: edit.new_text,
      preview: snapshot.text.slice(0, offset) + edit.new_text + snapshot.text.slice(offset + edit.old_text.length) };
  });
}

/** Own virtual diff documents and user-approved edits. Example: review(context, proposal, runner). */
function create(context) {
  const views = new Map();
  context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider('codetether-repair', {
    provideTextDocumentContent: uri => views.get(uri.toString()) || ''
  }));
  return async function review(snapshot, proposal, runner) {
    const changes = replacements(snapshot, proposal);
    if (!changes.length) { vscode.window.showInformationMessage(proposal.summary || 'No safe repair proposed.'); return; }
    await fresh(snapshot);
    const uris = [];
    try {
      for (const [index, change] of changes.entries()) {
        const uri = vscode.Uri.parse(`codetether-repair:/${randomUUID()}/${encodeURIComponent(change.snapshot.path)}`);
        uris.push(uri); views.set(uri.toString(), change.preview);
        await vscode.commands.executeCommand('vscode.diff', vscode.Uri.file(change.snapshot.file), uri, `CodeTether repair: ${change.snapshot.path}`);
        const last = index === changes.length - 1;
        const button = last ? 'Apply & Check' : 'Next file';
        const choice = await vscode.window.showWarningMessage(
          `${proposal.summary}\nReview ${index + 1}/${changes.length}: ${change.snapshot.path}.${last ? ' Applying also runs Cargo check, including build scripts.' : ''}`,
          { modal: false }, button, 'Cancel');
        if (choice !== button) return;
      }
      await fresh(snapshot);
      const edit = new vscode.WorkspaceEdit(), documents = [];
      for (const change of changes) {
        const document = await vscode.workspace.openTextDocument(vscode.Uri.file(change.snapshot.file));
        documents.push(document);
        edit.replace(document.uri, new vscode.Range(document.positionAt(change.offset), document.positionAt(change.offset + change.old.length)), change.text);
      }
      for (let index = 0; index < documents.length; index++) {
        const document = documents[index], original = changes[index].snapshot;
        if (document.isDirty || document.version !== original.version || document.getText() !== original.text) throw Error(`File changed before apply: ${original.path}`);
      }
      if (!await vscode.workspace.applyEdit(edit)) throw Error('VS Code refused the edit; no verification was started.');
      await runner.suppressSaves(async () => {
        for (const document of documents) if (!await document.save()) throw Error('Could not save the approved edit; review the buffer before retrying.');
      });
      await runner.run('check', snapshot.folder, false, { package: snapshot.packageName || '', bin: '' });
    } finally { for (const uri of uris) views.delete(uri.toString()); }
  };
}
module.exports = { create, replacements };