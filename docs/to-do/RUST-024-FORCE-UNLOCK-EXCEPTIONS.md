# RUST-024 - Force Unlock Approval and Policy Exceptions

Status: TODO

## Force unlock workflow
Request -> reason -> context capture -> approver review -> reverify live lock -> provider force unlock -> audit outcome.

## Required context
- repository;
- path;
- current owner;
- lock age;
- requester;
- reason;
- linked PR/incident if supplied;
- provider verification timestamp.

## Approval policy
Configurable:
- one admin;
- repository admin;
- two-person approval;
- no self-approval;
- emergency override role.

## Safety
Immediately before unlock:
- re-fetch lock;
- ensure ID/owner has not changed;
- cancel if changed;
- require explicit final confirmation;
- record provider response.

## Exceptions
Temporary policy exception must include:
- scope;
- subject/user/team;
- pattern/path;
- reason;
- approver;
- start;
- expiry;
- optional PR/incident;
- automatic expiration.

## Acceptance
Every forced unlock or bypass is explicit, time-bound where applicable and durably audited.
