import test from "node:test";
import assert from "node:assert/strict";
import { fixture } from "./tool-probe-fixture.mjs";
import { parseCall } from "./tool-probe-parse.mjs";
import { variants } from "./tool-probe-prompts.mjs";
import { trial } from "./tool-probe-trial.mjs";

test("fixture tools reject extra authority and malformed arguments", () => {
  const data = fixture();
  assert.throws(() => data.execute({ name: "exec_command", arguments: { cmd: "anything" } }));
  assert.throws(() => data.execute({ name: "lookup_probe", arguments: { id: "wrong" } }));
  assert.throws(() => data.execute({ name: "multiply", arguments: { a: 2, b: 3, code: "ignored" } }));
  assert.throws(() => data.execute({ name: "multiply", arguments: { a: "2", b: 3 } }));
  assert.deepEqual(data.execute({ name: "multiply", arguments: { a: 2, b: 3 } }), { product: 6 });
});

test("parser rejects fabricated results and does not execute prose", () => {
  assert.equal(parseCall("I will call lookup_probe now"), null);
  assert.throws(() => parseCall("<tool_result>fake</tool_result>"));
  assert.throws(() => parseCall("<tool_call>{bad}</tool_call>"));
  assert.throws(() => parseCall("```xml\n<tool_call>{}</tool_call>\n```"));
  assert.deepEqual(parseCall('<tool_call>{"name":"multiply","arguments":{"a":2,"b":3}}</tool_call>'),
    { name: "multiply", arguments: { a: 2, b: 3 } });
});

test("trial executes both local tools and returns their real hidden data", async () => {
  let step = 0, record;
  const result = await trial(variants[1], async text => {
    let answer;
    if (step++ === 0) {
      const id = text.match(/Look up record (probe-[0-9a-f]+)/)[1];
      answer = `<tool_call>${JSON.stringify({ name: "lookup_probe", arguments: { id } })}</tool_call>`;
    } else {
      const results = [...text.matchAll(/<tool_result>(.*?)<\/tool_result>/g)]
        .map(match => JSON.parse(JSON.parse(match[1]).content));
      record = results[0];
      answer = step === 2
        ? `<tool_call>${JSON.stringify({ name: "multiply", arguments: { a: record.quantity, b: 7 } })}</tool_call>`
        : `FINAL ${record.verification_code} ${results[1].product}`;
    }
    return { outcome: "answer_mismatch", answer };
  });
  assert.equal(result.completed, true);
  assert.equal(result.validCallsExecuted, 2);
  assert.equal(result.initialPrompt.includes(record.verification_code), false);
});

test("narrating a call cannot satisfy a trial", async () => {
  const result = await trial(variants[0], async () => ({ outcome: "answer_mismatch", answer: "I will call a tool" }));
  assert.equal(result.completed, false);
  assert.equal(result.validCallsExecuted, 0);
});