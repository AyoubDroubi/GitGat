# 03 — Decisions

### D-001 — Adopt .NET desktop foundation

**State:** Accepted  
**Date:** 2026-09-27  
**Related discussion:** DL-001  
**Affected requirements:** REQ-001..REQ-005  
**Baseline impact:** creates v1-approved

#### Decision

Use .NET 10, Avalonia 12, SukiUI 7, CommunityToolkit.Mvvm, Clean Architecture and System Git behind application interfaces.

#### Alternatives considered

- Continue TypeScript/Rust reference implementation.
- Windows-only WinUI 3.
- Flutter desktop.

#### Reason

Cross-platform desktop support, C#/.NET maintainability, modern custom UI capability and strong native/system integration.

#### Approval evidence

User explicitly selected Avalonia + SukiUI and authorized implementation on 2026-09-27.

#### Supersedes / superseded by

None.
