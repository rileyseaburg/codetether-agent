"""Use JSON construction instead of fragile nested string escaping."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/playwright/codetether/pr-review-response.cases.ts")
CONTENT = '''import { expect, test } from "@playwright/test";
import { normalize, rejects } from "./pr-review-response-runner";

const result = (value: object): string => JSON.stringify({ result: JSON.stringify(value) });

test("normalizes async JSON and rejects a missing blocking count", async () => {
  expect(await normalize(result({ markdown: "ready", blocking_findings: 2 }))).toBe("ready:2");
  const nested = JSON.stringify({ markdown: "ready", blocking_findings: 4 });
  expect(await normalize(JSON.stringify({ result: `analysis before result\\n${nested}` }))).toBe("ready:4");
  const clean = JSON.stringify({ markdown: "ready", blocking_findings: 0, nonblocking_findings: 0 });
  expect(await normalize(JSON.stringify({ result: `${clean}\\nCommitted` }))).toBe("ready:0");
  const one = JSON.stringify({ markdown: "one", blocking_findings: 0 });
  const two = JSON.stringify({ markdown: "two", blocking_findings: 0 });
  expect(await rejects(JSON.stringify({ result: `${one} then ${two}` }))).toBe(true);
  expect(await rejects(JSON.stringify({ markdown: "unsafe" }))).toBe(true);
});
'''


def main() -> None:
    """Replace only the response contract case module."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()