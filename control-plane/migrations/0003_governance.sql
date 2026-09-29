ALTER TABLE repository_registrations
    ADD COLUMN policy_mode text NOT NULL DEFAULT 'observe'
        CHECK (policy_mode IN ('observe','warn','enforce')),
    ADD COLUMN provider_health text NOT NULL DEFAULT 'unknown',
    ADD COLUMN last_verified_at timestamptz;

CREATE TABLE provider_connections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    provider text NOT NULL CHECK (provider IN ('github','azure_devops')),
    external_installation_id text NOT NULL,
    secret_reference text NOT NULL,
    health text NOT NULL DEFAULT 'unknown',
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(provider, external_installation_id)
);

CREATE TABLE webhook_deliveries (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_connection_id uuid NOT NULL REFERENCES provider_connections(id) ON DELETE CASCADE,
    delivery_id text NOT NULL,
    event_type text NOT NULL,
    payload_sha256 text NOT NULL,
    status text NOT NULL DEFAULT 'received',
    received_at timestamptz NOT NULL DEFAULT now(),
    processed_at timestamptz,
    UNIQUE(provider_connection_id, delivery_id)
);

ALTER TABLE lock_policies
    ADD COLUMN required_lock boolean NOT NULL DEFAULT true,
    ADD COLUMN excluded_patterns jsonb NOT NULL DEFAULT '[]'::jsonb,
    ADD COLUMN branch_scope jsonb NOT NULL DEFAULT '[]'::jsonb,
    ADD COLUMN max_lock_age_minutes bigint,
    ADD COLUMN force_unlock_approval_required boolean NOT NULL DEFAULT true;

CREATE TABLE lock_observations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    provider_lock_id text NOT NULL,
    path text NOT NULL,
    owner_external_id text NOT NULL,
    locked_at timestamptz,
    verified_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL,
    raw_evidence jsonb NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE(repository_id, provider_lock_id)
);

CREATE INDEX lock_observations_repo_verified_idx
    ON lock_observations(repository_id, verified_at DESC);

ALTER TABLE audit_events
    ADD COLUMN actor_type text NOT NULL DEFAULT 'user',
    ADD COLUMN repository_id uuid REFERENCES repository_registrations(id) ON DELETE SET NULL,
    ADD COLUMN path text,
    ADD COLUMN reason text,
    ADD COLUMN outcome text NOT NULL DEFAULT 'success',
    ADD COLUMN evidence_reference text,
    ADD COLUMN previous_hash text,
    ADD COLUMN event_hash text;
