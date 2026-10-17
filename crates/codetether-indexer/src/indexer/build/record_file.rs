//! record file for index construction.

use super::*;

pub(super) fn record_file(
    path: &Path,
    rel_path: &Path,
    metadata: &std::fs::Metadata,
    content: &str,
    options: &BuildOptions,
    data: &mut IndexInputs,
) {
    let file = make_record(path, rel_path, metadata, content);
    data.files.push(file);
    let file = &data.files[data.files.len() - 1];
    data.knowledge_inputs.push(analyze_file_knowledge(
        &file.path,
        &file.language,
        file.lines,
        file.bytes,
        file.modified_unix_ms,
        &content,
    ));
    data.embedding_inputs.push(build_embedding_input(
        &file.path,
        &file.language,
        &content,
        options.embedding_input_chars,
    ));

    data.stats.indexed_files += 1;
    data.stats.total_bytes += file.bytes;
    data.stats.total_lines += u64::from(file.lines);
    *data
        .stats
        .language_counts
        .entry(file.language.clone())
        .or_insert(0) += 1;
}
