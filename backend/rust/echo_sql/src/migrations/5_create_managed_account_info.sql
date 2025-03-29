
CREATE TABLE managed_account_info (
    id UUID PRIMARY KEY REFERENCES account_info(id),
    email varchar(255) UNIQUE NOT NULL,
    phone varchar(255) UNIQUE,
		verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_managed_account_info_email ON managed_account_info(email);
CREATE INDEX idx_managed_account_info_phone ON managed_account_info(phone);

