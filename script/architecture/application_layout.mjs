// Locate the application package before or after its workspace relocation.
import fs from 'node:fs';
import path from 'node:path';

export function applicationLayout(repository = process.cwd()) {
  const candidates = ['.', 'crates/codetether-agent'].filter(directory => {
    const root = path.join(repository, directory);
    if (!fs.existsSync(path.join(root, 'src/lib.rs'))) return false;
    const manifest = path.join(root, 'Cargo.toml');
    if (!fs.existsSync(manifest)) return false;
    const packageSection = fs.readFileSync(manifest, 'utf8')
      .match(/^\[package\]\s*\n([\s\S]*?)(?=^\[|$(?![\s\S]))/m)?.[1];
    return /^name\s*=\s*"codetether-agent"\s*$/m.test(packageSection ?? '');
  });
  if (candidates.length !== 1) {
    throw new Error(`Expected one codetether-agent application package; found ${candidates.length}`);
  }
  const directory = candidates[0];
  return {
    directory,
    manifest: path.posix.join(directory, 'Cargo.toml'),
    source: path.posix.join(directory, 'src'),
  };
}
