# RUST-016 - Forge Provider Abstraction

Status: IN PROGRESS

## Goal
Make forge features provider-neutral while keeping Git and Git LFS provider-independent.

## Contract
ForgeProvider must support:
- detect provider;
- authentication status;
- connect/disconnect;
- repository identity;
- list/create/edit PR;
- comment;
- review/approve;
- merge/complete;
- list CI runs;
- rerun;
- logs;
- provider-specific warnings/capabilities.

## Providers
- GitHub
- Azure DevOps

## Design rules
- no provider logic in generic GitClient;
- no provider token stored in GitGat desktop DB;
- unsupported capability is explicit, never silently ignored;
- provider label visible in UI;
- capability matrix documented.

## Tests
- GitHub URL parsing;
- Azure DevOps URL parsing;
- unsupported remote rejection;
- provider routing for all common operations;
- GitHub regression suite.

## Acceptance
The desktop UI calls a provider-neutral facade for forge operations.
