CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    external_subject text NOT NULL UNIQUE,
    display_name text NOT NULL,
    email text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE memberships (
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role text NOT NULL CHECK (role IN (
        'organization_owner','organization_admin','repository_admin',
        'team_lead','developer','auditor'
    )),
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (organization_id, user_id)
);

CREATE TABLE teams (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (organization_id, name)
);

CREATE TABLE team_memberships (
    team_id uuid NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (team_id, user_id)
);

CREATE TABLE repository_role_assignments (
    repository_id uuid NOT NULL REFERENCES repository_registrations(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role text NOT NULL CHECK (role IN ('repository_admin','team_lead','developer','auditor')),
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (repository_id, user_id)
);

CREATE INDEX memberships_user_idx ON memberships(user_id);
CREATE INDEX teams_org_idx ON teams(organization_id);
CREATE INDEX repository_role_user_idx ON repository_role_assignments(user_id);
