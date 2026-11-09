// Synthetic read/calculation tools only: never execute generated code or shell.
import { randomBytes, randomInt } from "node:crypto";

export const tools = [
  { name: "lookup_probe", description: "Read the quantity and verification code for a test record.",
    parameters: { type: "object", properties: { id: { type: "string" } },
      required: ["id"], additionalProperties: false } },
  { name: "multiply", description: "Multiply two integer arguments and return product.",
    parameters: { type: "object", properties: { a: { type: "integer" }, b: { type: "integer" } },
      required: ["a", "b"], additionalProperties: false } },
];

function exactKeys(value, keys) {
  return value !== null && typeof value === "object" && !Array.isArray(value)
    && JSON.stringify(Object.keys(value).sort()) === JSON.stringify(keys.sort());
}

export function fixture() {
  const id = `probe-${randomBytes(4).toString("hex")}`;
  const record = { id, quantity: randomInt(11, 99), verification_code: randomBytes(12).toString("hex") };
  return { id, expected: `FINAL ${record.verification_code} ${record.quantity * 7}`,
    execute(call) {
      if (!exactKeys(call, ["name", "arguments"])) throw new Error("invalid_call_shape");
      const args = call.arguments;
      if (call.name === "lookup_probe") {
        if (!exactKeys(args, ["id"]) || args.id !== id) throw new Error("invalid_lookup_arguments");
        return { ...record };
      }
      if (call.name === "multiply") {
        if (!exactKeys(args, ["a", "b"]) || !Number.isSafeInteger(args.a)
          || !Number.isSafeInteger(args.b) || Math.abs(args.a) > 1000000
          || Math.abs(args.b) > 1000000) throw new Error("invalid_multiply_arguments");
        return { product: args.a * args.b };
      }
      throw new Error("tool_not_allowlisted");
    },
  };
}