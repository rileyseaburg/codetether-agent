"""Short probe prompts that exposed the v2 regression.

Half of all short instructions returned an empty response from the v2 model
while its validation loss looked healthy, so these mirror the shortest real
requests in the corpus.
"""

PROMPTS = (
    'fix the failing test',
    'read src/main.rs',
    'what does this function do?',
    'add error handling here',
    'run the tests',
    'list the files in src',
    'rename this variable to count',
    'why is the build broken?',
)
"""Eight short instructions.

Kept small so a probe adds seconds rather than minutes to each evaluation.
"""
