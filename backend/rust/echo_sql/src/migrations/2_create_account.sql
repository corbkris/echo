CREATE TABLE
  accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username varchar(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
  );

CREATE INDEX idx_accounts_username ON accounts(username);
CREATE INDEX idx_accounts_created_at ON accounts(created_at);

