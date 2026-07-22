export function integer(value: number): string {
  return new Intl.NumberFormat().format(value);
}

export function bytes(value: number): string {
  if (value < 1024) return `${value} B`;
  const units = ["KB", "MB", "GB"];
  let amount = value / 1024;
  let unit = units[0];
  for (let index = 1; amount >= 1024 && index < units.length; index += 1) {
    amount /= 1024;
    unit = units[index];
  }
  return `${amount.toFixed(2)} ${unit}`;
}

export function seconds(value: number): string {
  return value < 1 ? `${(value * 1000).toFixed(2)} ms` : `${value.toFixed(2)} s`;
}
