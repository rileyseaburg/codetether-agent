"""Mocked MSI table regressions for the live Error 1722 startup failure."""
import unittest
import xml.etree.ElementTree as ET
from .actions import add_actions
from .payload import NS
from .table_rules import validate

class ExecutionTests(unittest.TestCase):
    def test_authoring_schedules_deferred_caller_context(self) -> None:
        product = ET.Element('{' + NS + '}Product')
        add_actions(product)
        action = product.find("{%s}CustomAction[@Id='ConfigureNativeOcr']" % NS)
        self.assertEqual(action.get('Execute'), 'deferred')
        self.assertEqual(action.get('Impersonate'), 'yes')
        self.assertEqual(action.get('Return'), 'check')

    def test_previous_immediate_action_is_rejected(self) -> None:
        for action_type in [50, 2098, 3122]:
            tables = {'CustomAction': [
                {'Action': 'ResolveSetupHost', 'Type': '51'},
                {'Action': 'ConfigureNativeOcr', 'Type': str(action_type)}]}
            with self.assertRaisesRegex(AssertionError, 'must be deferred'):
                validate(tables)

    def test_deferred_without_system_flag_is_accepted(self) -> None:
        tables = {'CustomAction': [
            {'Action': 'ResolveSetupHost', 'Type': '51'},
            {'Action': 'ConfigureNativeOcr', 'Type': '1074', 'Source': 'SETUPHOST',
             'Target': '-File "[INSTALLROOT]script\\windows-install\\msi-entry.ps1"'}],
            'InstallExecuteSequence': [{'Action': key, 'Sequence': str(value)} for key, value in
                [('CostFinalize',1000),('ResolveSetupHost',1001),('InstallInitialize',1500),
                 ('RemoveExistingProducts',1501),('InstallFiles',4000),('ConfigureNativeOcr',6501),('InstallFinalize',6600)]],
            'summary': 'Template: x64;1033\nSource: 10 (a)', 'Property': [],
            'Upgrade': [{'ActionProperty':'PREVIOUSVERSIONS','Attributes':'768'},
                        {'ActionProperty':'NEWERVERSION','Attributes':'2'}]}
        validate(tables)

if __name__ == '__main__':
    unittest.main()