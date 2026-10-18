// Tokenize Rust comments, identifiers, punctuation and literals for static audits.
// This intentionally does not parse or type-check Rust.
export function tokens(source) {
  const out = [];
  for (let i = 0; i < source.length;) {
    const start = i, c = source[i], next = source[i + 1];
    if (/\s/.test(c)) { i++; continue; }
    if (c === '/' && next === '/') { i = source.indexOf('\n', i); if (i < 0) break; continue; }
    if (c === '/' && next === '*') {
      let depth = 1; i += 2;
      while (i < source.length && depth) {
        if (source.slice(i, i + 2) === '/*') { depth++; i += 2; }
        else if (source.slice(i, i + 2) === '*/') { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    const raw = source.slice(i).match(/^(?:br|cr|r)(#*)"/);
    if (raw) {
      const end = source.indexOf('"' + raw[1], i + raw[0].length);
      if (end < 0) throw new Error('Unclosed raw literal');
      i = end + 1 + raw[1].length;
    } else if (c === '"' || (c === "'" && (next === '\\' || source[i + 2] === "'"))) {
      const quote = c; i++;
      while (i < source.length) {
        if (source[i] === '\\') i += 2;
        else if (source[i++] === quote) break;
      }
    } else if (/[a-zA-Z_]/.test(c)) {
      i++; while (i < source.length && /[a-zA-Z_0-9]/.test(source[i])) i++;
    } else i++;
    out.push({ value: source.slice(start, i), start, end: i });
  }
  return out;
}
