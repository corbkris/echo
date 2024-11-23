CREATE TABLE basic_account_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_info_id UUID REFERENCES account_info(id) UNIQUE,
    recovery_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
