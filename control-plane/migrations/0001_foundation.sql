CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE organizations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    slug text NOT NULL UNIQUE,
    name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE repository_registrations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    provider text NOT NULL CHECK (provider IN ('github','azure_devops')),
    provider_repository_id text NOT NULL,
    display_name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(provider, provider_repository_id)
);

CREATE TABLE lock_policies (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    version bigint NOT NULL DEFAULT 1,
    protected_patterns jsonb NOT NULL DEFAULT '[]'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE audit_events (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid REFERENCES organizations(id) ON DELETE SET NULL,
    actor_id text,
    action text NOT NULL,
    target_type text NOT NULL,
    target_id text,
    correlation_id text,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    occurred_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX audit_events_org_time_idx ON audit_events(organization_id, occurred_at DESC);
