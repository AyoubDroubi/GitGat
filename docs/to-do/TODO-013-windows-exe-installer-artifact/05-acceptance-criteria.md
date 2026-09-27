# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | A clean Windows x64 machine can launch the published GitGat.exe without installing .NET separately. | Draft |
| AC-002 | REQ-002 | If single-file is enabled, core Git/native/UI behavior passes smoke tests; otherwise documented multi-file self-contained output is used. | Draft |
| AC-003 | REQ-003 | A GitGat-Setup-x64.exe installs the app, Start Menu entry and uninstaller successfully. | Draft |
| AC-004 | REQ-004 | Installed app has correct GitGat identity/icon/version and uninstalls cleanly. | Draft |
| AC-005 | REQ-005 | ARTIFACT-REGISTRY records result, version, commit, mechanism, path and checksum. | Draft |
| AC-006 | REQ-006 | Packaging can be repeated from the exact main commit without hand-editing source. | Draft |
| AC-007 | REQ-007 | Fresh-machine smoke test covers launch, open repository, Git detection and clean exit. | Draft |

## Regression criteria

- [ ] No repository/user data corruption.
- [ ] UI stays responsive during expected operations.
- [ ] Failures are actionable and do not leak secrets.

## Gate

Criteria remain Draft until scope/baseline approval.
