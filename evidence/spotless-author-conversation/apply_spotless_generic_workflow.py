"""Move repository/deployment identities into Forgejo repository variables."""

from pathlib import Path

ROOT = Path('/home/riley/spotlessbinco')
WORKFLOW = ROOT / '.forgejo/workflows/codetether-pr-review.yml'
CONFIG_TEST = ROOT / 'tests/playwright/codetether/pr-review-config.cases.ts'
INFRA_TEST = ROOT / 'tests/playwright/infra/forgejo-workflow-actions.spec.ts'


def main() -> None:
    """Make the reusable review workflow depend only on explicit configuration."""
    text = WORKFLOW.read_text()
    replacements = {
        'runs-on: spotlessbinco-k8s': 'runs-on: ${{ vars.CODETETHER_RUNNER_LABEL }}',
        'FORGEJO_API_URL: https://forgejo.quantum-forge.io/api/v1': (
            'FORGEJO_API_URL: ${{ vars.CODETETHER_FORGEJO_API_URL }}'
        ),
        'CODETETHER_REVIEW_MODEL: openai-codex/gpt-5.6-sol-fast:high': (
            'CODETETHER_REVIEW_MODEL: ${{ vars.CODETETHER_REVIEW_MODEL }}'
        ),
        "CODETETHER_REVIEWER_CANDIDATES: 'codetether-bot=kv/forgejo/codetether-bot,spotless-agent=kv/forgejo/spotlessbinco-agent'": (
            'CODETETHER_REVIEWER_CANDIDATES: ${{ vars.CODETETHER_REVIEWER_CANDIDATES }}'
        ),
        'CODETETHER_REVIEW_TARGET_AGENT: pr-review-worker': (
            'CODETETHER_REVIEW_TARGET_AGENT: ${{ vars.CODETETHER_REVIEW_TARGET_AGENT }}'
        ),
    }
    for old, new in replacements.items():
        text = text.replace(old, new)
    WORKFLOW.write_text(text)
    text = WORKFLOW.read_text()
    api_entry = '      CODETETHER_API_URL: ${{ vars.CODETETHER_API_URL }}\n'
    token_entry = '      CODETETHER_TOKEN: ${{ secrets.CODETETHER_TOKEN }}\n'
    if api_entry not in text:
        anchor = '      FORGEJO_REPOSITORY: ${{ github.repository }}\n'
        text = text.replace(anchor, anchor + api_entry + token_entry, 1)
    WORKFLOW.write_text(text)
    test = CONFIG_TEST.read_text()
    test = test.replace(
        '"CODETETHER_REVIEW_TARGET_AGENT: pr-review-worker"',
        '"CODETETHER_REVIEW_TARGET_AGENT: ${{ vars.CODETETHER_REVIEW_TARGET_AGENT }}"',
    )
    test = test.replace(
        '"CODETETHER_REVIEW_MODEL: openai-codex/gpt-5.6-sol-fast:high"',
        '"CODETETHER_REVIEW_MODEL: ${{ vars.CODETETHER_REVIEW_MODEL }}"',
    )
    if 'CODETETHER_REVIEWER_CANDIDATES: ${{ vars.CODETETHER_REVIEWER_CANDIDATES }}' not in test:
        anchor = '"CODETETHER_REVIEW_AUTO_APPROVE: \'true\'",\n'
        test = test.replace(
            anchor,
            anchor + '    "CODETETHER_REVIEWER_CANDIDATES: ${{ vars.CODETETHER_REVIEWER_CANDIDATES }}",\n',
            1,
        )
    CONFIG_TEST.write_text(test)
    test = CONFIG_TEST.read_text()
    if 'CODETETHER_API_URL: ${{ vars.CODETETHER_API_URL }}' not in test:
        anchor = '  expect(workflow).toContain("FORGEJO_TOKEN: ${{ github.token }}");\n'
        addition = '''  expect(workflow).toContain(
    "CODETETHER_API_URL: ${{ vars.CODETETHER_API_URL }}",
  );
  expect(workflow).toContain(
    "CODETETHER_TOKEN: ${{ secrets.CODETETHER_TOKEN }}",
  );
'''
        test = test.replace(anchor, anchor + addition, 1)
    CONFIG_TEST.write_text(test)
    infra = INFRA_TEST.read_text().replace(
        'expect(reviewWorkflow).toContain("runs-on: spotlessbinco-k8s");',
        'expect(reviewWorkflow).toContain("runs-on: ${{ vars.CODETETHER_RUNNER_LABEL }}");',
    )
    INFRA_TEST.write_text(infra)


if __name__ == '__main__':
    main()