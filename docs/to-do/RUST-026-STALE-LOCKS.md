# RUST-026 - Stale Lock Detection, Reminders and Escalation

Status: TODO

## Goal
Reduce abandoned locks without dangerous automatic unlocking.

## Policy
- configurable warning threshold;
- configurable escalation threshold;
- no automatic force unlock by default;
- repository/team overrides.

## Flow
Lock age threshold reached -> verify provider lock -> mark stale candidate -> notify owner -> optional team lead escalation -> admin action if needed.

## Notification channels
Start with in-product notifications and provider-linked surfaces. External channels can be added later through integrations.

## UI
- stale locks queue;
- owner;
- age;
- repository/path;
- reminder state;
- escalation state;
- linked force-unlock request.

## Acceptance
Stale locks are surfaced proactively without silently changing provider state.
