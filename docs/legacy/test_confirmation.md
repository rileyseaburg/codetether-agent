# File Edit Confirmation Test

This file demonstrates the new confirmation tools for US-003.

## Test 1: Single File Edit Confirmation

To test the `confirm_edit` tool:

1. First, create a test file:
```bash
echo "Hello World
This is a test file
Line 3" > test.txt
```

2. Use confirm_edit without confirmation:
```json
{
  "path": "test.txt",
  "old_string": "Hello World",
  "new_string": "Hello Universe"
}
```

3. The tool will return a diff preview with metadata indicating `requires_confirmation: true`

4. Use confirm_edit with confirmation:
```json
{
  "path": "test.txt",
  "old_string": "Hello World",
  "new_string": "Hello Universe",
  "confirm": true
}
```

## Test 2: Multi-File Edit Confirmation

To test the `confirm_multiedit` tool:

1. Create multiple test files:
```bash
echo "version = 1.0.0" > Cargo.toml
echo "fn main() { println!(\"old\"); }" > src/main.rs
```

2. Use confirm_multiedit without confirmation:
```json
{
  "edits": [
    {
      "file": "Cargo.toml",
      "old_string": "version = 1.0.0",
      "new_string": "version = 1.1.0"
    },
    {
      "file": "src/main.rs",
      "old_string": "println!(\"old\")",
      "new_string": "println!(\"new\")"
    }
  ]
}
```

3. The tool will return diffs for all files with metadata

4. Use confirm_multiedit with confirmation:
```json
{
  "edits": [...],
  "confirm": true
}
```

## Features Implemented

✅ **Diff display with color coding** - Red for removals, green for additions
✅ **Can accept, reject, or edit changes** - Via `confirm` parameter
✅ **Changes only applied after confirmation** - Original file preserved until confirmed
✅ **Original file is preserved until confirmation** - No changes applied without explicit confirmation