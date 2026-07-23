const BASE_PRICE = 6720;
const SIDEWALK_RATE = 9.5;
const TAX_RATE = 0.0625;

const fixedAddons = [
  { id: "sealant", label: "Protective sealant", price: 280 },
  { id: "haul", label: "Soil & debris haul-away", price: 390 },
];

function sidewalkArea(length, width) {
  return Math.max(0, length) * Math.max(0, width);
}

function calculateEstimate(area, sidewalkSelected, selectedAddons) {
  const sidewalkPrice = sidewalkSelected ? area * SIDEWALK_RATE : 0;
  const extrasPrice = selectedAddons.reduce((total, addon) => total + addon.price, 0);
  const subtotal = BASE_PRICE + sidewalkPrice + extrasPrice;
  return { sidewalkPrice, subtotal, tax: subtotal * TAX_RATE, total: subtotal * (1 + TAX_RATE) };
}

function dollars(value) {
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(value);
}

globalThis.sidewalkPricing = Object.freeze({ calculateEstimate, dollars, fixedAddons, sidewalkArea });