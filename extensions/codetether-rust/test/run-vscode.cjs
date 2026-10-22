'use strict';
const fs = require('node:fs');
const path = require('node:path');
const { runTests } = require('@vscode/test-electron');

/** Run an isolated actual editor; example: xvfb-run -a node test/run-vscode.cjs. */
async function main() {
  const extension = path.resolve(__dirname, '..');
  const artifacts = path.resolve(extension, '../../artifacts');
  const evidence = fs.mkdtempSync(path.join(artifacts, 'rust-problems-vscode-'));
  const workspace = path.join(evidence, 'workspace');
  fs.mkdirSync(path.join(workspace, 'src'), { recursive: true });
  fs.writeFileSync(path.join(workspace, 'Cargo.toml'),
    '[workspace]\n[package]\nname="codetether-problems-fixture"\nversion="0.1.0"\nedition="2024"\n');
  fs.writeFileSync(path.join(workspace, 'src/lib.rs'), 'pub mod missing;\n');
  console.log(`VS Code evidence: ${evidence}`);
  try {
    const exitCode = await runTests({
      version: '1.143.0',
      extensionDevelopmentPath: extension,
      extensionTestsPath: path.join(__dirname, 'features-suite.cjs'),
      extensionTestsEnv: { CODETETHER_EDITOR_EVIDENCE: evidence, CARGO_TARGET_DIR: path.join(evidence, 'target') },
      launchArgs: [workspace, '--no-sandbox', '--disable-gpu', '--disable-workspace-trust',
        '--skip-welcome', '--skip-release-notes', '--telemetry-level=off',
        '--user-data-dir', path.join(evidence, 'user-data'),
        '--extensions-dir', path.join(evidence, 'extensions')]
    });
    fs.writeFileSync(path.join(evidence, 'exit.json'), JSON.stringify({ exitCode }));
    process.exitCode = exitCode;
  } catch (error) {
    fs.writeFileSync(path.join(evidence, 'exit.json'), JSON.stringify({ exitCode: 1, error: String(error) }));
    throw error;
  }
}
main().catch(error => { console.error(error); process.exitCode = 1; });