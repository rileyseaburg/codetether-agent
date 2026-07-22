const namespace = "http://www.w3.org/2000/svg";

export function element<T extends Element>(id: string): T {
  const found = document.getElementById(id);
  if (found === null) {
    throw new Error(`Missing UI element: ${id}`);
  }
  return found as unknown as T;
}

export function svg<K extends keyof SVGElementTagNameMap>(
  name: K,
  attributes: Record<string, string>,
): SVGElementTagNameMap[K] {
  const node = document.createElementNS(namespace, name);
  Object.entries(attributes).forEach(([key, value]) => node.setAttribute(key, value));
  return node;
}

export function clear(node: Element): void {
  node.replaceChildren();
}
