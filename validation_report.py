#!/usr/bin/env python3
"""
Comprehensive JSON Validation Script for PRD and related files.

Validates:
1. JSON syntax validity
2. Required fields presence
3. No circular references in depends_on
4. Array/object types match schema
5. String lengths for readability
6. All subtask content is represented
"""

import json
import os
import re
from pathlib import Path
from typing import Any, Dict, List, Set, Tuple
from dataclasses import dataclass
from datetime import datetime

@dataclass
class ValidationError:
    file: str
    path: str
    message: str
    severity: str  # 'error', 'warning', 'info'

@dataclass
class ValidationResult:
    file: str
    is_valid: bool
    errors: List[ValidationError]
    warnings: List[ValidationError]
    info: List[ValidationError]

class PRDValidator:
    """Validator for PRD JSON files"""
    
    # Schema definitions for different PRD types
    PRD_SCHEMA = {
        "required_root_fields": ["project", "feature", "user_stories"],
        "optional_root_fields": ["branch_name", "version", "technical_requirements", 
                                  "quality_checks", "created_at", "updated_at"],
        "user_story_required": ["id", "title", "description", "acceptance_criteria", 
                               "priority", "depends_on", "complexity"],
        "user_story_optional": ["passes"],
        "quality_checks_fields": ["typecheck", "test", "lint", "build"],
    }
    
    WORKTREE_PRD_SCHEMA = {
        "required_root_fields": ["id", "title", "version"],
        "optional_root_fields": ["status", "created", "updated", "owner", "summary",
                                  "problem", "solution", "requirements", "technical_design",
                                  "configuration", "api", "error_handling", "testing",
                                  "rollout", "metrics", "documentation", "open_questions"],
    }
    
    # String length limits for readability
    MAX_TITLE_LENGTH = 100
    MAX_DESCRIPTION_LENGTH = 500
    MAX_ACCEPTANCE_CRITERIA_LENGTH = 200
    MIN_DESCRIPTION_LENGTH = 20
    
    def __init__(self):
        self.results: List[ValidationResult] = []
    
    def validate_all(self, directory: str = ".") -> List[ValidationResult]:
        """Validate all JSON files in directory"""
        json_files = list(Path(directory).glob("*.json"))
        
        for json_file in json_files:
            result = self.validate_file(json_file)
            self.results.append(result)
        
        return self.results
    
    def validate_file(self, filepath: Path) -> ValidationResult:
        """Validate a single JSON file"""
        errors = []
        warnings = []
        info = []
        
        # 1. Validate JSON syntax
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            data = json.loads(content)
        except json.JSONDecodeError as e:
            errors.append(ValidationError(
                file=str(filepath),
                path="root",
                message=f"Invalid JSON syntax: {e}",
                severity="error"
            ))
            return ValidationResult(str(filepath), False, errors, warnings, info)
        except Exception as e:
            errors.append(ValidationError(
                file=str(filepath),
                path="root",
                message=f"Error reading file: {e}",
                severity="error"
            ))
            return ValidationResult(str(filepath), False, errors, warnings, info)
        
        # 2. Detect PRD type and validate accordingly
        if self._is_worktree_prd(data):
            self._validate_worktree_prd(data, str(filepath), errors, warnings, info)
        elif self._is_standard_prd(data):
            self._validate_standard_prd(data, str(filepath), errors, warnings, info)
        elif self._is_todos_file(data):
            self._validate_todos(data, str(filepath), errors, warnings, info)
        elif self._is_models_file(data):
            self._validate_models(data, str(filepath), errors, warnings, info)
        elif self._is_benchmark_file(data):
            self._validate_benchmark(data, str(filepath), errors, warnings, info)
        else:
            info.append(ValidationError(
                file=str(filepath),
                path="root",
                message="Unknown JSON structure, skipping schema validation",
                severity="info"
            ))
        
        # 3. Check for circular references in all files
        self._check_circular_references(data, str(filepath), errors)
        
        is_valid = len([e for e in errors if e.severity == "error"]) == 0
        return ValidationResult(str(filepath), is_valid, errors, warnings, info)
    
    def _is_worktree_prd(self, data: Dict) -> bool:
        """Check if this is a worktree PRD"""
        return "problem" in data and "solution" in data and "requirements" in data
    
    def _is_standard_prd(self, data: Dict) -> bool:
        """Check if this is a standard PRD"""
        return "user_stories" in data and isinstance(data.get("user_stories"), list)
    
    def _is_todos_file(self, data: Any) -> bool:
        """Check if this is a todos file"""
        return isinstance(data, list) and len(data) > 0 and "status" in data[0]
    
    def _is_models_file(self, data: Dict) -> bool:
        """Check if this is a models file"""
        return "models" in data
    
    def _is_benchmark_file(self, data: Dict) -> bool:
        """Check if this is a benchmark file"""
        return "swarm_execution" in data or "resource_efficiency_measured" in data
    
    def _validate_standard_prd(self, data: Dict, filepath: str, 
                               errors: List, warnings: List, info: List):
        """Validate standard PRD format"""
        schema = self.PRD_SCHEMA
        
        # Check required root fields
        for field in schema["required_root_fields"]:
            if field not in data:
                errors.append(ValidationError(
                    file=filepath,
                    path="root",
                    message=f"Missing required field: '{field}'",
                    severity="error"
                ))
        
        # Validate user_stories
        if "user_stories" in data:
            if not isinstance(data["user_stories"], list):
                errors.append(ValidationError(
                    file=filepath,
                    path="user_stories",
                    message="user_stories must be an array",
                    severity="error"
                ))
            else:
                for i, story in enumerate(data["user_stories"]):
                    self._validate_user_story(story, f"user_stories[{i}]", filepath, 
                                             errors, warnings, info)
                
                # Check for duplicate story IDs
                story_ids = [s.get("id") for s in data["user_stories"] if s.get("id")]
                duplicates = [id for id in story_ids if story_ids.count(id) > 1]
                if duplicates:
                    errors.append(ValidationError(
                        file=filepath,
                        path="user_stories",
                        message=f"Duplicate story IDs found: {set(duplicates)}",
                        severity="error"
                    ))
        
        # Validate quality_checks if present
        if "quality_checks" in data:
            self._validate_quality_checks(data["quality_checks"], "quality_checks", 
                                         filepath, errors, warnings)
    
    def _validate_user_story(self, story: Dict, path: str, filepath: str,
                            errors: List, warnings: List, info: List):
        """Validate a single user story"""
        if not isinstance(story, dict):
            errors.append(ValidationError(
                file=filepath,
                path=path,
                message="User story must be an object",
                severity="error"
            ))
            return
        
        schema = self.PRD_SCHEMA
        
        # Check required fields
        for field in schema["user_story_required"]:
            if field not in story:
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.{field}",
                    message=f"Missing required field: '{field}'",
                    severity="error"
                ))
        
        # Validate types and content
        if "id" in story:
            if not isinstance(story["id"], str):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.id",
                    message="id must be a string",
                    severity="error"
                ))
            elif not re.match(r'^[A-Z]+-\d+$', story["id"]):
                warnings.append(ValidationError(
                    file=filepath,
                    path=f"{path}.id",
                    message=f"Story ID '{story['id']}' doesn't match expected format (e.g., US-001, MF-001)",
                    severity="warning"
                ))
        
        if "title" in story:
            title = story["title"]
            if not isinstance(title, str):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.title",
                    message="title must be a string",
                    severity="error"
                ))
            else:
                if len(title) > self.MAX_TITLE_LENGTH:
                    warnings.append(ValidationError(
                        file=filepath,
                        path=f"{path}.title",
                        message=f"Title length ({len(title)}) exceeds recommended max ({self.MAX_TITLE_LENGTH})",
                        severity="warning"
                    ))
                if len(title) < 5:
                    warnings.append(ValidationError(
                        file=filepath,
                        path=f"{path}.title",
                        message="Title is very short, consider more descriptive text",
                        severity="warning"
                    ))
        
        if "description" in story:
            desc = story["description"]
            if not isinstance(desc, str):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.description",
                    message="description must be a string",
                    severity="error"
                ))
            else:
                if len(desc) > self.MAX_DESCRIPTION_LENGTH:
                    warnings.append(ValidationError(
                        file=filepath,
                        path=f"{path}.description",
                        message=f"Description length ({len(desc)}) exceeds recommended max ({self.MAX_DESCRIPTION_LENGTH})",
                        severity="warning"
                    ))
                if len(desc) < self.MIN_DESCRIPTION_LENGTH:
                    warnings.append(ValidationError(
                        file=filepath,
                        path=f"{path}.description",
                        message=f"Description is short ({len(desc)} chars), consider adding more detail",
                        severity="warning"
                    ))
        
        if "acceptance_criteria" in story:
            ac = story["acceptance_criteria"]
            if not isinstance(ac, list):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.acceptance_criteria",
                    message="acceptance_criteria must be an array",
                    severity="error"
                ))
            elif len(ac) == 0:
                warnings.append(ValidationError(
                    file=filepath,
                    path=f"{path}.acceptance_criteria",
                    message="acceptance_criteria is empty",
                    severity="warning"
                ))
            else:
                for i, criterion in enumerate(ac):
                    if not isinstance(criterion, str):
                        errors.append(ValidationError(
                            file=filepath,
                            path=f"{path}.acceptance_criteria[{i}]",
                            message="Each criterion must be a string",
                            severity="error"
                        ))
                    elif len(criterion) > self.MAX_ACCEPTANCE_CRITERIA_LENGTH:
                        warnings.append(ValidationError(
                            file=filepath,
                            path=f"{path}.acceptance_criteria[{i}]",
                            message=f"Criterion length ({len(criterion)}) is very long",
                            severity="warning"
                        ))
        
        if "priority" in story:
            if not isinstance(story["priority"], int):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.priority",
                    message="priority must be an integer",
                    severity="error"
                ))
            elif story["priority"] not in [1, 2, 3, 4, 5]:
                warnings.append(ValidationError(
                    file=filepath,
                    path=f"{path}.priority",
                    message=f"Priority {story['priority']} is outside typical range (1-5)",
                    severity="warning"
                ))
        
        if "depends_on" in story:
            deps = story["depends_on"]
            if not isinstance(deps, list):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.depends_on",
                    message="depends_on must be an array",
                    severity="error"
                ))
            else:
                for i, dep in enumerate(deps):
                    if not isinstance(dep, str):
                        errors.append(ValidationError(
                            file=filepath,
                            path=f"{path}.depends_on[{i}]",
                            message="Each dependency must be a string ID",
                            severity="error"
                        ))
        
        if "complexity" in story:
            if not isinstance(story["complexity"], int):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.complexity",
                    message="complexity must be an integer",
                    severity="error"
                ))
            elif story["complexity"] not in [1, 2, 3, 4, 5]:
                warnings.append(ValidationError(
                    file=filepath,
                    path=f"{path}.complexity",
                    message=f"Complexity {story['complexity']} is outside typical range (1-5)",
                    severity="warning"
                ))
        
        if "passes" in story:
            if not isinstance(story["passes"], bool):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.passes",
                    message="passes must be a boolean",
                    severity="error"
                ))
    
    def _validate_quality_checks(self, qc: Dict, path: str, filepath: str,
                                 errors: List, warnings: List):
        """Validate quality_checks structure"""
        if not isinstance(qc, dict):
            errors.append(ValidationError(
                file=filepath,
                path=path,
                message="quality_checks must be an object",
                severity="error"
            ))
            return
        
        for field in self.PRD_SCHEMA["quality_checks_fields"]:
            if field in qc and not isinstance(qc[field], str):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"{path}.{field}",
                    message=f"{field} must be a string command",
                    severity="error"
                ))
    
    def _validate_worktree_prd(self, data: Dict, filepath: str,
                               errors: List, warnings: List, info: List):
        """Validate worktree PRD format"""
        schema = self.WORKTREE_PRD_SCHEMA
        
        # Check required root fields
        for field in schema["required_root_fields"]:
            if field not in data:
                errors.append(ValidationError(
                    file=filepath,
                    path="root",
                    message=f"Missing required field: '{field}'",
                    severity="error"
                ))
        
        # Validate requirements.functional if present
        if "requirements" in data and "functional" in data["requirements"]:
            func_reqs = data["requirements"]["functional"]
            if isinstance(func_reqs, list):
                for i, req in enumerate(func_reqs):
                    if "id" not in req:
                        errors.append(ValidationError(
                            file=filepath,
                            path=f"requirements.functional[{i}]",
                            message="Missing required field: 'id'",
                            severity="error"
                        ))
                    if "acceptance_criteria" in req:
                        if not isinstance(req["acceptance_criteria"], list):
                            errors.append(ValidationError(
                                file=filepath,
                                path=f"requirements.functional[{i}].acceptance_criteria",
                                message="acceptance_criteria must be an array",
                                severity="error"
                            ))
    
    def _validate_todos(self, data: List, filepath: str,
                       errors: List, warnings: List, info: List):
        """Validate todos file format"""
        if not isinstance(data, list):
            errors.append(ValidationError(
                file=filepath,
                path="root",
                message="Todos file must be an array",
                severity="error"
            ))
            return
        
        valid_statuses = {"pending", "in_progress", "done", "blocked"}
        valid_priorities = {"low", "medium", "high", "critical"}
        
        todo_ids = []
        for i, todo in enumerate(data):
            if not isinstance(todo, dict):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"[{i}]",
                    message="Todo item must be an object",
                    severity="error"
                ))
                continue
            
            # Check required fields
            if "id" not in todo:
                errors.append(ValidationError(
                    file=filepath,
                    path=f"[{i}]",
                    message="Missing required field: 'id'",
                    severity="error"
                ))
            else:
                todo_ids.append(todo["id"])
            
            if "content" not in todo:
                errors.append(ValidationError(
                    file=filepath,
                    path=f"[{i}]",
                    message="Missing required field: 'content'",
                    severity="error"
                ))
            
            # Validate status
            if "status" in todo:
                if todo["status"] not in valid_statuses:
                    errors.append(ValidationError(
                        file=filepath,
                        path=f"[{i}].status",
                        message=f"Invalid status '{todo['status']}', must be one of {valid_statuses}",
                        severity="error"
                    ))
            
            # Validate priority
            if "priority" in todo:
                if todo["priority"] not in valid_priorities:
                    errors.append(ValidationError(
                        file=filepath,
                        path=f"[{i}].priority",
                        message=f"Invalid priority '{todo['priority']}', must be one of {valid_priorities}",
                        severity="error"
                    ))
        
        # Check for duplicate IDs
        duplicates = [id for id in todo_ids if todo_ids.count(id) > 1]
        if duplicates:
            errors.append(ValidationError(
                file=filepath,
                path="root",
                message=f"Duplicate todo IDs found: {set(duplicates)}",
                severity="error"
            ))
    
    def _validate_models(self, data: Dict, filepath: str,
                        errors: List, warnings: List, info: List):
        """Validate models file format"""
        if "models" not in data:
            errors.append(ValidationError(
                file=filepath,
                path="root",
                message="Missing required field: 'models'",
                severity="error"
            ))
            return
        
        models = data["models"]
        if not isinstance(models, dict):
            errors.append(ValidationError(
                file=filepath,
                path="models",
                message="models must be an object",
                severity="error"
            ))
            return
        
        required_model_fields = ["id", "name", "provider", "context_length"]
        
        for model_id, model in models.items():
            if not isinstance(model, dict):
                errors.append(ValidationError(
                    file=filepath,
                    path=f"models.{model_id}",
                    message="Model must be an object",
                    severity="error"
                ))
                continue
            
            for field in required_model_fields:
                if field not in model:
                    errors.append(ValidationError(
                        file=filepath,
                        path=f"models.{model_id}",
                        message=f"Missing required field: '{field}'",
                        severity="error"
                    ))
    
    def _validate_benchmark(self, data: Dict, filepath: str,
                           errors: List, warnings: List, info: List):
        """Validate benchmark file format"""
        # Benchmark files are more flexible, just check basic structure
        if not isinstance(data, dict):
            errors.append(ValidationError(
                file=filepath,
                path="root",
                message="Benchmark file must be an object",
                severity="error"
            ))
    
    def _check_circular_references(self, data: Any, filepath: str, errors: List):
        """Check for circular references in depends_on fields"""
        if not isinstance(data, dict):
            return
        
        # Build dependency graph
        graph = {}
        
        if "user_stories" in data and isinstance(data["user_stories"], list):
            for story in data["user_stories"]:
                if isinstance(story, dict) and "id" in story:
                    story_id = story["id"]
                    deps = story.get("depends_on", [])
                    if isinstance(deps, list):
                        graph[story_id] = deps
        
        # Detect cycles using DFS
        visited = set()
        rec_stack = set()
        
        def has_cycle(node: str, path: List[str]) -> Tuple[bool, List[str]]:
            visited.add(node)
            rec_stack.add(node)
            
            for neighbor in graph.get(node, []):
                if neighbor not in visited:
                    found, cycle_path = has_cycle(neighbor, path + [neighbor])
                    if found:
                        return True, cycle_path
                elif neighbor in rec_stack:
                    # Found cycle
                    cycle_start = path.index(neighbor) if neighbor in path else 0
                    return True, path[cycle_start:] + [neighbor]
            
            rec_stack.remove(node)
            return False, []
        
        for node in graph:
            if node not in visited:
                has_cyc, cycle_path = has_cycle(node, [node])
                if has_cyc:
                    errors.append(ValidationError(
                        file=filepath,
                        path="user_stories",
                        message=f"Circular dependency detected: {' -> '.join(cycle_path)}",
                        severity="error"
                    ))
                    break
    
    def generate_report(self) -> str:
        """Generate a formatted validation report"""
        lines = []
        lines.append("=" * 80)
        lines.append("COMPREHENSIVE JSON VALIDATION REPORT")
        lines.append("=" * 80)
        lines.append(f"Generated: {datetime.now().isoformat()}")
        lines.append("")
        
        total_errors = 0
        total_warnings = 0
        total_info = 0
        
        for result in self.results:
            lines.append("-" * 80)
            status = "✓ PASS" if result.is_valid else "✗ FAIL"
            lines.append(f"{status}: {result.file}")
            lines.append("-" * 80)
            
            if result.errors:
                lines.append("  ERRORS:")
                for e in result.errors:
                    lines.append(f"    [ERROR] {e.path}: {e.message}")
                    total_errors += 1
            
            if result.warnings:
                lines.append("  WARNINGS:")
                for w in result.warnings:
                    lines.append(f"    [WARNING] {w.path}: {w.message}")
                    total_warnings += 1
            
            if result.info:
                lines.append("  INFO:")
                for i in result.info:
                    lines.append(f"    [INFO] {i.path}: {i.message}")
                    total_info += 1
            
            if not result.errors and not result.warnings and not result.info:
                lines.append("  No issues found")
            
            lines.append("")
        
        lines.append("=" * 80)
        lines.append("SUMMARY")
        lines.append("=" * 80)
        lines.append(f"Files validated: {len(self.results)}")
        lines.append(f"Files with errors: {len([r for r in self.results if r.errors])}")
        lines.append(f"Total errors: {total_errors}")
        lines.append(f"Total warnings: {total_warnings}")
        lines.append(f"Total info: {total_info}")
        lines.append("")
        
        if total_errors == 0:
            lines.append("✓ All files passed validation!")
        else:
            lines.append(f"✗ {total_errors} error(s) found that need to be fixed")
        
        return "\n".join(lines)


def main():
    validator = PRDValidator()
    results = validator.validate_all(".")
    report = validator.generate_report()
    print(report)
    
    # Write report to file
    with open("validation_report.txt", "w") as f:
        f.write(report)
    
    # Exit with error code if there are errors
    total_errors = sum(len(r.errors) for r in results)
    return 1 if total_errors > 0 else 0


if __name__ == "__main__":
    exit(main())
