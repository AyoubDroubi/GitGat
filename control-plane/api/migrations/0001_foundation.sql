CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE organizations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    slug text NOT NULL,
    name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT organizations_slug_format CHECK (slug ~ '^[a-z0-9][a-z0-9-]{1,62}$')
);
CREATE UNIQUE INDEX organizations_slug_unique_ci ON organizations (lower(slug));

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    subject text NOT NULL UNIQUE,
    email text,
    display_name text NOT NULL,
    disabled_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX users_email_unique_ci
    ON users (lower(email))
    WHERE email IS NOT NULL;

CREATE TABLE organization_memberships (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (organization_id, user_id),
    CONSTRAINT organization_memberships_role_check CHECK (
        role IN ('owner', 'admin', 'repository_admin', 'team_lead', 'developer', 'auditor')
    )
);

CREATE TABLE provider_connections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    provider_kind text NOT NULL,
    provider_account_key text NOT NULL,
    display_name text NOT NULL,
    credential_reference text,
    status text NOT NULL DEFAULT 'pending',
    last_verified_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (organization_id, provider_kind, provider_account_key),
    CONSTRAINT provider_connections_kind_check CHECK (
        provider_kind IN ('github', 'azure_devops')
    ),
    CONSTRAINT provider_connections_status_check CHECK (
        status IN ('pending', 'healthy', 'degraded', 'disconnected', 'revoked')
    )
);

CREATE TABLE repository_registrations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    provider_connection_id uuid REFERENCES provider_connections(id) ON DELETE SET NULL,
    provider_kind text NOT NULL,
    provider_repository_key text NOT NULL,
    owner_key text NOT NULL,
    project_key text,
    repository_name text NOT NULL,
    default_branch text NOT NULL DEFAULT 'main',
    enforcement_mode text NOT NULL DEFAULT 'observe',
    enabled boolean NOT NULL DEFAULT true,
    enrolled_by_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    enrolled_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (organization_id, provider_kind, provider_repository_key),
    CONSTRAINT repository_registrations_kind_check CHECK (
        provider_kind IN ('github', 'azure_devops')
    ),
    CONSTRAINT repository_registrations_enforcement_check CHECK (
        enforcement_mode IN ('observe', 'warn', 'enforce')
    )
);

CREATE TABLE lock_policies (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    name text NOT NULL,
    enabled boolean NOT NULL DEFAULT true,
    version integer NOT NULL DEFAULT 1,
    stale_after_seconds bigint,
    force_unlock_approvals smallint NOT NULL DEFAULT 1,
    created_by_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT lock_policies_version_positive CHECK (version > 0),
    CONSTRAINT lock_policies_stale_positive CHECK (
        stale_after_seconds IS NULL OR stale_after_seconds > 0
    ),
    CONSTRAINT lock_policies_approval_range CHECK (
        force_unlock_approvals BETWEEN 1 AND 5
    )
);

CREATE TABLE lock_policy_patterns (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id uuid NOT NULL REFERENCES lock_policies(id) ON DELETE CASCADE,
    pattern text NOT NULL,
    required boolean NOT NULL DEFAULT true,
    sort_order integer NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (policy_id, pattern)
);

CREATE TABLE lock_observations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    provider_lock_id text NOT NULL,
    path text NOT NULL,
    owner_key text,
    owner_display text,
    locked_at timestamptz,
    observed_at timestamptz NOT NULL DEFAULT now(),
    verification_state text NOT NULL DEFAULT 'verified',
    UNIQUE (repository_id, provider_lock_id),
    CONSTRAINT lock_observations_state_check CHECK (
        verification_state IN ('verified', 'stale', 'unavailable')
    )
);
CREATE INDEX lock_observations_repository_path_idx
    ON lock_observations (repository_id, path);
CREATE INDEX lock_observations_observed_at_idx
    ON lock_observations (observed_at DESC);

CREATE TABLE policy_exceptions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    subject_key text,
    path_pattern text NOT NULL,
    reason text NOT NULL,
    requested_by_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    approved_by_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    starts_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL,
    revoked_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT policy_exceptions_expiry_check CHECK (expires_at > starts_at)
);
CREATE INDEX policy_exceptions_active_idx
    ON policy_exceptions (repository_id, expires_at)
    WHERE revoked_at IS NULL;

CREATE TABLE force_unlock_requests (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    path text NOT NULL,
    provider_lock_id text NOT NULL,
    observed_owner_key text,
    observed_owner_display text,
    requested_by_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    reason text NOT NULL,
    status text NOT NULL DEFAULT 'pending',
    requested_at timestamptz NOT NULL DEFAULT now(),
    decided_at timestamptz,
    executed_at timestamptz,
    provider_response jsonb,
    CONSTRAINT force_unlock_requests_status_check CHECK (
        status IN ('pending', 'approved', 'rejected', 'executed', 'cancelled', 'superseded')
    )
);
CREATE INDEX force_unlock_requests_pending_idx
    ON force_unlock_requests (repository_id, requested_at DESC)
    WHERE status = 'pending';

CREATE TABLE force_unlock_approvals (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id uuid NOT NULL REFERENCES force_unlock_requests(id) ON DELETE CASCADE,
    user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    decision text NOT NULL,
    reason text,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (request_id, user_id),
    CONSTRAINT force_unlock_approvals_decision_check CHECK (
        decision IN ('approve', 'reject')
    )
);

CREATE TABLE audit_events (
    id bigserial PRIMARY KEY,
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    actor_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    actor_key text,
    action text NOT NULL,
    target_type text NOT NULL,
    target_id text,
    repository_id uuid REFERENCES repository_registrations(id) ON DELETE SET NULL,
    path text,
    reason text,
    correlation_id uuid NOT NULL DEFAULT gen_random_uuid(),
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX audit_events_org_created_idx
    ON audit_events (organization_id, created_at DESC);
CREATE INDEX audit_events_repository_created_idx
    ON audit_events (repository_id, created_at DESC)
    WHERE repository_id IS NOT NULL;

CREATE TABLE webhook_deliveries (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_kind text NOT NULL,
    delivery_key text NOT NULL,
    organization_id uuid REFERENCES organizations(id) ON DELETE CASCADE,
    repository_id uuid REFERENCES repository_registrations(id) ON DELETE SET NULL,
    event_type text NOT NULL,
    status text NOT NULL DEFAULT 'received',
    attempts integer NOT NULL DEFAULT 0,
    payload_hash text NOT NULL,
    received_at timestamptz NOT NULL DEFAULT now(),
    processed_at timestamptz,
    last_error text,
    UNIQUE (provider_kind, delivery_key),
    CONSTRAINT webhook_deliveries_kind_check CHECK (
        provider_kind IN ('github', 'azure_devops')
    ),
    CONSTRAINT webhook_deliveries_status_check CHECK (
        status IN ('received', 'processing', 'processed', 'failed', 'dead_letter')
    )
);

CREATE TABLE provider_health_snapshots (
    id bigserial PRIMARY KEY,
    provider_connection_id uuid NOT NULL REFERENCES provider_connections(id) ON DELETE CASCADE,
    status text NOT NULL,
    details jsonb NOT NULL DEFAULT '{}'::jsonb,
    checked_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT provider_health_snapshots_status_check CHECK (
        status IN ('healthy', 'degraded', 'unavailable', 'unauthorized')
    )
);
CREATE INDEX provider_health_connection_checked_idx
    ON provider_health_snapshots (provider_connection_id, checked_at DESC);
