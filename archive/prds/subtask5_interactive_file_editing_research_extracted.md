# Subtask 5: Interactive File Editing Research - Extracted Content

## Overview
This document contains the extracted structured content from Subtask 5 research on interactive file editing in CLI/TUI tools, focusing on Claude Code, Aider, and Cursor.

---

## 1. File Editing Approaches

### 1.1 Claude Code

**Architecture:**
- Built on the Claude Agent SDK
- Runs in a lightweight Linux VM providing a secure sandbox
- Uses a tool-based approach with explicit file operations

**Editing Model:**
- Uses `Edit` tool for precise file modifications
- Tool-based architecture where each edit is a discrete operation
- Files must be explicitly added to the context before editing
- Supports reading files first, then applying targeted edits

**Key Characteristics:**
- Explicit file addition workflow (similar to git add)
- Line-number based or search/replace based edits
- Confirmation required for each edit operation
- Integration with git for version control

### 1.2 Aider

**Architecture:**
- Python-based CLI tool
- Tightly integrated with git
- Uses "edit formats" to communicate with LLMs

**Editing Model:**
Aider supports multiple edit formats optimized for different LLMs:

| Format | Description | Use Case |
|--------|-------------|----------|
| `whole` | Returns complete updated file | Simple files, smaller changes |
| `diff` | Search/replace blocks with `<<<<<<< SEARCH` / `=======` / `>>>>>>> REPLACE` | Most efficient, preferred format |
| `diff-fenced` | File path inside fence (for Gemini models) | Gemini family compatibility |
| `udiff` | Unified diff format | GPT-4 Turbo (reduces "lazy coding") |
| `editor-diff` / `editor-whole` | Streamlined for architect mode | Architect + Editor workflow |

**Key Implementation Details:**
```python
# Example diff format from Aider
mathweb/flask/app.py
<<<<<<< SEARCH
from flask import Flask
=======
import math
from flask import Flask
>>>>>>> REPLACE
```

**Smart Matching:**
- Perfect match first
- Whitespace-flexible matching
- Fuzzy matching with similarity threshold (0.8)
- "Did you mean?" suggestions for failed matches
- Handles elided code with `...` notation

### 1.3 Cursor

**Architecture:**
- Full IDE built on VS Code
- AI-native editor with integrated chat
- Uses inline editing and diff views

**Editing Model:**
- Inline code generation and modification
- Side-by-side diff view in editor
- Tab-based autocomplete
- Composer for multi-file changes
- Agent mode for autonomous task execution

---

## 2. Diff Display Formats

### 2.1 Terminal-Based Diff Display

**Aider's Approach:**
- Shows diffs in terminal using color coding
- Uses `/diff` command to show changes since last message
- Integrates with git diff for history viewing

**Color Coding Convention:**
- Red/deleted lines (removed content)
- Green/added lines (new content)
- Context lines (unchanged)
- Line numbers for reference

### 2.2 Side-by-Side Diff

**Cursor's Approach:**
- Native IDE diff viewer
- Split-pane view showing before/after
- Syntax highlighting
- Click-to-accept/reject individual hunks
- Inline diff within editor

### 2.3 Unified Diff Format

**Standard Pattern:**
```diff
--- filename.py
+++ filename.py
@@ -10,7 +10,8 @@
 def old_function():
-    pass
+    return True
```

**Aider's udiff Format:**
- Simplified unified diff
- Modified for LLM consumption
- Reduces "lazy coding" where models skip code sections

---

## 3. Edit Confirmation Flows

### 3.1 Explicit Confirmation Model

**Claude Code:**
- Each edit tool call requires user approval
- Preview of changes shown before application
- Yes/No/Edit prompt for each modification
- Can batch multiple edits in single confirmation

**Aider:**
- Auto-commits by default (configurable)
- Shows diff before commit
- `/undo` command for immediate rollback
- `--no-auto-commits` flag for manual control

### 3.2 Implicit/Automatic Model

**Cursor:**
- Real-time edits in editor
- Accept/reject via UI buttons
- Auto-apply for trusted operations
- Undo via standard editor undo (Ctrl+Z)

### 3.3 Confirmation UI Patterns

**Command-Line Patterns:**
```
# Aider pattern
Apply these changes? (Y/n/edit)

# Git-style confirmation  
Accept this hunk? [y,n,q,a,d,e,?]

# Multi-option pattern
[Yes/No/Show diff/Edit/Skip]
```

**Interactive Elements:**
- Progress indicators during application
- Success/failure feedback
- Error messages with context
- Suggestions for fixing failed edits

---

## 4. Undo/Redo Capabilities

### 4.1 Git-Based Undo

**Aider's Git Integration:**
```bash
# Automatic git commits for each change
- Commits with descriptive messages
- Uses Conventional Commits format
- Marks commits with "(aider)" attribution

# Undo commands
/undo          # Undo last change via git reset
/diff          # Show changes since last message
/commit        # Manual commit with AI message
/git <cmd>     # Raw git commands
```

**Key Features:**
- Pre-existing changes committed separately before edits
- Dirty file handling (commits existing changes first)
- Full git history preserved
- Can disable with `--no-auto-commits`

### 4.2 Editor-Based Undo

**Cursor:**
- Standard IDE undo/redo stack
- Persistent across sessions
- Multi-level undo for complex operations
- Branch-based undo for agent sessions

### 4.3 Checkpoint System

**Claude Code (Agent SDK):**
- File checkpointing capability
- Can rewind to specific checkpoints
- Session-based state management
- Integration with git for persistent checkpoints

