CREATE TABLE managed_account_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_info_id UUID REFERENCES account_info(id) UNIQUE,
    email varchar(255) UNIQUE NOT NULL,
    phone varchar(255) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
