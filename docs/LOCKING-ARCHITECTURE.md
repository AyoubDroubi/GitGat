# GitGat Locking and Forge Architecture

## Goal

GitGat must provide one safe file-locking workflow across GitHub and Azure DevOps without inventing a second incompatible lock protocol.

The authoritative file lock remains the standard Git LFS locking service attached to the repository remote. GitGat adds client-side enforcement, visibility, audit, and optional organization governance.

## Core decisions

1. Git remains the source of truth for commits, trees, blobs, refs, and working-copy state.
2. Git LFS remains the source of truth for binary objects and file locks.
3. Lockable files are declared in committed `.gitattributes` using the standard `lockable` attribute.
4. GitGat enables `lfs.setlockablereadonly=true` so lockable files are read-only unless the current user owns the lock.
5. GitGat verifies remote lock ownership before staging or committing lockable files.
6. Strict repositories fail closed before push if the LFS lock verification endpoint cannot be verified.
7. GitGat enables the Git LFS `locksverify` push policy for strict repositories, so the standard Git LFS pre-push verifier rejects updates to files locked by another user.
8. Force unlock is not part of the normal desktop workflow.
9. GitHub and Azure DevOps are forge providers; they do not replace the Git/LFS core.
10. A future admin portal is a governance/control plane, not a second lock database.

## Lock lifecycle

```text
Available / read-only
        |
        | Lock
        v
Locked by me / writable
        |
        | Edit
        v
Modified
        |
        | Stage (GitGat verifies lock ownership)
        v
Staged
        |
        | Commit (GitGat verifies lock ownership again)
        v
Committed
        |
        | Push (strict LFS lock verification)
        v
Pushed
        |
        | Pull request + CI/policies
        v
Merged
        |
        | Unlock
        v
Available / read-only
```

If another user owns the lock:

```text
Locked by teammate
  -> local file remains read-only
  -> GitGat refuses Stage
  -> GitGat refuses Commit
  -> Git LFS push verification rejects conflicting updates
```

## Repository setup

For binary assets that should never be edited concurrently:

```bash
git lfs track --lockable "*.uasset"
git lfs track --lockable "*.umap"
git lfs track --lockable "*.psd"
git lfs track --lockable "*.blend"
git add .gitattributes
git commit -m "chore: protect binary assets with LFS locks"
```

GitGat exposes this workflow through the LFS Locks screen and configures the repository-local LFS hooks and read-only policy.

## Provider architecture

```text
GitGat UI
  |
  +-- GitClient
  |     +-- status/diff/stage/commit
  |     +-- fetch/pull/push
  |     +-- branches/history/recovery
  |
  +-- LfsClient behavior inside GitClient
  |     +-- track --lockable
  |     +-- lock/unlock/list/verify
  |     +-- read-only policy
  |     +-- strict push verification
  |
  +-- ForgeProvider
        +-- GitHub
        |     +-- gh authentication
        |     +-- pull requests/reviews
        |     +-- Actions/notifications/activity
        |
        +-- Azure DevOps
              +-- Microsoft Entra / Azure CLI authentication
              +-- Azure Repos pull requests/review votes/policies
              +-- Azure Pipelines
              +-- Work Items (optional integration)
```

## Azure DevOps direction

Initial Azure DevOps support should mirror the existing GitHub security model: GitGat does not persist provider tokens.

For the first production adapter:
- detect Azure Repos remotes under `dev.azure.com` and supported Azure DevOps Server URLs;
- delegate interactive authentication to Azure CLI / Microsoft Entra ID;
- use the Azure DevOps CLI extension for stable PR and pipeline operations where it is complete;
- use Azure DevOps REST only for gaps that the CLI cannot cover;
- do not introduce PAT storage into GitGat;
- prefer HTTPS for repositories using Azure Repos + Git LFS.

A later native OAuth adapter can replace the CLI dependency without changing the provider contract.

## Admin portal / control plane

The core lock workflow does **not** require a custom portal. Building one too early would create a second lock authority and a split-brain risk.

A portal becomes valuable when GitGat needs organization-wide governance across many repositories/providers. It should be called the GitGat Control Plane and should store policy and audit data, while continuing to treat the provider LFS server as the authoritative active-lock registry.

Recommended responsibilities:
- repository enrollment and provider mapping;
- protected/lockable pattern policy;
- stale-lock policy and optional lease/TTL warnings;
- organization roles for force-unlock approval;
- lock event audit history;
- dashboard of active locks across repositories;
- webhook ingestion from GitHub/Azure DevOps;
- required CI/PR status check for GitGat lock-policy compliance;
- policy exceptions with reason, approver, and expiry;
- health checks for Git LFS locking support.

The portal must never silently create a lock that does not also exist on the provider LFS server.

## Enforcement layers

### Layer 1 - Working copy
Standard Git LFS `lockable` behavior keeps protected files read-only when the current user does not own the lock.

### Layer 2 - GitGat operations
GitGat verifies remote ownership before Stage and Commit. A lockable file must be owned by the current user.

### Layer 3 - Push
Strict repositories require a working LFS lock verification endpoint and enable `lfs.<endpoint>.locksverify=true`. Git LFS then blocks pushes that update files locked by another user.

### Layer 4 - Provider policy (planned)
A required GitHub check or Azure Repos branch policy validates GitGat lock-policy evidence before merge. This protects the team when a user bypasses GitGat or local hooks.

### Layer 5 - Governance (optional)
The Control Plane records policy exceptions and force-unlock decisions.

## Failure behavior

- Offline: cached lock information may be shown, but lock-changing operations and strict protected-file Stage/Commit must not claim authoritative ownership.
- Lock API unavailable: strict mode fails closed for protected workflows.
- Authentication expired: show provider/LFS authentication failure; do not downgrade silently.
- File locked by teammate: show owner and timestamp and block protected operations.
- Force unlock: admin-only future workflow with explicit reason and durable audit.
- Repository moved: existing GitGat relocation flow remains valid.
- Provider unavailable: local Git work on non-protected text files remains available; authoritative lock operations do not.

## Testing requirements

A locking feature is complete only when tests cover:
- verified `ours/theirs` JSON parsing;
- plain lock-list compatibility;
- lockable attribute detection;
- Stage denied without an owned lock;
- Stage denied with a teammate lock;
- Commit denied after external/manual staging without an owned lock;
- Stage/Commit allowed with an owned lock;
- strict push configuration;
- GitHub live LFS lock/list/unlock;
- Azure Repos live LFS lock/list/unlock;
- two-clone team scenario proving the second clone sees the first user's lock;
- read-only behavior on Windows, macOS, and Linux;
- provider authentication failure behavior;
- force-unlock absence from the normal UI.

## Delivery order

1. Strict LFS desktop enforcement.
2. Cross-platform E2E for two-clone locking.
3. Forge provider abstraction.
4. Azure DevOps provider.
5. Provider-side required lock-policy checks.
6. Control Plane/admin portal only after the multi-provider contract is stable.
