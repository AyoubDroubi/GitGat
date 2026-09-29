CREATE TABLE force_unlock_requests (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    path text NOT NULL,
    provider_lock_id text NOT NULL,
    current_owner text NOT NULL,
    requester_user_id uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    reason text NOT NULL,
    linked_reference text,
    verified_at timestamptz NOT NULL,
    status text NOT NULL DEFAULT 'requested'
        CHECK (status IN ('requested','approved','rejected','executed','cancelled')),
    version bigint NOT NULL DEFAULT 1,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE force_unlock_approvals (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id uuid NOT NULL REFERENCES force_unlock_requests(id) ON DELETE CASCADE,
    approver_user_id uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    decision text NOT NULL CHECK (decision IN ('approve','reject')),
    reason text,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (request_id, approver_user_id)
);

CREATE TABLE policy_exceptions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    repository_id uuid REFERENCES repository_registrations(id) ON DELETE CASCADE,
    subject_external_id text,
    path_pattern text NOT NULL,
    reason text NOT NULL,
    approved_by_user_id uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    starts_at timestamptz NOT NULL,
    expires_at timestamptz NOT NULL,
    linked_reference text,
    created_at timestamptz NOT NULL DEFAULT now(),
    CHECK (expires_at > starts_at)
);

CREATE TABLE stale_lock_alerts (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    lock_observation_id uuid NOT NULL REFERENCES lock_observations(id) ON DELETE CASCADE,
    state text NOT NULL DEFAULT 'warning'
        CHECK (state IN ('warning','notified','escalated','resolved')),
    notified_at timestamptz,
    escalated_at timestamptz,
    resolved_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(lock_observation_id)
);

CREATE TABLE background_jobs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid REFERENCES organizations(id) ON DELETE CASCADE,
    kind text NOT NULL,
    dedupe_key text,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    status text NOT NULL DEFAULT 'queued'
        CHECK (status IN ('queued','running','succeeded','failed','dead_letter')),
    attempts integer NOT NULL DEFAULT 0,
    next_attempt_at timestamptz NOT NULL DEFAULT now(),
    last_error text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX background_jobs_dedupe_active_idx
    ON background_jobs(kind, dedupe_key)
    WHERE dedupe_key IS NOT NULL AND status IN ('queued','running');

CREATE INDEX force_unlock_repo_status_idx
    ON force_unlock_requests(repository_id, status, created_at DESC);
CREATE INDEX policy_exceptions_active_idx
    ON policy_exceptions(organization_id, expires_at);
CREATE INDEX background_jobs_ready_idx
    ON background_jobs(status, next_attempt_at);
