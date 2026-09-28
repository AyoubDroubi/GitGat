# RUST-014 - Strict Git LFS Desktop Enforcement

Status: IN PROGRESS

## Goal
Turn basic Git LFS lock/unlock support into enforced team-safe editing.

## Dependencies
- RUST-007
- system Git LFS
- provider LFS lock service

## Scope
- parse verified locks into ours/theirs;
- add lockable patterns through GitGat;
- enable repository-local read-only behavior;
- require authoritative owned lock before Stage;
- require authoritative owned lock before Commit;
- enable strict lock verification before Push;
- show owner, timestamp and ownership state;
- expose explicit refresh;
- keep force unlock out of normal UI;
- fail closed when lock verification is unavailable for protected files.

## UX
- Protect as lockable
- Enable strict locking
- Lock
- Unlock
- Refresh locks
- Locked by you / Locked by teammate
- clear failure reasons

## Tests
- verified JSON ours/theirs;
- plain JSON compatibility;
- lockable attribute detection;
- Stage denied without owned lock;
- Stage denied when teammate owns lock;
- Commit denied after manual external staging;
- Stage/Commit allowed with owned lock;
- strict push configuration;
- no force-unlock control in normal UI.

## Acceptance
A lockable file cannot pass GitGat Stage or Commit unless the remote verified lock belongs to the current user.

## Evidence
- Windows/macOS/Linux quality pass;
- GitHub live LFS lock/list/unlock;
- regression of existing Git workflows.
