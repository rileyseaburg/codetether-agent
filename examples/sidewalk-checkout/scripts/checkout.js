"use strict";
const { calculateEstimate, dollars, fixedAddons, sidewalkArea } = globalThis.sidewalkPricing;
const byId = (id) => document.getElementById(id);
const controls = [byId("sidewalk"), byId("length"), byId("width"), ...fixedAddons.map(({ id }) => byId(id))];

function addonLine(label, detail, price, controlId) {
  return `<div class="line-item"><span>${label}<small>${detail}<button class="remove-addon" data-remove="${controlId}" type="button">Remove</button></small></span><strong>${dollars(price)}</strong></div>`;
}

function render() {
  const area = sidewalkArea(Number(byId("length").value), Number(byId("width").value));
  const sidewalkSelected = byId("sidewalk").checked;
  const selectedAddons = fixedAddons.filter(({ id }) => byId(id).checked);
  const estimate = calculateEstimate(area, sidewalkSelected, selectedAddons);
  byId("sidewalk-measurements").hidden = !sidewalkSelected;
  byId("area").value = `${area.toLocaleString()} sq ft`;
  const sidewalk = sidewalkSelected ? addonLine("New sidewalk", `${area.toLocaleString()} sq ft`, estimate.sidewalkPrice, "sidewalk") : "";
  const fixed = selectedAddons.map(({ id, label, price }) => addonLine(label, "Added option", price, id)).join("");
  byId("addon-lines").innerHTML = sidewalk + fixed;
  byId("subtotal").textContent = dollars(estimate.subtotal);
  byId("tax").textContent = dollars(estimate.tax);
  byId("total").textContent = dollars(estimate.total);
}

controls.forEach((control) => control.addEventListener("input", render));
byId("addon-lines").addEventListener("click", ({ target }) => {
  const remove = target.closest("[data-remove]");
  if (remove) { byId(remove.dataset.remove).checked = false; render(); }
});
byId("continue").addEventListener("click", () => {
  const toast = byId("toast");
  toast.classList.add("show");
  window.setTimeout(() => toast.classList.remove("show"), 2800);
});
render();