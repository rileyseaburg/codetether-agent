import { closeDetail } from "./detail.js";
import { element } from "./dom.js";
import { loadFile, loadUrl } from "./loader.js";
import { filtered } from "./metrics.js";
import { render } from "./render.js";
import { table } from "./table.js";
let records = [];
function hideLoading() {
    element("loading").classList.add("hidden");
}
function applyFilter() {
    const query = element("search").value;
    const syscall = element("syscall").value;
    table(filtered(records, query, syscall));
}
async function start() {
    try {
        records = await loadUrl("/trace/all.jsonl");
        render(records);
    }
    catch (error) {
        element("source-name").textContent = error instanceof Error ? error.message : String(error);
    }
    finally {
        hideLoading();
    }
}
element("file-input").addEventListener("change", async (event) => {
    const input = event.currentTarget;
    if (!input.files?.[0])
        return;
    records = await loadFile(input.files[0]);
    element("source-name").textContent = input.files[0].name;
    render(records);
});
element("search").addEventListener("input", applyFilter);
element("syscall").addEventListener("change", applyFilter);
element("close-detail").addEventListener("click", closeDetail);
void start();
