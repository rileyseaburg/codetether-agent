import { clear, element } from "./dom.js";
import { bytes, integer, seconds } from "./format.js";
export function summary(data) {
    const target = element("metrics");
    clear(target);
    const values = [
        ["Captured records", integer(data.records)],
        ["Transferred bytes", bytes(data.bytes)],
        ["Observed span", seconds(data.span)],
        ["Socket descriptors", integer(data.sockets)],
    ];
    values.forEach(([label, value]) => {
        const card = document.createElement("article");
        card.className = "metric";
        const caption = document.createElement("span");
        caption.textContent = label;
        const strong = document.createElement("strong");
        strong.textContent = value;
        card.append(caption, strong);
        target.append(card);
    });
}
