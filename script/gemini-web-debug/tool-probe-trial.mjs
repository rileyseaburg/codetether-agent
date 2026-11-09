// Bounded two-tool round trip; model output never becomes executable code.
import { fixture } from "./tool-probe-fixture.mjs";
import { prompt } from "./tool-probe-prompts.mjs";
import { parseCall } from "./tool-probe-parse.mjs";

export async function trial(variant, ask) {
  const data = fixture(), turns = [], executions = [];
  const initialPrompt = prompt(variant, data.id);
  let transcript = initialPrompt, finalMatches = false, failure = "turn_limit";
  for (let step = 0; step < 4; step++) {
    const reply = await ask(transcript);
    turns.push(reply);
    if (!["ok", "answer_mismatch"].includes(reply.outcome)) {
      failure = reply.outcome;
      break;
    }
    if (reply.answer.trim() === data.expected) {
      finalMatches = executions.some(item => item.call.name === "lookup_probe")
        && executions.some(item => item.call.name === "multiply");
      failure = finalMatches ? null : "final_without_required_tools";
      break;
    }
    try {
      const call = parseCall(reply.answer);
      if (!call) { failure = "no_tool_call_or_wrong_final"; break; }
      const result = data.execute(call);
      const toolCallId = `probe-call-${step + 1}`;
      executions.push({ toolCallId, call, result });
      transcript += `\nAssistant: ${reply.answer}\nTool: <tool_result>${JSON.stringify({ tool_call_id: toolCallId, name: call.name, content: JSON.stringify(result) })}</tool_result>\nAssistant:`;
    } catch (error) {
      failure = error.message;
      break;
    }
  }
  return { variant: variant.name, initialPrompt, turns, executions,
    expectedFinal: data.expected, expectedFinalWithheldFromInitialPrompt: !initialPrompt.includes(data.expected),
    validCallsExecuted: executions.length, finalMatches, failure,
    completed: finalMatches && failure === null };
}