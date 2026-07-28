"""Verify schema grading separates our tools from invented ones."""

import unittest

from model_training.probe_tools import TOOL_SCHEMA
from model_training.schema_grade import grade


def _call(tool: str, param: str) -> str:
    """Return one rendered tool call in Qwen3.5 syntax."""
    body = f'<parameter={param}>a.py</parameter>'
    return f'<tool_call><function={tool}>{body}</function></tool_call>'


KNOWN = _call('read', 'path')
INVENTED = _call('open_file', 'path')
BAD_PARAM = _call('read', 'filename')
TEMPLATE_ECHO = '<function=example_function_name>'


class SchemaGradeTest(unittest.TestCase):
    """An untuned base already emits valid syntax, so names must be graded."""

    def test_known_tool_is_accepted(self) -> None:
        verdict = grade(KNOWN, TOOL_SCHEMA)
        self.assertTrue(verdict['emitted'])
        self.assertTrue(verdict['known_tool'])
        self.assertFalse(verdict['invented_tool'])
        self.assertTrue(verdict['params_valid'])

    def test_invented_tool_is_rejected(self) -> None:
        verdict = grade(INVENTED, TOOL_SCHEMA)
        self.assertTrue(verdict['invented_tool'])
        self.assertFalse(verdict['known_tool'])

    def test_template_placeholder_is_not_a_call(self) -> None:
        """`example_function_name` appears in the instruction block."""
        self.assertFalse(grade(TEMPLATE_ECHO, TOOL_SCHEMA)['emitted'])

    def test_wrong_parameter_name_fails_params(self) -> None:
        verdict = grade(BAD_PARAM, TOOL_SCHEMA)
        self.assertTrue(verdict['known_tool'])
        self.assertFalse(verdict['params_valid'])

    def test_plain_prose_emits_nothing(self) -> None:
        self.assertFalse(grade('I will read the file.', TOOL_SCHEMA)['emitted'])


if __name__ == '__main__':
    unittest.main()
