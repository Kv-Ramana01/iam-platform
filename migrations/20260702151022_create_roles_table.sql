-- Add migration script here
CREATE TABLE roles (
    id UUID PRIMARY KEY,

    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    name TEXT NOT NULL,

    description TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(organization_id, name)
)