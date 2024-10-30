CREATE TABLE managed_account_info (
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES accounts(id) UNIQUE,
    email varchar(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);
