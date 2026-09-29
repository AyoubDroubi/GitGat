CREATE TABLE provider_policy_observations (
    repository_id uuid PRIMARY KEY REFERENCES repository_registrations(id) ON DELETE CASCADE,
    provider text NOT NULL CHECK (provider IN ('github','azure_devops')),
    required_check_configured boolean NOT NULL DEFAULT false,
    commit_binding_verified boolean NOT NULL DEFAULT false,
    observed_source_sha text,
    observed_target_branch text,
    details jsonb NOT NULL DEFAULT '{}'::jsonb,
    observed_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE provider_health_snapshots (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_connection_id uuid NOT NULL REFERENCES provider_connections(id) ON DELETE CASCADE,
    health text NOT NULL CHECK (health IN ('healthy','degraded','unavailable','unknown')),
    latency_ms bigint,
    provider_request_id text,
    details jsonb NOT NULL DEFAULT '{}'::jsonb,
    observed_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX provider_health_connection_time_idx
    ON provider_health_snapshots(provider_connection_id, observed_at DESC);

CREATE INDEX webhook_deliveries_status_time_idx
    ON webhook_deliveries(status, received_at DESC);
