with open("src/tui/worker_bridge.rs", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "if let Err(e) = register_worker(" in line:
        # replace the lines from i to i+8
        pass