**Implementation Pattern:**
```python
# Conceptual checkpoint system
checkpoint_id = create_checkpoint()
apply_edits(edits)
if user_rejects:
    restore_checkpoint(checkpoint_id)
```

---

## 5. UI Patterns and Interaction Models

### 5.1 Chat-Based Interface

**Common Pattern (Aider, Claude Code):**
```
> User request
[AI thinks...]
[Proposed changes shown as diff]
> Apply changes? (Y/n)
[Changes applied and committed]
```

**Key Elements:**
- Natural language requests
- Context-aware responses
- Inline code blocks for proposed changes
- Command interface for tool operations

### 5.2 Command Interface

**Aider Commands:**
```
/add <file>       # Add file to context
/drop <file>      # Remove file from context
/undo             # Undo last change
/diff             # Show diff
/commit           # Commit changes
/help             # Show help
```

**Claude Code Tools:**
- `Read` - Read file contents
- `Edit` - Modify file
- `Bash` - Execute commands
- `Write` - Create new files

### 5.3 Visual Feedback Patterns

**Progress Indicators:**
- Spinner during processing
- Step-by-step progress for multi-file operations
- Token usage display
- Cost tracking (Aider shows tokens/week)

**Error Handling:**
- Clear error messages with context
- Suggestions for resolution
- Partial success handling
- Failed edit reporting with "did you mean"

---

## 6. Best Practices and Design Principles

### 6.1 Safety and Control

1. **Git Integration**: All tools use git as the foundation for safety
2. **Explicit Confirmation**: User approval for destructive operations
3. **Atomic Operations**: All-or-nothing edit application
4. **Backup Creation**: Automatic preservation of original state

### 6.2 User Experience

1. **Minimal Context Switching**: Edit proposals shown inline
2. **Familiar Patterns**: Git-like commands and workflows
3. **Progressive Disclosure**: Detailed diffs available but not required
4. **Undo Confidence**: Clear indication that changes can be reverted

### 6.3 Error Resilience

1. **Fuzzy Matching**: Handle minor formatting differences
2. **Whitespace Flexibility**: Ignore insignificant whitespace changes
3. **Partial Success**: Apply what can be applied, report failures
4. **Recovery Suggestions**: Help users fix failed edits

---

## 7. Comparison Matrix

| Feature | Claude Code | Aider | Cursor |
|---------|-------------|-------|--------|
| **Interface** | CLI/Chat | CLI/Chat | IDE |
| **Edit Format** | Tool-based | Multiple formats | Native editor |
| **Diff Display** | Inline/Markdown | Terminal colors | Side-by-side |
| **Confirmation** | Per-tool | Auto-commit (opt) | UI buttons |
| **Undo Method** | Git + Checkpoints | Git integration | Editor undo + Git |
| **Multi-file** | Yes | Yes | Yes |
| **Offline Use** | No | Yes (local models) | Partial |
| **Git Required** | Recommended | Recommended | Optional |

---

## 8. Implementation Recommendations

### For CLI Tools:

1. **Use Git as Foundation**: Automatic commits provide safety net
2. **Support Multiple Edit Formats**: Different LLMs work better with different formats
3. **Implement Fuzzy Matching**: Real-world code rarely matches exactly
4. **Provide Clear Undo Path**: `/undo` command or similar
5. **Show Diffs Before Application**: Let users see what will change

### For TUI Tools:

1. **Side-by-Side Diff View**: Easier to understand changes
2. **Keyboard Navigation**: Vim-style bindings for efficiency
3. **Syntax Highlighting**: Maintain context with color coding
4. **Hunk-Level Control**: Accept/reject individual changes
5. **Persistent State**: Remember user preferences

### For All Tools:

1. **Preserve User's Working State**: Handle dirty files gracefully
2. **Clear Feedback Loops**: Success/failure clearly communicated
3. **Error Recovery**: Help users fix issues when edits fail
4. **Audit Trail**: Maintain history of AI changes
5. **User Control**: Always allow override of AI suggestions

---

## 9. Implementation Examples Reference

### Aider Diff Format Example
```python
# Example diff format from Aider
mathweb/flask/app.py
<<<<<<< SEARCH
from flask import Flask
=======
import math
from flask import Flask
>>>>>>> REPLACE
```

### Checkpoint System Pattern
```python
# Conceptual checkpoint system
checkpoint_id = create_checkpoint()
apply_edits(edits)
if user_rejects:
    restore_checkpoint(checkpoint_id)
```

### Aider Git Commands
```bash
/undo          # Undo last change via git reset
/diff          # Show changes since last message
/commit        # Manual commit with AI message
/git <cmd>     # Raw git commands
```

### Confirmation UI Patterns
```
# Aider pattern
Apply these changes? (Y/n/edit)

# Git-style confirmation  
Accept this hunk? [y,n,q,a,d,e,?]

# Multi-option pattern
[Yes/No/Show diff/Edit/Skip]
```

---

## 10. References

- Aider Documentation: https://aider.chat/docs/
- Aider GitHub: https://github.com/Aider-AI/aider
- Claude Code Documentation: https://docs.anthropic.com/en/docs/agents-and-tools/claude-code
- Cursor Documentation: https://cursor.com/docs
- Edit Formats Analysis: https://aider.chat/docs/more/edit-formats.html
- Git Integration: https://aider.chat/docs/git.html

---

*Extracted from: research_interactive_file_editing_cli_tools.md*
*Research compiled: 2025*
*Tools analyzed: Claude Code, Aider, Cursor*
