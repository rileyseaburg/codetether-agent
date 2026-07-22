import { clear, element, svg } from "./dom.js";
const colors = ["#6fffe0", "#c8ff55", "#ff9d55", "#a994ff", "#55a7ff"];
export function donut(data) {
    const target = element("donut");
    const legend = element("legend");
    clear(target);
    clear(legend);
    target.append(svg("circle", { cx: "110", cy: "110", r: "74", class: "donut-bg" }));
    const circumference = 2 * Math.PI * 74;
    const entries = [...data.calls.entries()].sort((left, right) => right[1] - left[1]).slice(0, 5);
    let offset = 0;
    entries.forEach(([name, count], index) => {
        const color = colors[index] ?? colors[0] ?? "#6fffe0";
        const length = (count / data.records) * circumference;
        target.append(svg("circle", { cx: "110", cy: "110", r: "74", class: "donut-segment", stroke: color, "stroke-dasharray": `${length} ${circumference - length}`, "stroke-dashoffset": `${-offset}` }));
        offset += length;
        const item = document.createElement("li");
        const dot = document.createElement("i");
        dot.style.background = color;
        const label = document.createElement("span");
        label.textContent = name;
        const value = document.createElement("b");
        value.textContent = `${((count / data.records) * 100).toFixed(1)}%`;
        item.append(dot, label, value);
        legend.append(item);
    });
}
