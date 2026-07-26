"""Short single-turn examples that prevent premature end-of-sequence.

The v2 regression showed 50% empty responses on brief prompts because every
training example was a long agent transcript. These teach the model that a
short standalone instruction still requires an answer.
"""

CASES: list[tuple[str, str]] = [
    (
        'Fix this Rust so it compiles. Return only code: '
        'fn double(v: &[i32]) -> Vec<i32> { v.iter().map(|x| x * 2) }',
        'fn double(v: &[i32]) -> Vec<i32> '
        '{ v.iter().map(|x| x * 2).collect() }',
    ),
    (
        'Fix this Rust so it compiles. Return only code: '
        'fn first(v: &Vec<String>) -> String { v[0] }',
        'fn first(v: &Vec<String>) -> String { v[0].clone() }',
    ),
    (
        'Fix this TypeScript type error. Return only code: '
        'const n: number = "5";',
        'const n: number = 5;',
    ),
    (
        'Complete this Rust so it returns the length. Return only code: '
        'fn size(s: &str) -> usize { s. }',
        'fn size(s: &str) -> usize { s.len() }',
    ),
    (
        'What must you verify before applying a patch?',
        'Verify that the change builds, that focused tests pass, and that it '
        'preserves unrelated work already in the worktree.',
    ),
    (
        'Reply with exactly: ready',
        'ready',
    ),
]


def messages() -> list[list[dict[str, str]]]:
    """Return conversations for each short instruction case."""
    return [
        [{'role': 'user', 'content': p}, {'role': 'assistant', 'content': a}]
        for p, a in CASES
    ]
