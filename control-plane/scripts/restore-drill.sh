#!/usr/bin/env bash
set -euo pipefail

: "${SOURCE_DATABASE_URL:?SOURCE_DATABASE_URL is required}"
: "${RESTORE_DATABASE_URL:?RESTORE_DATABASE_URL is required}"

backup_file="${TMPDIR:-/tmp}/gitgat-control-plane-$$.sql"
trap 'rm -f "$backup_file"' EXIT

pg_dump --no-owner --no-acl "$SOURCE_DATABASE_URL" >"$backup_file"
psql "$RESTORE_DATABASE_URL" -v ON_ERROR_STOP=1 -f "$backup_file" >/dev/null

source_orgs="$(psql "$SOURCE_DATABASE_URL" -Atqc 'SELECT COUNT(*) FROM organizations')"
restore_orgs="$(psql "$RESTORE_DATABASE_URL" -Atqc 'SELECT COUNT(*) FROM organizations')"
source_audit="$(psql "$SOURCE_DATABASE_URL" -Atqc 'SELECT COUNT(*) FROM audit_events')"
restore_audit="$(psql "$RESTORE_DATABASE_URL" -Atqc 'SELECT COUNT(*) FROM audit_events')"

test "$source_orgs" = "$restore_orgs"
test "$source_audit" = "$restore_audit"

echo "restore drill passed: organizations=$source_orgs audit_events=$source_audit"
echo "provider lock observations must be reconciled before enforcement is re-enabled"
