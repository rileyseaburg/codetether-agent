"""MSI execution rules, including deferred file installation semantics."""

def validate(tables: dict) -> None:
    actions = {row['Action']: row for row in tables['CustomAction']}
    assert int(actions['ResolveSetupHost']['Type']) == 51, 'host property must use caller context'
    action = actions['ConfigureNativeOcr']
    # 50 (property-sourced EXE) + 1024 (deferred); no NoImpersonate bit.
    assert int(action['Type']) == 1074, 'native setup must be deferred until InstallFiles executes'
    assert action['Source'] == 'SETUPHOST'
    assert '-File "[INSTALLROOT]script\\windows-install\\msi-entry.ps1"' in action['Target']
    sequence = {row['Action']: int(row['Sequence']) for row in tables['InstallExecuteSequence']}
    assert sequence['CostFinalize'] < sequence['ResolveSetupHost'] < sequence['ConfigureNativeOcr']
    assert sequence['InstallInitialize'] < sequence['RemoveExistingProducts'] < sequence['InstallFiles']
    assert sequence['InstallFiles'] < sequence['ConfigureNativeOcr'] < sequence['InstallFinalize']
    assert 'Template: x64;1033' in tables['summary'] and 'Source: 10 ' in tables['summary']
    properties = {row['Property']: row['Value'] for row in tables['Property']}
    assert not properties.get('ALLUSERS'), 'package must remain per-user'
    prior = next(row for row in tables['Upgrade'] if row['ActionProperty'] == 'PREVIOUSVERSIONS')
    assert int(prior['Attributes']) & 768 == 768, 'same-version upgrades must be detected'
    assert len(tables['Upgrade']) == 2, 'version ranges must not duplicate upgrade rows'