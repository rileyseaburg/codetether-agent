//! Non-executing Python mentions and unrelated tool commands stay permitted.

pub(super) const COMMANDS: &[&str] = &[
    "rg python3 src",
    "rg 'python3; /usr/bin/python' docs",
    "echo python3",
    "printf '%s\\n' 'python3 --version'",
    "ls /usr/bin/python3",
    "cat scripts/python3_example.txt",
    "python-config --includes",
    "python3-config --includes",
    "cargo test --lib shell_command_guard",
    "node scripts/build.js",
    "env FOO=python3 cargo test",
    "# python3 is not invoked",
    "echo '$(python3 script.py)'",
    "echo '`python3 script.py`'",
    "echo \"\\$(python3 script.py)\"",
];
