import { clear, element, svg } from "./dom.js";
export function timeline(records, data) {
    const target = element("timeline");
    clear(target);
    const bins = Array.from({ length: 72 }, () => 0);
    const span = data.span || 1;
    records.forEach(({ parsed }) => {
        const time = Number.parseFloat(parsed.timestamp ?? "");
        if (Number.isFinite(time)) {
            const index = Math.min(71, Math.max(0, Math.floor(((time - data.first) / span) * 72)));
            bins[index] = (bins[index] ?? 0) + 1;
        }
    });
    const peak = Math.max(...bins, 1);
    for (let row = 0; row < 5; row += 1) {
        target.append(svg("line", { x1: "45", y1: `${25 + row * 42}`, x2: "880", y2: `${25 + row * 42}`, class: "grid-line" }));
    }
    const points = bins.map((count, index) => `${45 + index * (835 / 71)},${205 - (count / peak) * 170}`);
    const defs = svg("defs", {});
    const gradient = svg("linearGradient", { id: "area-fill", x1: "0", y1: "0", x2: "0", y2: "1" });
    gradient.append(svg("stop", { offset: "0", "stop-color": "#6fffe0", "stop-opacity": ".26" }), svg("stop", { offset: "1", "stop-color": "#6fffe0", "stop-opacity": "0" }));
    defs.append(gradient);
    target.append(defs, svg("polygon", { points: `45,205 ${points.join(" ")} 880,205`, class: "area" }), svg("polyline", { points: points.join(" "), class: "trace-line" }));
    points.filter((_, index) => index % 8 === 0).forEach((point) => {
        const [cx = "0", cy = "0"] = point.split(",");
        target.append(svg("circle", { cx, cy, r: "3", class: "trace-dot" }));
    });
    element("timeline-label").textContent = `${data.span.toFixed(2)} SEC / ${peak} PEAK`;
}
