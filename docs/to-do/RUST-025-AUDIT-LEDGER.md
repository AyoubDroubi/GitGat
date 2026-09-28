# RUST-025 - Durable Audit Ledger and Compliance History

Status: TODO

## Events
Audit at minimum:
- login/security-sensitive admin events;
- repository enrollment/removal;
- provider connection changes;
- role/permission changes;
- policy create/update/delete;
- lock observation changes of operational significance;
- force-unlock requests/approvals/execution;
- exception lifecycle;
- provider status-check decisions;
- webhook failures/replays.

## Event fields
- immutable event ID;
- organization;
- actor;
- actor type;
- action;
- target type/id;
- repository/path when applicable;
- before/after summary;
- reason;
- timestamp;
- correlation/request ID;
- provider evidence reference;
- outcome.

## Requirements
- append-oriented;
- tamper-evident strategy documented;
- searchable;
- exportable;
- retention policy;
- privacy/secret redaction;
- organization isolation.

## Acceptance
An auditor can reconstruct who changed policy or forced an unlock and why.
