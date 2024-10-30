CREATE TABLE basic_account_info (
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES accounts(id) UNIQUE,
    recovery_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);
