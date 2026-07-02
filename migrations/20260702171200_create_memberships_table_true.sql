-- Add migration script here
-- Add migration script here
CREATE TABLE memberships (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL
    REFERENCES users(id)
    ON DELETE CASCADE,

    organization_id UUID NOT NULL
    REFERENCES organizations(id)
    ON DELETE CASCADE,

    role_id UUID NOT NULL
    REFERENCES roles(id)
    ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL
    DEFAULT NOW(),

    UNIQUE (user_id, organization_id)
);