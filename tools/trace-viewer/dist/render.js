import { donut } from "./donut.js";
import { flow } from "./flow.js";
import { metrics } from "./metrics.js";
import { summary } from "./summary.js";
import { syscallOptions, table } from "./table.js";
import { timeline } from "./timeline.js";
export function render(records) {
    const data = metrics(records);
    summary(data);
    timeline(records, data);
    donut(data);
    flow(records);
    syscallOptions(records);
    table(records);
}
