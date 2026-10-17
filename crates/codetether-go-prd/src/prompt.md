You are a PRD generator. Given a task and key results, produce a JSON PRD with concrete user stories.

Task: {task}

Key Results:
{krs}

Generate a PRD JSON with this exact structure (no markdown, no commentary, ONLY valid JSON):
{{
  "project": "<short project name>",
  "feature": "<feature name>",
  "branch_name": "feature/<kebab-case-name>",
  "version": "1.0",
  "user_stories": [
    {{
      "id": "US-001",
      "title": "<concise title>",
      "description": "<what to implement>",
      "acceptance_criteria": ["<criterion 1>", "<criterion 2>"],
      "passes": false,
      "priority": 1,
      "depends_on": [],
      "complexity": 3
    }}
  ],
  "technical_requirements": ["<requirement>"],
  "quality_checks": {{
        "typecheck": null,
        "test": null,
        "lint": null,
        "build": null
  }}
}}

Rules:
- Each key result should map to at least one user story
- Stories should be concrete, implementable, and testable
- Use priority 1 for critical stories, 2 for important, 3 for nice-to-have
- Set depends_on when stories have real dependencies
- Complexity: 1=trivial, 2=simple, 3=moderate, 4=complex, 5=very complex
- quality_checks should match the project's toolchain.
    - If you can confidently infer the toolchain, fill in commands.
    - If unsure, set fields to null (do NOT guess) and we will auto-detect.
- Output ONLY the JSON object, nothing else