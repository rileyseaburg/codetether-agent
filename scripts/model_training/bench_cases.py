"""Fixed probe cases for the local model integration benchmark."""

TOOL_SCHEMA = [
    {
        'type': 'function',
        'function': {
            'name': 'read',
            'description': 'Read a file from the repository',
            'parameters': {
                'type': 'object',
                'properties': {'path': {'type': 'string'}},
                'required': ['path'],
            },
        },
    }
]

TOOL_PROMPTS = [
    'Read the file src/main.rs.',
    'Open README.md and summarize it.',
    'Show me the contents of Cargo.toml.',
    'I need to see what is inside src/lib.rs.',
    'Fetch the text of AGENTS.md.',
]

CODE_CASES = [
    (
        'Fix this Rust function so it compiles. Return only code: '
        'fn double(v: &[i32]) -> Vec<i32> { v.iter().map(|x| x * 2) }',
        'collect',
    ),
    (
        'Fix this Rust so it compiles. Return only code: '
        'fn first(v: &Vec<String>) -> String { v[0] }',
        'clone',
    ),
    (
        'Fix this TypeScript type error. Return only code: '
        'const n: number = "5";',
        '5',
    ),
    (
        'Complete this Rust so it returns the length. Return only code: '
        'fn size(s: &str) -> usize { s. }',
        'len',
    ),
]
