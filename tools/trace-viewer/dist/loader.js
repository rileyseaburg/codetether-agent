function object(value) {
    return typeof value === "object" && value !== null;
}
function record(value) {
    return object(value) && typeof value.line === "number" && typeof value.raw === "string"
        && typeof value.raw_base64 === "string" && Array.isArray(value.tokens) && object(value.parsed);
}
export function parseJsonl(text) {
    return text.split("\n").filter(Boolean).map((line, index) => {
        const value = JSON.parse(line);
        if (!record(value))
            throw new Error(`Invalid trace record at JSONL line ${index + 1}`);
        return value;
    });
}
async function text(url) {
    for (let attempt = 0; attempt < 2; attempt += 1) {
        try {
            const response = await fetch(url);
            if (!response.ok)
                throw new Error(`Trace request failed: HTTP ${response.status}`);
            return response.text();
        }
        catch (error) {
            if (attempt === 1)
                throw error;
        }
    }
    throw new Error("Trace request failed");
}
export async function loadUrl(url) {
    return parseJsonl(await text(url));
}
export async function loadFile(file) {
    return parseJsonl(await file.text());
}
