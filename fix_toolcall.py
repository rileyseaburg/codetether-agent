import os
import glob
import re

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # We want to replace:
    # ContentPart::ToolCall {
    #     id,
    #     name,
    #     arguments,
    # }
    # with
    # ContentPart::ToolCall {
    #     id,
    #     name,
    #     arguments,
    #     ..
    # }
    
    # regex to find ContentPart::ToolCall { ... arguments, \n } or similar
    # We can just look for `arguments,` inside ContentPart::ToolCall block.
    # A safer approach: replace `arguments,\n` with `arguments,\n ..\n` if it's followed by `}`.
    
    # Let's use a regex that matches `ContentPart::ToolCall {` up to `}`
    # and if it doesn't contain `..` or `thought_signature`, we add `..` before `}`.
    
    def replacer(match):
        block = match.group(0)
        if '..' in block or 'thought_signature' in block:
            return block
        
        # Insert `..` before the last `}`
        last_brace = block.rfind('}')
        if last_brace != -1:
            return block[:last_brace] + '.. }'
        return block

    new_content = re.sub(r'ContentPart::ToolCall\s*\{[^}]+\}', replacer, content)
    
    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Fixed {filepath}")

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            fix_file(os.path.join(root, file))
