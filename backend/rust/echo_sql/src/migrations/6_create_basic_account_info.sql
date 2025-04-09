CREATE TABLE basic_account_info (
    id UUID PRIMARY KEY REFERENCES account_info(id),
    recovery_key UUID UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
