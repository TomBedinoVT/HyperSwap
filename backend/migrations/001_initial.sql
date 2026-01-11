-- Utilisateurs
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255),
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Authentifications OAuth
CREATE TABLE oauth_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL, -- 'google', 'microsoft', 'github'
    provider_user_id VARCHAR(255) NOT NULL,
    access_token_encrypted TEXT, -- Optionnel, chiffré si stocké
    refresh_token_encrypted TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

-- Organisations (optionnel)
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Membres d'organisation
CREATE TABLE organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- 'owner', 'admin', 'member'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, user_id)
);

-- Secrets (partage classique)
CREATE TABLE secrets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_id UUID REFERENCES users(id) ON DELETE SET NULL,
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    token VARCHAR(64) UNIQUE NOT NULL, -- Token public pour le lien
    encrypted_data TEXT NOT NULL, -- Données chiffrées (JSON avec IV, ciphertext, etc.)
    encrypted_metadata TEXT, -- Métadonnées chiffrées (nom, description optionnel)
    max_views INTEGER DEFAULT NULL, -- NULL = illimité
    current_views INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ,
    burn_after_reading BOOLEAN NOT NULL DEFAULT false,
    is_file BOOLEAN NOT NULL DEFAULT false,
    file_path TEXT, -- Chemin S3 si fichier
    file_size BIGINT, -- Taille en bytes
    file_mime_type VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed_at TIMESTAMPTZ
);

CREATE INDEX idx_secrets_token ON secrets(token);
CREATE INDEX idx_secrets_expires_at ON secrets(expires_at);
CREATE INDEX idx_secrets_creator_id ON secrets(creator_id);

-- Secrets de collecte (reverse sharing)
CREATE TABLE secret_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requester_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    token VARCHAR(64) UNIQUE NOT NULL,
    encrypted_prompt TEXT, -- Question/prompt chiffré pour le client
    encrypted_data TEXT, -- Secret reçu (chiffré)
    max_views INTEGER DEFAULT 1,
    current_views INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'expired'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_secret_requests_token ON secret_requests(token);
CREATE INDEX idx_secret_requests_requester_id ON secret_requests(requester_id);
CREATE INDEX idx_secret_requests_status ON secret_requests(status);

-- Sessions JWT (pour invalidation)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE, -- Hash du JWT
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

-- Audit logs (optionnel mais recommandé)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL, -- 'secret_created', 'secret_viewed', 'secret_deleted', etc.
    resource_type VARCHAR(50), -- 'secret', 'secret_request', 'user', etc.
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);

-- Rate limiting (pour protection brute force)
CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL, -- IP, user_id, token, etc.
    action VARCHAR(100) NOT NULL, -- 'secret_view', 'secret_create', 'login', etc.
    count INTEGER NOT NULL DEFAULT 1,
    window_start TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_rate_limits_identifier_action ON rate_limits(identifier, action);
CREATE INDEX idx_rate_limits_expires_at ON rate_limits(expires_at);

