const namespace = "http://www.w3.org/2000/svg";
export function element(id) {
    const found = document.getElementById(id);
    if (found === null) {
        throw new Error(`Missing UI element: ${id}`);
    }
    return found;
}
export function svg(name, attributes) {
    const node = document.createElementNS(namespace, name);
    Object.entries(attributes).forEach(([key, value]) => node.setAttribute(key, value));
    return node;
}
export function clear(node) {
    node.replaceChildren();
}
