# RUST-023 - Organization-wide Active Lock Dashboard

Status: TODO

## Goal
Give teams one operational view across GitHub and Azure DevOps repositories.

## Dashboard
Show:
- provider;
- organization/project;
- repository;
- file path;
- lock owner;
- lock ID;
- locked at;
- age;
- stale status;
- related branch/PR when derivable;
- provider health;
- last verified time.

## Data model
Active lock truth is fetched from provider LFS.
Control Plane stores observations/snapshots for search, audit and stale detection.

## Reconciliation
- on repository page load;
- webhook/event-triggered where possible;
- scheduled reconciliation;
- manual refresh;
- stale observation expiry.

## Filters
- organization;
- team;
- repository;
- owner;
- age;
- stale;
- provider;
- path pattern.

## Acceptance
Displayed active locks are traceable to a recent provider verification and never presented as authoritative when stale.
