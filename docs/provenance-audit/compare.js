// Inputs `source` and `document` are supplied by run.tether.
// Compare transcription content, not protocol correctness or rendering.
const originalBody = source.replace(/\f/g, "").split("— abstract")[1]
  .replace(/— (middle|back)/g, "").replace("{::boilerplate bcp14-tagged}", "")
  .replace(/ \+--[\s\S]*?\{: title=[^}]+\}/, "");
const markdownBody = document.split("## Abstract")[1].split("## References")[0]
  .replace(/### 1\.1\. Requirements Language[\s\S]*?(?=## 2\.)/, "Requirements Language\n")
  .replace(/^#{2,3} (?:[0-9.]+ |Appendix [ABC]\. )?/gm, "")
  .replace(/```text\n\+--[\s\S]*?\*CodeTether Profile Component Model\*/, "")
  .replace(/```(?:text|json)?/g, "");
const originalText = originalBody.replace(/[^A-Za-z0-9]/g, "");
const markdownText = markdownBody.replace(/[^A-Za-z0-9]/g, "");
let offset = 0;
while (offset < originalText.length && originalText[offset] === markdownText[offset]) offset++;
const originalToken = JSON.parse(source.slice(source.indexOf(" {\n"), source.indexOf("Acknowledgments")).replace(/\f/g, "").trim());
const markdownToken = JSON.parse(document.match(/```json\n([\s\S]*?)```/)[1]);
const definitions = (document.match(/^\[[^\]]+\]:/gm) || []).map(line => line.slice(1, -2));
const citations = (document.match(/\[(?:RFC[^\]]+|I-D\.[^\]]+|SPIFFE|A2A|MCP)\]/g) || []).map(value => value.slice(1, -1));
const report = {
  level: "static/local",
  normalized_body_equal: originalText === markdownText,
  source_characters: originalText.length,
  document_characters: markdownText.length,
  first_difference: originalText === markdownText ? null : { offset, source: originalText.slice(offset - 50, offset + 160), document: markdownText.slice(offset - 50, offset + 160) },
  example_json_equal: JSON.stringify(originalToken) === JSON.stringify(markdownToken),
  reference_definitions: definitions.length,
  missing_references: citations.filter(id => !definitions.includes(id)),
  main_sections: (document.match(/^## [0-9]+\./gm) || []).length,
  appendices: (document.match(/^## Appendix [ABC]\./gm) || []).length,
  code_fences: (document.match(/^```/gm) || []).length
};
report.ok = report.normalized_body_equal && report.example_json_equal && report.missing_references.length === 0 && report.main_sections === 11 && report.appendices === 3 && report.code_fences === 6;
JSON.stringify(report, null, 2);