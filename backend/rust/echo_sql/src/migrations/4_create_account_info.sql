CREATE TYPE account_type AS ENUM ('basic', 'managed');

CREATE TABLE account_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) UNIQUE,
    account_type account_type NOT NULL,
    days_active int,
    verified boolean,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
